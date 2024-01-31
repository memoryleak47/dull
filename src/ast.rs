#[derive(Debug)]
pub struct Data {
    pub name: String,
    pub arity: u32,
}

#[derive(Debug)]
pub struct FnDef {
    pub name: String,
    pub args: Vec<String>,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Match(Match),
}

#[derive(Debug)]
pub struct Match {
    pub head: Box<Expr>,
    pub arms: Vec<Arm>,
}

#[derive(Debug)]
pub enum Pattern {
    Variable(String),
    Data(String, /*vars: */ Vec<String>),
}

#[derive(Debug)]
pub struct Arm {
    pub pattern: Pattern,
    pub result: Expr,
}

#[derive(Debug)]
pub struct Ast {
    pub datas: Vec<Data>,
    pub fns: Vec<FnDef>,
}
