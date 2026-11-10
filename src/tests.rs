use std::io::{Cursor, ErrorKind};

use crate::model::{OperationType, Transaction};
use crate::{bin_format, convert, csv_format, error::Error, format::Format, text_format};

#[test]
fn check_read_transactions() {
    let input = Cursor::new("2026-04-01;salary;income;120000\n");
    let result = text_format::parse_lines(input).expect("ожидалась корректная транзакция");
    let expected = vec![Transaction {
        date: "2026-04-01".into(),
        op_category: "salary".into(),
        op_type: OperationType::Income,
        amount: 120000,
    }];

    assert_eq!(result, expected);
}

#[test]
fn reads_csv() {
    let input = Cursor::new("date,category,kind,amount\n2026-01-01,grocery,income,10000\n");
    let result = csv_format::parse_lines(input).expect("ожидался корректный CSV");
    let expected = vec![Transaction {
        date: "2026-01-01".into(),
        op_category: "grocery".into(),
        op_type: OperationType::Income,
        amount: 10000,
    }];

    assert_eq!(result, expected);
}

#[test]
fn writes_and_reads_binary() {
    let expected = vec![Transaction {
        date: "2026-01-01".into(),
        op_category: "еда".into(),
        op_type: OperationType::Expense,
        amount: 10000,
    }];

    let mut bytes = Vec::new();
    bin_format::write_transactions(&mut bytes, &expected).expect("ожидалась запись транзакции");

    let input = Cursor::new(bytes);
    let result = bin_format::read_transactions(input).expect("ожидалась бинарная транзакция");

    assert_eq!(result, expected);
}

#[test]
fn rejects_csv_with_wrong_header() {
    let input = Cursor::new("date,category,amount,kind\n");
    let result = csv_format::parse_lines(input);

    assert!(matches!(result, Err(Error::InvalidData(_))));
}

#[test]
fn rejects_text_with_unknown_operation_type() {
    let input = Cursor::new("2026-04-01;salary;unknown;120000\n");
    let result = text_format::parse_lines(input);

    assert!(matches!(result, Err(Error::InvalidData(_))));
}

#[test]
fn rejects_truncated_binary() {
    let input = Cursor::new(1u32.to_le_bytes());
    let result = bin_format::read_transactions(input);
    let Err(Error::IO(error)) = result else {
        panic!("ожидалась ошибка чтения");
    };

    assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn converts_csv_to_text() {
    let input = Cursor::new("date,category,kind,amount\n2026-04-01,salary,income,120000\n");
    let mut result = Vec::new();
    convert(input, Format::Csv, &mut result, Format::Text).expect("ожидалась конвертация в text");
    let expected = b"2026-04-01;salary;income;120000\n";

    assert_eq!(result, expected);
}
