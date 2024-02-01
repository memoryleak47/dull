use crate::*;

use std::fmt::{self, Display, Formatter};

use std::collections::HashMap;

#[derive(Clone)]
struct Value(String, Vec<Value>);

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)?;
        if !self.1.is_empty() {
            write!(f, "(")?;
            for (i, a) in self.1.iter().enumerate() {
                write!(f, "{}", a)?;
                if i != self.1.len() - 1 {
                write!(f, ", ")?;
                }
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

fn call_fn(name: &str, args: &[Value], ast: &Ast) -> Value {
    let fn_def: &FnDef = ast.fns.iter().find(|x| x.name == name).unwrap();
    let mut var_ctxt = HashMap::new();
    for (var, val) in fn_def.args.iter().zip(args.iter()) {
        var_ctxt.insert(var.clone(), (*val).clone());
    }
    eval_expr(&fn_def.expr, &var_ctxt, ast)
}

fn eval_expr(expr: &Expr, var_ctxt: &HashMap<String, Value>, ast: &Ast) -> Value {
    match expr {
        Expr::Match(Match { head, arms }) => {
            let head = eval_expr(head, var_ctxt, ast);
            for Arm { pattern, result } in arms {
                let mut var_ctxt = var_ctxt.clone();
                // pattern match:
                match pattern {
                    Pattern::Var(v) => { var_ctxt.insert(v.to_string(), head); },
                    Pattern::Data(name, vars) => {
                        if name != &head.0 { continue; }
                        if vars.len() != head.1.len() { continue; }

                        for (var, val) in vars.iter().zip(head.1.iter()) {
                            var_ctxt.insert(var.clone(), val.clone());
                        }
                    },
                }

                // evaluation
                return eval_expr(result, &var_ctxt, ast);
            }
            eprintln!("Runtime error! Non-exhaustive match occured!");
            std::process::exit(1)
        },
        Expr::DataConstr(name, args) => {
            let args: Vec<_> = args.iter().map(|x| eval_expr(x, &var_ctxt, ast)).collect();
            Value(name.to_string(), args)
        },
        Expr::FnCall(name, args) => {
            let args: Vec<_> = args.iter().map(|x| eval_expr(x, &var_ctxt, ast)).collect();
            call_fn(name, &args, ast)
        },
        Expr::Var(x) => var_ctxt[x].clone(),
    }
}

pub fn run(ast: &Ast) {
    let output = call_fn("main", &[], ast);
    println!("{}", output);
}
