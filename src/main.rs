use core::fmt;
use std::env::{self};

use crate::lexer::Token;
mod lexer;
mod parser;

#[derive(Debug)]
enum CompilerError {
    InputFileError(std::io::Error),
    InvalidArguments,
    UnexpectedCharacter,
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::InputFileError(e) => write!(f, "Cannot open file: {}", e),
            CompilerError::InvalidArguments => {
                write!(f, "Usage: ./compiler <file.bms> [-o output]")
            }
            CompilerError::UnexpectedCharacter => {
                write!(f, "UnexpectedCharacter")
            }
        }
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        CompilerError::InputFileError(e)
    }
}

fn run() -> Result<(), CompilerError> {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).ok_or(CompilerError::InvalidArguments)?;

    let source: String = std::fs::read_to_string(path)?;

    let mut pos = 0;

    while pos < source.len() {
        let remaining = &source[pos..];

        match Token::get_token(remaining) {
            Some((token, matched_str)) => {
                println!("{:?}", token);
                pos += matched_str.len();
            }
            None => {
                if remaining.starts_with(char::is_whitespace) {
                    pos += 1;
                } else {
                    return Err(CompilerError::UnexpectedCharacter);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error {}", e);
        std::process::exit(1);
    }
}
