use crate::*;

pub type ValueId = Id;

pub type Sigma = Map<String, ValueId>; // same as var_ctxt in previous impl.
pub type Deref = Map<ValueId, Semi>; // Children of Semi are ValueIds again.

pub fn eg_eval(expr: &Expr, ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    match expr {
        Expr::Match(m) => todo!(),
        Expr::DataConstr(s, args) => todo!(),
        Expr::FnCall(f, args) => {
            let mut states: Vec<(Deref, /*args*/Vec<ValueId>)> = vec![(deref, Vec::new())];
            for a in args.iter() {
                states = states.into_iter().map(|(deref, args)| {
                    eg_eval(a, ast, sigma.clone(), deref, eg)
                        .into_iter()
                        .map(move |(deref, a)| {
                            let mut args = args.clone();
                            args.push(a);
                            (deref, args)
                        })
                }).flatten().collect();
            }
            let mut outs = Vec::new();
            for (deref, args) in states {
                outs.extend(eg_call_fn(f, &args, deref, ast, eg));
            }
            outs
        },
        Expr::Var(s) => vec![(deref, sigma[s])]
    }
}

pub fn eg_call_fn(name: &str, args: &[ValueId], deref: Deref, ast: &Ast, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    todo!()
}
