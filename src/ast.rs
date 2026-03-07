#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntegerLiteral(i32),
    StringLiteral(String),
    BoolLiteral(bool),
    FloatLiteral(f64),
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Bool,
    Float,
    String,
    Func(Vec<Box<Type>>, Option<Box<Type>>),
}

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    //Comparison
    EQ, // ==
    GT,
    LT,
    GEQ,
    LEQ,
}

#[derive(Clone, Debug)]
pub struct Block {
    body: Vec<Stmt>,
    ret_expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct BinaryOp {
    left: Box<Expr>,
    op: Operator,
    right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Param {
    id: String,
    p_type: Type,
}

#[derive(Clone, Debug)]
pub struct Func {
    id: String,
    params: Vec<Param>,
    ret_type: Option<Type>,
    body: Block,
}

#[derive(Clone, Debug)]
pub struct IfElse {
    condition: Box<Expr>,
    then: Block,
    else_branch: Option<Block>,
}

#[derive(Clone, Debug)]

pub struct FuncCall {
    id: String,
    params: Vec<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    BinaryOp(BinaryOp),
    Func(Func),
    Identifier(String),
    FuncCall(FuncCall),
    IfElse(IfElse),
}

#[derive(Clone, Debug)]
pub struct LetBinding {
    id: String,
    var_type: Option<Type>,
    val: Box<Expr>,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    LetBinding(LetBinding),
    FuncCall(FuncCall),
}

pub struct AST {
    program: Vec<Func>,
}
