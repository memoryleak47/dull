use crate::*;

pub type ValueId = Id;

pub type Sigma = Map<String, ValueId>; // same as var_ctxt in previous impl.
pub type Deref = Map<ValueId, Semi>; // Children of Semi are ValueIds again.

pub fn eg_eval(expr: &Expr, ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    match expr {
        Expr::Match(m) => todo!(),
        Expr::DataConstr(s, args) => todo!(),
        Expr::FnCall(f, args) => todo!(),
        Expr::Var(s) => vec![(deref, sigma[s])]
    }
}

pub fn eg_call_fn(name: &str, args: &[ValueId], deref: Deref, ast: &Ast, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    todo!()
}
