use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::token::{Token, Tokens};

enum LexemeState {
    None,
    GreaterOrLowerThan,
    EqualsEquals,
    NotEqualTo,
    CommentStart,
    SingleLineComment,
    MultiLineComment,
    MultiLineClose,
    Number,
    Alphabetic,
}

pub struct Lexical {
    state: LexemeState,
    pub variables: u32,
    pub constants: u32,
    pub assignments: u32,
    pub conditionals: u32,
    pub loops: u32,
    pub comments: u32,
    pub line: u32,
    pub main_line_start: u32,
    pub main_line_ends: u32,
    pub input: PathBuf,
    pub buffer: String,
    pub forward: usize,
    pub initial: u32,
}

impl Lexical {
    pub fn new() -> Lexical {
        let mut input = env::current_dir().unwrap();
        input.push("src/input.txt");
        Lexical {
            state: LexemeState::None,
            variables: 0,
            constants: 0,
            assignments: 0,
            conditionals: 0,
            loops: 0,
            comments: 0,
            line: 1,
            main_line_start: 0,
            main_line_ends: 0,
            input,
            buffer: String::new(),
            forward: 0,
            initial: 0,
        }
    }
    pub fn next_char(&mut self) -> char {
        let input = BufReader::new(File::open(self.input.clone()).expect("Failed to open file."));
        if self.initial == 0 {
            for line in input.lines() {
                self.buffer += format!("{}{}", line.expect("Failed to read line."), '\n').as_str();
            }
            self.initial = 1;
            self.buffer += "~";
        }
        let next_char = self.buffer.as_bytes()[self.forward];
        self.forward += 1;
        next_char as char
    }
    pub fn retract(&mut self) {
        self.forward -= 1
    }
    pub fn set_lexeme(&mut self) -> Token {
        self.state = LexemeState::None;
        let mut character: char;
        let mut lexeme = String::new();
        loop {
            character = self.next_char();
            match self.state {
                LexemeState::None => match character {
                    '\r' => (),
                    '\t' => (),
                    ' ' => (),
                    '\0' => self.retract(),
                    '\n' => self.line += 1,
                    ';' => return Token::new(Tokens::EndOfStatement, character.to_string()),
                    '(' => return Token::new(Tokens::OpenParenthesis, character.to_string()),
                    ')' => return Token::new(Tokens::ClosedParenthesis, character.to_string()),
                    '{' => return Token::new(Tokens::OpenBracket, character.to_string()),
                    '}' => {
                        self.main_line_ends = self.line;
                        return Token::new_single(Tokens::ClosedBracket);
                    }
                    '*' => return Token::new(Tokens::ArithmeticOperator, character.to_string()),
                    '+' => return Token::new(Tokens::ArithmeticOperator, character.to_string()),
                    '-' => return Token::new(Tokens::ArithmeticOperator, character.to_string()),
                    '/' => self.state = LexemeState::CommentStart,
                    '>' | '<' => {
                        self.state = LexemeState::GreaterOrLowerThan;
                        lexeme += character.to_string().as_str();
                    }
                    '=' => self.state = LexemeState::EqualsEquals,
                    '!' => self.state = LexemeState::NotEqualTo,
                    '~' => {
                        self.main_line_ends = self.main_line_start + self.main_line_ends;
                        return Token::new_single(Tokens::Eof);
                    }
                    _ => {
                        if character.is_alphabetic() || character == '_' {
                            self.state = LexemeState::Alphabetic;
                            lexeme += character.to_string().as_str()
                        } else if character.is_digit(10) {
                            self.state = LexemeState::Number;
                            lexeme += character.to_string().as_str()
                        } else {
                            return Token::new(Tokens::Error, String::from("Simbolo Indefinido"));
                        }
                    }
                },
                LexemeState::GreaterOrLowerThan => {
                    if character == '=' {
                        lexeme += character.to_string().as_str();
                    } else {
                        self.retract();
                        self.conditionals += 1;
                    }
                    return Token::new(Tokens::RelationalOperator, lexeme);
                }
                LexemeState::EqualsEquals => {
                    return if character == '=' {
                        self.conditionals += 1;
                        Token::new(Tokens::RelationalOperator, String::from("=="))
                    } else {
                        self.retract();
                        self.assignments += 1;
                        Token::new(Tokens::Assignment, String::from("="))
                    }
                }
                LexemeState::NotEqualTo => {
                    return if character == '=' {
                        self.conditionals += 1;
                        Token::new(Tokens::RelationalOperator, String::from("!="))
                    } else {
                        self.retract();
                        Token::new(
                            Tokens::Error,
                            String::from("Se esperaba '=' despues de '!'"),
                        )
                    }
                }
                LexemeState::Number => {
                    if !character.is_digit(10) {
                        self.retract();
                        return Token::new(Tokens::NaturalNumber, lexeme);
                    }
                    lexeme += character.to_string().as_str();
                }
                LexemeState::Alphabetic => {
                    if character.is_alphabetic() || character.is_digit(10) || character.eq(&'_') {
                        lexeme += character.to_string().as_str();
                    } else {
                        if KEYWORDS.contains(&lexeme.as_str()) {
                            if lexeme.to_lowercase().eq("for") || lexeme.to_lowercase().eq("while")
                            {
                                self.loops += 1;
                            } else if lexeme.to_lowercase().eq("const") {
                                self.constants += 1;
                            } else if lexeme.to_lowercase().eq("main") {
                                self.main_line_start += self.line - 1
                            }
                            self.retract();
                            return Token::new(Tokens::Keyword, lexeme);
                        }
                        return if DATA_TYPES.contains(&lexeme.as_str()) {
                            self.retract();
                            self.variables += 1;
                            Token::new(Tokens::DataType, lexeme)
                        } else {
                            self.retract();
                            Token::new(Tokens::Identifier, lexeme)
                        };
                    }
                }
                LexemeState::CommentStart => {
                    if character.eq(&'/') {
                        self.state = LexemeState::SingleLineComment;
                    } else if character.eq(&'*') {
                        self.state = LexemeState::MultiLineComment
                    } else {
                        self.retract();
                        return Token::new(Tokens::ArithmeticOperator, String::from("/"));
                    }
                }
                LexemeState::SingleLineComment => {
                    if character.eq(&'\n') {
                        self.state = LexemeState::None;
                        self.line += 1;
                        self.comments += 1;
                        return Token::new(Tokens::LineComment, lexeme);
                    } else {
                        lexeme += character.to_string().as_str()
                    }
                }
                LexemeState::MultiLineComment => {
                    if !character.eq(&'*') && character.is_alphabetic() || character.eq(&'_') {
                        lexeme += character.to_string().as_str()
                    }
                    if character.eq(&'\n') {
                        self.line += 1;
                    } else if character.eq(&'*') {
                        self.state = LexemeState::MultiLineClose;
                    }
                }
                LexemeState::MultiLineClose => {
                    if !character.eq(&'/') {
                        if character.is_alphabetic() || character.eq(&'_') {
                            lexeme += character.to_string().as_str()
                        } else if character.eq(&'\n') {
                            self.line += 1;
                        }
                        self.state = LexemeState::MultiLineComment;
                    } else {
                        self.state = LexemeState::None;
                        self.comments += 1;
                        return Token::new(Tokens::MultilineComment, lexeme);
                    }
                }
            }
        }
    }
}

const KEYWORDS: &'static [&str; 22] = &[
    "main",
    "case",
    "class",
    "const",
    "default",
    "delete",
    "else",
    "enum",
    "false",
    "true",
    "if",
    "for",
    "while",
    "do",
    "new",
    "private",
    "protected",
    "switch",
    "try",
    "catch",
    "return",
    "public",
];

const DATA_TYPES: &'static [&str; 7] =
    &["char", "int", "long", "double", "string", "short", "bool"];
