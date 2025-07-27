mod ast;
pub use ast::*;

mod tokenize;
pub use tokenize::*;

mod assemble;
pub use assemble::*;

mod strategy;
pub use strategy::*;

use std::fs::File;
use std::io::Read;
pub use egg::{Language, define_language, Id, Symbol, Rewrite, Searcher, Applier, EGraph, SearchMatches, Var, Subst, PatternAst, RecExpr, SymbolLang, Runner};
use std::collections::HashMap as Map;
use std::str::FromStr;

fn load_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Cannot find file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn dull_rewrite(filename: &str) -> Rewrite<SymbolLang, ()> {
    let s = load_file(&filename);
    let tks = tokenize(s);
    let ast = assemble(&tks);
    mk_dull_rewrite(ast)
}

fn main() {
    let rw = dull_rewrite("rw1.dull");
    // let j = g.add_term(RecExpr::parse("(Suc foo)").unwrap());
    let mut runner: Runner<SymbolLang, ()> = Runner::new(());
    let i = runner.egraph.add_expr(&RecExpr::from_str("foo").unwrap());
    let j = runner.egraph.add_expr(&RecExpr::from_str("(Suc foo)").unwrap());
    let runner = runner.run(&[rw]);
    dbg!(runner.egraph.find(i));
    dbg!(runner.egraph.find(j));
}
