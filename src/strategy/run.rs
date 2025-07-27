use crate::*;

pub type ValueId = Id;

pub type Sigma = Map<String, ValueId>; // same as var_ctxt in previous impl.
pub type Deref = Map<ValueId, Semi>; // Children of Semi are ValueIds again.

fn eval_args(args: &[Expr], ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<SymbolLang, ()>) -> Vec<(Deref, Vec<ValueId>)> {
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

pub fn eg_eval(expr: &Expr, ast: &Ast, sigma: Sigma, deref: Deref, eg: &EGraph<SymbolLang, ()>) -> Vec<(Deref, ValueId)> {
    match expr {
        Expr::Match(m) => {
            let mut out = Vec::new();
            for (deref, vid) in eg_eval(&m.head, ast, sigma.clone(), deref, eg) {
                out.extend(eg_match(vid, &m.arms, sigma.clone(), deref, ast, eg));
            }
            out
        },
        Expr::DataConstr(s, args) => {
            let states = eval_args(args, ast, sigma, deref, eg);
            states.into_iter().map(|(mut deref, args)| {
                let vid = ValueId::from(deref.len());
                let g = SymbolLang {
                    op: Symbol::from(s),
                    children: args.into(),
                };
                deref.insert(vid, Semi::L(g));
                (deref, vid)
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

pub fn eg_call_fn(name: &str, args: &[ValueId], deref: Deref, ast: &Ast, eg: &EGraph<SymbolLang, ()>) -> Vec<(Deref, ValueId)> {
    let f = ast.fns.iter().find(|x| x.name == name).unwrap();

    let mut sigma = Sigma::new();
    for (a, v) in f.args.iter().zip(args.iter()) {
        sigma.insert(a.clone(), *v);
    }

    eg_eval(&f.expr, ast, sigma, deref, eg)
}

fn eg_match(vid: ValueId, arms: &[Arm], sigma: Sigma, deref: Deref, ast: &Ast, eg: &EGraph<SymbolLang, ()>) -> Vec<(Deref, ValueId)> {
    let mut outs: Vec<(Deref, ValueId)> = Vec::new();

    // The states sent to the next match arm.
    let mut nexts: Vec<Deref> = vec![deref];

    for arm in arms {
        for deref in nexts.split_off(0) {
            for (deref, opt_sigma) in eg_match_pat(vid, &arm.pattern, Sigma::new(), deref, eg) {
                match opt_sigma {
                    Some(subsigma) => {
                        let mut sigma = sigma.clone();
                        sigma.extend(subsigma);
                        outs.extend(eg_eval(&arm.result, ast, sigma, deref, eg));
                    },
                    None => nexts.push(deref),
                }
            }
        }
    }
    outs
}

// If the returned Sigma is None, the pattern didn't match in that case.
// The returned Sigma only contains freshly matched variables. Not the old context.
fn eg_match_pat(vid: ValueId, pat: &Pattern, mut sigma: Sigma, deref: Deref, eg: &EGraph<SymbolLang, ()>) -> Vec<(Deref, Option<Sigma>)> {
    match pat {
        Pattern::Var(v) => {
            // This checks that a pattern that matches the same variable multiple times,
            // works out as intended.
            // patterns like D(x, x).
            // TODO I think sometimes both things can be equal, but they don't have the same ValueId yet! unification of ValueIds would be required here I fear.
            if let Some(old) = sigma.insert(v.to_string(), vid) && old != vid {
                vec![(deref, None)]
            } else {
                vec![(deref, Some(sigma))]
            }
        },
        Pattern::Data(constr, subpats) => {
            // branch 'vid' up.
            let mut cases: Vec<Deref> = Vec::new();
            if let Semi::Class(c) = &deref[&vid] {
                for n in &eg[*c].nodes {
                    let mut n = n.clone();
                    let mut deref = deref.clone();
                    for ch in n.children_mut() {
                        let v = ValueId::from(deref.len());
                        deref.insert(v, Semi::Class(*ch));
                        *ch = v;
                    }
                    deref.insert(vid, Semi::L(n.clone()));
                    cases.push(deref);
                }
            } else { cases.push(deref); };

            // match on these options.
            let mut outs = Vec::new();
            for deref in cases {
                let Semi::L(l) = &deref[&vid] else { unreachable!() };
                if Symbol::from(constr) != l.op || subpats.len() != l.children.len() {
                    outs.push((deref, None));
                    continue;
                }

                // The current partially matched state.
                // Some variables might have been found (thus we require sigma).
                let mut nexts = vec![(deref.clone(), sigma.clone())];
                for (p, x) in subpats.iter().zip(l.children()) {
                    for (deref, sigma) in nexts.split_off(0) {
                        for (deref, opt_sigma) in eg_match_pat(*x, p, sigma, deref, eg) {
                            if let Some(sigma) = opt_sigma {
                                nexts.push((deref, sigma));
                            } else {
                                // we push to `outs` and not `nexts`, as we don't want to pattern match the remaining children, if one match already failed.
                                outs.push((deref, None));
                            }
                        }
                    }
                }
                outs.extend(nexts.into_iter().map(|(deref, sigma)| (deref, Some(sigma))));
            }
            outs
        },
    }
}
