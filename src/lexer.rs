use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Int,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntegerLiteral(i32),
    StringLiteral(String),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Assign,
    EQ,
    GT,
    LT,
    GEQ,
    LEQ,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Literal(Literal),
    Identifier(String),
    Operator(Operator),
    Punctuation(Punctuation),
}

type TokenRule<'a> = (&'a str, fn(&str) -> Token);

impl Token {
    pub fn get_token(input: &str) -> Option<(Token, String)> {
        let patterns: &[TokenRule] = &[
            (r"^[0-9]+", |s| {
                Token::Literal(Literal::IntegerLiteral(s.parse::<i32>().unwrap()))
            }),
            (r#"^"[^"]*""#, |s| {
                let inner = s[1..s.len() - 1].to_string();
                Token::Literal(Literal::StringLiteral(inner))
            }),
            (r"^[a-zA-Z_][a-zA-Z0-9_]*", |s| match s {
                "if" => Token::Keyword(Keyword::If),
                "else" => Token::Keyword(Keyword::Else),
                "int" => Token::Keyword(Keyword::Int),
                _ => Token::Identifier(s.to_string()),
            }),
            (r"^\+", |_| Token::Operator(Operator::Plus)),
            (r"^==", |_| Token::Operator(Operator::EQ)),
            (r"^=", |_| Token::Operator(Operator::Assign)),
            (r"^<=", |_| Token::Operator(Operator::LEQ)),
            (r"^>=", |_| Token::Operator(Operator::GEQ)),
            (r"^>", |_| Token::Operator(Operator::GT)),
            (r"^<", |_| Token::Operator(Operator::LT)),
            (r"^;", |_| Token::Punctuation(Punctuation::Semicolon)),
            (r"^\(", |_| Token::Punctuation(Punctuation::LParen)),
            (r"^\)", |_| Token::Punctuation(Punctuation::RParen)),
            (r"^\{", |_| Token::Punctuation(Punctuation::LBrace)),
            (r"^\}", |_| Token::Punctuation(Punctuation::RBrace)),
        ];

        patterns.iter().find_map(|(pattern, constructor)| {
            let re = Regex::new(pattern).ok()?;
            re.find(input).map(|mat| {
                let matched_str = mat.as_str();
                Some((constructor(matched_str), matched_str.to_string()))
            })
        })?
    }
}
