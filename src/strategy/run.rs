use crate::*;

pub type ValueId = Id;

pub type Sigma = Map<String, ValueId>; // same as var_ctxt in previous impl.
pub type Deref = Map<ValueId, Semi>; // Children of Semi are ValueIds again.

fn eval_args(args: &[Expr], ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, Vec<ValueId>)> {
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
    states
}

pub fn eg_eval(expr: &Expr, ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    match expr {
        Expr::Match(m) => {
            let mut out = Vec::new();
            for (deref, vid) in eg_eval(&m.head, ast, sigma, deref, eg) {
                out.extend(eg_match(vid, &m.arms, deref, ast, eg));
            }
            out
        },
        Expr::DataConstr(s, args) => {
            let states = eval_args(args, ast, sigma, deref, eg);
            states.into_iter().map(|(mut deref, args)| {
                let vid1 = ValueId::from(deref.len());
                deref.insert(vid1, Semi::L(GeneralLang::Constant(Symbol::from(s))));

                let vid2 = ValueId::from(deref.len());
                let args = std::iter::once(vid1).chain(args.into_iter()).collect();
                deref.insert(vid2, Semi::L(GeneralLang::App(args)));
                (deref, vid2)
            }).collect()
        },
        Expr::FnCall(f, args) => {
            let states = eval_args(args, ast, sigma, deref, eg);

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
    let f = ast.fns.iter().find(|x| x.name == name).unwrap();

    let mut sigma = Sigma::new();
    for (a, v) in f.args.iter().zip(args.iter()) {
        sigma.insert(a.clone(), *v);
    }

    eg_eval(&f.expr, ast, sigma, deref, eg)
}

fn eg_match(vid: ValueId, arms: &[Arm], deref: Deref, ast: &Ast, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    let c = match deref[&vid].clone() {
        Semi::L(l) => return eg_match_l(l, arms, deref, ast, eg),
        Semi::Class(c) => c,
    };
    todo!()
}

fn eg_match_l(l: GeneralLang, arms: &[Arm], deref: Deref, ast: &Ast, eg: &EGraph<GeneralLang, ()>) -> Vec<(Deref, ValueId)> {
    todo!()
}
