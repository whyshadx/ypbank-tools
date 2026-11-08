use std::io::{BufRead, BufReader, Read, Write};

use crate::{
    error::Error,
    model::{OperationType, Transaction},
};

/// Читает список транзакций в формате CSV.
/// Возвращает ошибку при сбое чтения или некорректных данных.
pub fn parse_lines(input: impl Read) -> Result<Vec<Transaction>, Error> {
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();
    let header = lines.next();
    let header = match header {
        Some(str) => str?,
        None => return Err(Error::InvalidData("Отсутствует заголовок".into())),
    };
    if header != "date,category,kind,amount" {
        return Err(Error::InvalidData("Неверный заголовок CSV".into()));
    }

    let mut transactions = Vec::new();
    for line in lines {
        let line = line?;
        let transaction = crate::text_format::parse_line(&line, ',')?;
        transactions.push(transaction);
    }

    Ok(transactions)
}

/// Записывает список транзакций в формате CSV.
/// Возвращает ошибку, если данные не удалось закодировать или записать.
pub fn write_transactions(mut out: impl Write, transactions: &[Transaction]) -> Result<(), Error> {
    writeln!(out, "date,category,kind,amount")?;
    for transaction in transactions {
        let op_type = match transaction.op_type {
            OperationType::Income => "income",
            OperationType::Expense => "expense",
        };
        writeln!(
            out,
            "{},{},{},{}",
            transaction.date, transaction.op_category, op_type, transaction.amount
        )?;
    }
    Ok(())
}
