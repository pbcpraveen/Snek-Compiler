use im::HashMap;

pub const OFFSET_SCALE: i64 = 8;
pub const MIN : i64 = -i64::pow(2, 62);
pub const MAX : i64 = i64::pow(2, 62) - 1;
pub const KEYWORDS : &'static [&'static str] = &["add1", "sub1", "isnum", "isbool", "let", "set!",
                                            "block", "print", "set!", "fun", "if", "break", "true",
                                            "false", "loop"];
pub const VALUE_1 : i64 = 2;
pub const TRUE : i64 = 3;
pub const FALSE : i64 = 1;
pub const ERROR_INVALID_ARGUMENT : i64 = 1;
pub const ERROR_OVERFLOW : i64 = 2;

pub const ROUTINE_PRINT : &str = "snek_print";
pub const ROUTINE_ERROR : &str = "throw_error";

#[derive(Debug)]
pub struct Context<'a> {
    pub si: i64,
    pub env: &'a HashMap<String, Loc>,
    pub brake: &'a str,
    pub target: Loc,
    pub user_defined_functions: &'a HashMap<String, i64>,
    pub current_function: &'a str,
}
#[derive(Debug)]
pub struct Program {
    pub defs: Vec<Definition>,
    pub main: Expr,
}

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expr,
}



#[derive(Clone,Copy,Debug)]
pub enum Reg {
    RAX,
    RBX,
    RSP,
    RDI,
    RCX,
    RDX,
}

#[derive(Clone,Copy,Debug)]
pub enum Loc {
    LReg(Reg),
    LStack(i64)
}


#[derive(Clone,Copy,Debug)]
pub enum Val {
    VReg(Reg),
    VStack(i64),
    VImm(i64),
}



#[derive(Debug)]
pub enum Instr {
    IMov(Loc, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    ICmp(Val, Val),
    IJnz(String),
    IJz(String),
    IJmp(String),
    IJe(String),
    IJne(String),
    IJl(String),
    IJg(String),
    IJle(String),
    IJge(String),
    ILabel(String),
    ITest(Val, Val),
    ICmove(Val, Val),
    ICmovne(Val, Val),
    IAnd(Val, Val),
    INot(Val),
    IXor(Val, Val),
    IPush(Val),
    IPop(Val),
    ISar(Val, Val),
    ISal(Val, Val),
    IShr(Val, Val),
    IJo(String),
    IRet,
    ICall(String),
}

#[derive(Debug)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    Print,
}

#[derive(Debug)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Greater,
    Less,
    Equal,
    GreaterEqual,
    LessEqual,
}


#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Call(String, Vec<Expr>),
}
