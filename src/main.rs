mod ast;
pub use ast::*;

mod tokenize;
pub use tokenize::*;

mod assemble;
pub use assemble::*;

mod run;
pub use run::*;

mod strategy;
pub use strategy::*;

use std::fs::File;
use std::io::Read;
pub use egg::{Language, define_language, Id, Symbol, Rewrite, Searcher, Applier, EGraph, SearchMatches, Var, Subst, PatternAst, RecExpr};
use std::collections::HashMap as Map;

fn load_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Cannot find file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn main() {
    let filename = std::env::args().nth(1).unwrap_or(String::from("file.dull"));
    let s = load_file(&filename);
    let tks = tokenize(s);
    let ast = assemble(&tks);
    run(&ast);
}
