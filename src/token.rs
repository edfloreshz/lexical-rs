pub struct Token {
    pub token_type: Tokens,
    pub value: String
}

pub enum Tokens {
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

impl Token {
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
