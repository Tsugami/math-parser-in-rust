#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Add,
    Sub,
    Multi,
    Div,
    Number(i64),
}
