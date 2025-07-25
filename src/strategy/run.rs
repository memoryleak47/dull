use crate::*;

pub type ValueId = usize;

// TODO: open question:
// Should sigma only map pvars from main, or generally any variable from context?
pub type Sigma = Map<String, ValueId>;
pub type Deref = Map<ValueId, RecExpr<Semi>>;

pub fn eval(expr: &Expr, ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<GeneralLang, ()>) -> Vec<(Sigma, Deref, RecExpr<Semi>)> {
    match expr {
        Expr::Match(m) => todo!(),
        Expr::DataConstr(s, args) => todo!(),
        Expr::FnCall(f, args) => todo!(),
        Expr::Var(s) => todo!(),
    }
}
