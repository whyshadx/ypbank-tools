use std::io::Cursor;

use crate::model::{OperationType, Transaction};
use crate::text_format;

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
