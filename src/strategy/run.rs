use crate::*;

pub type ValueId = usize;

define_language! {
    pub enum Semi {
        Class(usize), // this usize is conceptually an `Id`!
        "term" = L(Id),
    }
}

// TODO: open question:
// Should sigma only map pvars from main, or generally any variable from context?
pub type Sigma = Map<String, ValueId>;
pub type Deref = Map<ValueId, RecExpr<Semi>>;

pub fn eval(expr: &Expr, ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<GeneralLang, ()>) -> Vec<(Sigma, Deref, RecExpr<Semi>)> {
    todo!()
}
