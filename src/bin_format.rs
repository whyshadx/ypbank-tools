use std::io::{Read, Write};
use crate::{error::Error, model::Transaction};

fn write_string(out: &mut impl Write, value: &str) -> Result<(), Error> {
    let len = u32::try_from(value.len())?;
    let len = len.to_le_bytes();
    out.write_all(&len)?;
    out.write_all(value.as_bytes())?;
    
    Ok(())
}

fn write_transaction(out: &mut impl Write, value: &Transaction) -> Result<(), Error> {
    write_string(out, &value.date)?;
    write_string(out, &value.op_category)?;
    let byte: u8 = match value.op_type {
        crate::model::OperationType::Expense => 1,
        crate::model::OperationType::Income => 0
    };

    out.write_all(&[byte])?;

    out.write_all(&value.amount.to_le_bytes())?;
    
    Ok(())
}

pub fn write_transactions(out: &mut impl Write, value: &[Transaction]) -> Result<(), Error> {
    let count = u32::try_from(value.len())?;
    out.write_all(&count.to_le_bytes())?;

    for transaction in value {
        write_transaction(out, transaction)?;
    }

    Ok(())
}

fn read_u32(input: &mut impl Read) -> Result<u32, Error> {
    let mut bytes = [0u8; 4];

    input.read_exact(&mut bytes)?;

    let number = u32::from_le_bytes(bytes);

    Ok(number)
}

fn read_string(input: &mut impl Read) -> Result<String, Error> {
    let mut bytes = [0u8; 4];

    input.read_exact(&mut bytes)?;

    let len = u32::from_le_bytes(bytes);

    let mut buf = vec![0; usize::try_from(len)?];

    input.read_exact(&mut buf)?;

    let str = String::from_utf8(buf)?;

    Ok(str)
}

fn read_transaction(input: &mut impl Read) -> Result<Transaction, Error> {
    use crate::model::OperationType;

    let date = read_string(input)?;
    let op_category = read_string(input)?;

    let mut kind = [0u8; 1];
    input.read_exact(&mut kind)?;
    let op_type = match kind[0] {
        0 => OperationType::Income,
        1 => OperationType::Expense,
        byte => return Err(Error::InvalidData(format!("Неизвестный тип операции: {byte}"))),
    };
    let amount = read_u32(input)?;

    let transaction = Transaction {
        date,
        op_category,
        op_type,
        amount,
    };

    Ok(transaction)
}

pub fn read_transactions(mut input: impl Read) -> Result<Vec<Transaction>, Error> {
    let count = read_u32(&mut input)?;
    let mut transactions = Vec::new();
    for _ in 0..count {
        transactions.push(read_transaction(&mut input)?);
    }
    Ok(transactions)
}
