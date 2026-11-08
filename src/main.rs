use std::fs::File;
use std::io::{self, Write};

use ypbank_tools::{convert, error::Error, format::Format};

fn main() {
    let mut args = std::env::args().skip(1);

    let (Some(input_type), Some(output_type)) = (args.next(), args.next()) else {
        eprintln!("Использование: converter <из> <в> [файл]");
        std::process::exit(1);
    };

    let file = args.next();
    if args.next().is_some() {
        eprintln!("Лишний аргумент. Использование: converter <из> <в> [файл]");
        std::process::exit(1);
    }

    let (Some(input_format), Some(output_format)) =
        (parse_format(&input_type), parse_format(&output_type))
    else {
        eprintln!("Допустимые форматы: csv, text, bin. Получено: {input_type} → {output_type}");
        std::process::exit(1);
    };

    if let Err(error) = convert_source(file, input_format, output_format) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn convert_source(
    file: Option<String>,
    input_format: Format,
    output_format: Format,
) -> Result<(), Error> {
    let mut output = io::stdout();
    match file {
        Some(path) => convert(File::open(path)?, input_format, &mut output, output_format)?,
        None => convert(io::stdin(), input_format, &mut output, output_format)?,
    }
    output.flush()?;
    Ok(())
}

fn parse_format(value: &str) -> Option<Format> {
    match value {
        "csv" => Some(Format::Csv),
        "text" => Some(Format::Text),
        "bin" => Some(Format::Bin),
        _ => None,
    }
}