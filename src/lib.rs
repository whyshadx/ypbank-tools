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

#[cfg(test)]
mod tests;
