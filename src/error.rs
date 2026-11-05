use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidData(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(internal_error) => {
                write!(f, "Ошибка ввода-вывода: {}", internal_error)
            }
            Self::InvalidData(reason) => {
                write!(f, "Некорректные данные: {}", reason)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::InvalidData(format!(
            "Невозможно преобразовать значение в целое число: {}",
            value
        ))
    }
}
