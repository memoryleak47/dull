#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub args: Vec<String>,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Match(Match),
    DataConstr(String, Vec<Expr>),
    FnCall(String, Vec<Expr>),
    Var(String),
}

#[derive(Debug, Clone)]
pub struct Match {
    pub head: Box<Expr>,
    pub arms: Vec<Arm>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Var(String),
    Data(String, Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub struct Arm {
    pub pattern: Pattern,
    pub result: Expr,
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub fns: Vec<FnDef>,
}
