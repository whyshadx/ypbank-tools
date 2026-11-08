pub mod error;
pub mod format;
pub mod model;

pub mod csv_format;
pub mod text_format;
pub mod bin_format;

use std::io::{Read, Write};

use error::Error;
use format::Format;
use model::Transaction;

pub fn read(input: impl Read, format: Format) -> Result<Vec<Transaction>, Error> {
    match format {
        Format::Csv => csv_format::parse_lines(input),
        Format::Text => text_format::parse_lines(input),
        Format::Bin => bin_format::read_transactions(input),
    }
}

pub fn write(
    mut output: impl Write,
    format: Format,
    transactions: &[Transaction],
) -> Result<(), Error> {
    match format {
        Format::Csv => csv_format::write_transactions(output, transactions),
        Format::Text => text_format::write_transactions(output, transactions),
        Format::Bin => bin_format::write_transactions(&mut output, transactions),
    }
}

pub fn convert(
    input: impl Read,
    input_format: Format,
    output: impl Write,
    output_format: Format,
) -> Result<(), Error> {
    let transactions = read(input, input_format)?;
    write(output, output_format, &transactions)
}

/// Первое различие между двумя списками транзакций.
#[derive(Debug, PartialEq, Eq)]
pub enum Difference {
    /// У записей на указанной позиции различается поле.
    Field {
        /// Позиция записи, начиная с единицы.
        position: usize,
        /// Имя поля в формате обмена.
        field: &'static str,
    },
    /// В одном из списков есть запись без пары.
    ExtraTransaction {
        /// Позиция первой записи без пары, начиная с единицы.
        position: usize,
    },
}

/// Сравнивает транзакции по порядку и возвращает первое различие.
/// Возвращает `None`, если списки совпадают.
pub fn compare(left: &[Transaction], right: &[Transaction]) -> Option<Difference> {
    for (index, (left, right)) in left.iter().zip(right).enumerate() {
        let field = if left.date != right.date {
            "date"
        } else if left.op_category != right.op_category {
            "category"
        } else if left.op_type != right.op_type {
            "kind"
        } else if left.amount != right.amount {
            "amount"
        } else {
            continue;
        };

        return Some(Difference::Field {
            position: index + 1,
            field,
        });
    }

    if left.len() != right.len() {
        return Some(Difference::ExtraTransaction {
            position: left.len().min(right.len()) + 1,
        });
    }

    None
}

#[cfg(test)]
mod tests;
