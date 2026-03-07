use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    Float,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Fn,
    Let,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntegerLiteral(i32),
    StringLiteral(String),
    BoolLiteral(bool),
    FloatLiteral(f64),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    //Comparison
    EQ,
    GT,
    LT,
    GEQ,
    LEQ,
    Assign,
    Arrow, // fn (x: a) -> y: a
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    Semicolon,
    Colon,
    Comma,
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
    Type(Type),
    Operator(Operator),
    Punctuation(Punctuation),
}

type TokenRule<'a> = (&'a str, fn(&str) -> Token);

impl Token {
    pub fn get_token(input: &str) -> Option<(Token, String)> {
        let patterns: Vec<TokenRule> = vec![
            (r"^[0-9]+\.[0-9]+", |s| {
                Token::Literal(Literal::FloatLiteral(s.parse::<f64>().unwrap()))
            }),
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
                "string" => Token::Type(Type::String),
                "int" => Token::Type(Type::Int),
                "float" => Token::Type(Type::Float),
                "bool" => Token::Type(Type::Bool),
                "let" => Token::Keyword(Keyword::Let),
                "fn" => Token::Keyword(Keyword::Fn),
                "true" => Token::Literal(Literal::BoolLiteral(true)),
                "false" => Token::Literal(Literal::BoolLiteral(false)),
                _ => Token::Identifier(s.to_string()),
            }),
            (r"^\*", |_| Token::Operator(Operator::Star)),
            (r"^/", |_| Token::Operator(Operator::Slash)),
            (r"^->", |_| Token::Operator(Operator::Arrow)),
            (r"^\+", |_| Token::Operator(Operator::Plus)),
            (r"^\-", |_| Token::Operator(Operator::Minus)),
            (r"^==", |_| Token::Operator(Operator::EQ)),
            (r"^=", |_| Token::Operator(Operator::Assign)),
            (r"^<=", |_| Token::Operator(Operator::LEQ)),
            (r"^>=", |_| Token::Operator(Operator::GEQ)),
            (r"^>", |_| Token::Operator(Operator::GT)),
            (r"^<", |_| Token::Operator(Operator::LT)),
            (r"^:", |_| Token::Punctuation(Punctuation::Colon)),
            (r"^;", |_| Token::Punctuation(Punctuation::Semicolon)),
            (r"^,", |_| Token::Punctuation(Punctuation::Comma)),
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
