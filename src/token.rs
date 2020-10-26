pub struct Token {
    pub token_type: Tokens,
    pub value: String
}

impl Token {
    pub fn new_empty() -> Token {
        Token {
            token_type: Tokens::Empty,
            value: String::new()
        }
    }
    pub fn new_single(token_type: Tokens) -> Token {
        Token {
            token_type,
            value: String::new()
        }
    }
    pub fn new(token_type: Tokens, value: String) -> Token {
        Token {
            token_type,
            value
        }
    }
}

pub enum Tokens {
    Empty,
    OpenParenthesis,
    ClosedParenthesis,
    ArithmeticOperator,
    RelationalOperator,
    Assignment,
    Identifier,
    NaturalNumber,
    Error,
    OpenBracket,
    ClosedBracket,
    EndOfStatement,
    Keyword,
    LineComment,
    MultilineComment,
    Eof,
    DataType,
}

impl Tokens {
    pub fn value(&self) -> u32 {
        match *self {
            Tokens::Empty => 0,
            Tokens::OpenParenthesis => 1,
            Tokens::ClosedParenthesis => 2,
            Tokens::ArithmeticOperator => 3,
            Tokens::RelationalOperator => 4,
            Tokens::Assignment => 5,
            Tokens::Identifier => 6,
            Tokens::NaturalNumber => 7,
            Tokens::Error => 8,
            Tokens::OpenBracket => 9,
            Tokens::ClosedBracket => 10,
            Tokens::EndOfStatement => 11,
            Tokens::Keyword => 12,
            Tokens::LineComment => 13,
            Tokens::MultilineComment => 14,
            Tokens::Eof => 15,
            Tokens::DataType => 16
        }
    }
}