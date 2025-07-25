use crate::*;
use egg::*;

use std::sync::mpsc::*;
use std::sync::Mutex;
use std::str::FromStr;

define_language! {
    enum GeneralLang {
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

fn mk_dull_rewrite(a: Ast) -> Rewrite<GeneralLang, ()> {
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
    ) -> Option<SearchMatches<'_, GeneralLang>> { todo!() }

    fn vars(&self) -> Vec<Var> {
        for x in &self.ast.fns {
            if x.name != "main" { continue }
            return x.args.iter().map(|x| Var::from_str(&*x).unwrap()).collect();
        }
        panic!("mo main FnDef found!")
    }
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
