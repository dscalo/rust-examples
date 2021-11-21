#[derive(Debug,PartialEq)]
pub enum Token {
    Numb(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    LParen,
    RParen
}