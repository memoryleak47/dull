use crate::*;

use std::sync::mpsc::*;
use std::sync::Mutex;
use std::str::FromStr;

mod run;
pub use run::*;

// basic idea:
// We write a dull function "main" with one argument (representing the term we e-match over).
// def main(x) {
//     match x {
//       Suc(Dec(a)) => a,
//       _ => x,
//     }
// }
//
// For now we simply add the rewrite "x => main(x)",
// but later on main could return us insightful information and specialized commands (and failure options)

define_language! {
    pub enum GeneralLang {
        Constant(Symbol),
        "app" = App(Box<[Id]>), // (f, arg1, ...)
    }
}

// runs the FnDef called "main".
// Any argument to "main" is a ?-var.
struct DullSearcher {
    ast: Ast,
    sender: Mutex<Sender<DullInfo>>,
}

struct DullApplier {
    ast: Ast,
    receiver: Mutex<Receiver<DullInfo>>,
}

struct DullInfo;

pub fn mk_dull_rewrite(a: Ast) -> Rewrite<GeneralLang, ()> {
    let (sender, receiver) = channel::<DullInfo>();
    let sender = Mutex::new(sender);
    let receiver = Mutex::new(receiver);
    let searcher = DullSearcher {
        ast: a.clone(),
        sender,
    };
    let applier = DullApplier {
        ast: a.clone(),
        receiver,
    };
    Rewrite::new("dull", searcher, applier).unwrap()
}

impl Searcher<GeneralLang, ()> for DullSearcher {
    fn search_eclass_with_limit(
        &self,
        egraph: &EGraph<GeneralLang, ()>,
        eclass: Id,
        limit: usize,
    ) -> Option<SearchMatches<'_, GeneralLang>> {
        let expr = Expr::FnCall("main".to_string(), vec![Expr::Var("x".to_string())]);
        let mut sigma = Sigma::new();
        let mut deref = Deref::new();
        sigma.insert("x".to_string(), 0);
        let mut re = RecExpr::default();
        re.add(Semi::Class(usize::from(eclass)));
        deref.insert(0, re);
        let _ = eval(&expr, &self.ast, sigma, deref, egraph);
        todo!()
    }

    // I'm not using egg variables at all!
    fn vars(&self) -> Vec<Var> { Vec::new() }
}

impl Applier<GeneralLang, ()> for DullApplier {
   fn apply_one(
        &self,
        egraph: &mut EGraph<GeneralLang, ()>,
        eclass: Id,
        subst: &Subst,
        searcher_ast: Option<&PatternAst<GeneralLang>>,
        rule_name: Symbol,
    ) -> Vec<Id> { todo!() }
}
