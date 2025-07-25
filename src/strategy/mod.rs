use crate::*;

use std::sync::mpsc::*;
use std::sync::Mutex;
use std::str::FromStr;

mod semi;
pub use semi::*;

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

struct DullInfo {
    eqs: Vec<[RecExpr<Semi>; 2]>,
}

pub fn mk_dull_rewrite(a: Ast) -> Rewrite<SymbolLang, ()> {
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

impl Searcher<SymbolLang, ()> for DullSearcher {
    fn search_eclass_with_limit(
        &self,
        egraph: &EGraph<SymbolLang, ()>,
        eclass: Id,
        limit: usize,
    ) -> Option<SearchMatches<'_, SymbolLang>> {
        let mut deref = Deref::new();
        let vid = ValueId::from(0);
        let semi = Semi::Class(eclass);
        deref.insert(vid, semi.clone());
        let outs = eg_call_fn("main", &[vid], deref, &self.ast, egraph);

        let mut eqs = Vec::new();

        for (deref, x) in outs {
            let lhs = RecExpr::from(vec![semi.clone()]);
            let rhs = deref_extract(x, &deref);
            eqs.push([lhs, rhs]);
        }
        self.sender.lock().unwrap().send(DullInfo { eqs });
        Some(SearchMatches {
            eclass,
            substs: vec![Subst::default()],
            ast: None,
        })
    }

    // I'm not using egg variables at all!
    fn vars(&self) -> Vec<Var> { Vec::new() }
}

fn deref_extract(vid: ValueId, deref: &Deref) -> RecExpr<Semi> {
    match &deref[&vid] {
        Semi::Class(c) => RecExpr::from(vec![Semi::Class(*c)]),
        Semi::L(l) => Semi::L(l.clone()).join_recexprs(|i| deref_extract(i, deref)),
    }
}

impl Applier<SymbolLang, ()> for DullApplier {
   fn apply_one(
        &self,
        egraph: &mut EGraph<SymbolLang, ()>,
        eclass: Id,
        subst: &Subst,
        searcher_ast: Option<&PatternAst<SymbolLang>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        let info = self.receiver.lock().unwrap().recv().unwrap();
        let mut out = Vec::new(); // this "out" technically doesn't cover newly added subterms, but I don't think I care.
        for [x, y] in info.eqs {
            let x = add_semi(x, egraph);
            let y = add_semi(y, egraph);
            out.push(x);
            out.push(y);
            egraph.union(x, y);
        }
        out
    }
}
