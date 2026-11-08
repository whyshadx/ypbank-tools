/// Ошибка чтения, записи или преобразования данных транзакции.
#[derive(Debug)]
pub enum Error {
    /// Сбой ввода-вывода.
    IO(std::io::Error),
    /// Некорректные данные с описанием причины.
    InvalidData(String),
}

impl std::fmt::Display for Error {
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

impl From<std::num::TryFromIntError> for Error {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self::InvalidData(format!("Невозможно преобразовать в целое число: {}", value))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::InvalidData(format!("Некорректный UTF-8: {value}"))
    }
}
