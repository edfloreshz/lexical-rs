mod token;
mod lexical;

use token::{Token, Tokens};
use lexical::Lexical;

fn main() {
    let mut token: Token;
    let mut lexical = Lexical::new();
    loop {
        token = lexical.set_lexeme();
        match token.token_type {
            Tokens::OpenParenthesis => println!("Open Parenthesis: {}", token.value),
            Tokens::ClosedParenthesis => println!("Closed Parenthesis: {}", token.value),
            Tokens::ArithmeticOperator => println!("Arithmetic Operator: {}", token.value),
            Tokens::RelationalOperator => println!("Relational Operator: {}", token.value),
            Tokens::Assignment => println!("Assignment: {}", token.value),
            Tokens::Identifier => println!("Identifier: {}", token.value),
            Tokens::NaturalNumber => println!("Natural Number: {}", token.value),
            Tokens::Error => println!("Error: {}", token.value),
            Tokens::OpenBracket => println!("Open Bracket: {}", token.value),
            Tokens::ClosedBracket => println!("Closed Bracket: {}", token.value),
            Tokens::EndOfStatement => println!("End Of Statement: {}", token.value),
            Tokens::Keyword => println!("Keyword: {}", token.value),
            Tokens::LineComment => println!("Comment: {}", token.value),
            Tokens::MultilineComment => println!("Multiline Comment: {}", token.value),
            Tokens::Eof => {
                println!("End Of File: {} \n", token.value);
                println!("Variables: {}", lexical.variables);
                println!("Constants: {}", lexical.constants);
                println!("Assignments: {}", lexical.assignments);
                println!("Conditionals: {}", lexical.conditionals);
                println!("Loops: {}", lexical.loops);
                println!("Comments: {}", lexical.comments);
                println!("Main line count: {}", lexical.main_line_ends);
                break;
            },
            Tokens::DataType => println!("Data Type: {}", token.value)
        }
    }
}
