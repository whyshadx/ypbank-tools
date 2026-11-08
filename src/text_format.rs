use std::io::{BufRead, BufReader, Read, Write};

use crate::error::Error;
use crate::model::{Amount, OperationType, Transaction};

pub fn write_transactions(mut out: impl Write, transactions: &[Transaction]) -> Result<(), Error> {
    for transaction in transactions {
        let op_type = match transaction.op_type {
            OperationType::Expense => "expense",
            OperationType::Income => "income",
        };
        writeln!(
            out,
            "{};{};{};{}",
            transaction.date, transaction.op_category, op_type, transaction.amount
        )?;
    }
    Ok(())
}

pub fn parse_lines(input: impl Read) -> Result<Vec<Transaction>, Error> {
    let buffered = BufReader::new(input);

    let mut transactions: Vec<Transaction> = vec![];

    for line in buffered.lines() {
        let line = line?;
        let transaction = parse_line(&line, ';')?;
        transactions.push(transaction);
    }

    Ok(transactions)
}

pub(crate) fn parse_line(text: &str, separator: char) -> Result<Transaction, Error> {
    let split = text.split(separator);
    let result: Vec<&str> = split.collect();

    if result.len() > 4 {
        return Err(Error::InvalidData(
            "Запись не может содержать столбцов больше 4".into(),
        ));
    } else if result.len() < 4 {
        return Err(Error::InvalidData(
            "Запись не может содержать столбцов меньше 4".into(),
        ));
    }

    validate_date(result[0]).ok_or_else(|| Error::InvalidData("Неправильная дата".into()))?;
    let date = result[0].into();
    let op_category = result[1].into();
    let amount = result[3].parse::<Amount>()?;
    let op_type = match result[2] {
        "income" => OperationType::Income,
        "expense" => OperationType::Expense,
        _ => {
            return Err(Error::InvalidData(
                "Тип операции может быть или приход, или трата".into(),
            ));
        }
    };

    let transaction = Transaction {
        amount,
        date,
        op_type,
        op_category,
    };

    Ok(transaction)
}

pub(crate) fn validate_date(input: &str) -> Option<()> {
    let split = input.split('-');
    let parts: Vec<&str> = split.collect();

    if parts.len() != 3 {
        return None;
    }

    let month = parse_date_part(parts[1], 2, 1, 12)?;
    let day = parse_date_part(parts[2], 2, 1, 31)?;
    let year = parse_date_part(parts[0], 4, 1, 9999)?;

    let max_day = if month == 2 {
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        if leap { 29 } else { 28 }
    } else {
        // Формула для месяцев, кроме февраля: https://cmcenroe.me/2014/12/05/days-in-month-formula.html
        30 + (month + month / 8) % 2
    };
    if day > max_day {
        return None;
    }

    Some(())
}

fn parse_date_part(input: &str, char_count: i32, floor: i32, ceiling: i32) -> Option<i32> {
    if input.len() != char_count as usize || input.bytes().any(|x| !(b'0'..=b'9').contains(&x)) {
        return None;
    }

    let parse = input.parse::<i32>().ok()?;

    if parse < floor || parse > ceiling {
        return None;
    }

    Some(parse)
}
