/// Формат файла.
#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    /// Comma-separated values.
    Csv,
    /// Binary,
    Bin,
    /// Plain text.
    Text,
}
