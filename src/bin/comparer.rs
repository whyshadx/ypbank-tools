use std::fs::File;

use ypbank_tools::{Difference, compare, error::Error, format::Format, read};

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(left_type), Some(left_path), Some(right_type), Some(right_path), None) = (
        args.next(),
        args.next(),
        args.next(),
        args.next(),
        args.next(),
    ) else {
        eprintln!("Использование: comparer <формат> <файл> <формат> <файл>");
        std::process::exit(2);
    };

    let (Some(left_format), Some(right_format)) =
        (parse_format(&left_type), parse_format(&right_type))
    else {
        eprintln!("Допустимые форматы: csv, text, bin. Получено: {left_type}, {right_type}");
        std::process::exit(2);
    };

    match compare_files(left_format, &left_path, right_format, &right_path) {
        Ok(None) => println!("Транзакции совпадают"),
        Ok(Some(Difference::Field { position, field })) => {
            println!("Транзакции отличаются на позиции {position}: поле {field}");
            std::process::exit(1);
        }
        Ok(Some(Difference::ExtraTransaction { position })) => {
            println!("Лишняя транзакция на позиции {position}");
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}

fn compare_files(
    left_format: Format,
    left_path: &str,
    right_format: Format,
    right_path: &str,
) -> Result<Option<Difference>, Error> {
    let left = read(File::open(left_path)?, left_format)?;
    let right = read(File::open(right_path)?, right_format)?;
    Ok(compare(&left, &right))
}

fn parse_format(value: &str) -> Option<Format> {
    match value {
        "csv" => Some(Format::Csv),
        "text" => Some(Format::Text),
        "bin" => Some(Format::Bin),
        _ => None,
    }
}
