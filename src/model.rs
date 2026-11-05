/// Модель транзакции.
#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    /// Дата транзакции в формате YYYY-MM-DD.
    pub date: String,
    /// Категория операции, например salary или rent.
    pub op_category: Category,
    /// Тип операции.
    pub op_type: OperationType,
    /// Сумма операции в копейках.
    pub amount: Amount,
}

/// Тип операции.
#[derive(Debug, PartialEq, Eq)]
pub enum OperationType {
    /// Доход.
    Income,
    /// Расход.
    Expense,
}

/// Сумма операции в копейках.
pub type Amount = u32;
/// Произвольное название категории операции.
pub type Category = String;
