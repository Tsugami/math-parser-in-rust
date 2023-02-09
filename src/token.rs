#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /
    Number(usize),
}
