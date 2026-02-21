use core::fmt;
use std::env::{self};

// #[derive(Debug)]
// enum AST {
//     Num(),
//     Char,
//      String([Char]),
//      Fn
// }
//

#[derive(Debug)]
enum CompilerError {
    InputFileError(std::io::Error),
    InvalidArguments,
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::InputFileError(e) => write!(f, "Cannot open file: {}", e),
            CompilerError::InvalidArguments => {
                write!(f, "Usage: ./compiler <file.bms> [-o output]")
            }
        }
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        CompilerError::InputFileError(e)
    }
}

fn char_lexer(c: &u8) -> u8 {
    *c
}

fn run() -> Result<(), CompilerError> {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).ok_or(CompilerError::InvalidArguments)?;

    let source: Vec<u8> = std::fs::read(path)?;
    let tokens = source.iter().map(char_lexer).collect::<Vec<_>>();

    tokens
        .iter()
        .for_each(|c| print!("{:?} ", char::from_u32(*c as u32)));
    print!("\n");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error {}", e);
        std::process::exit(1);
    }
}
