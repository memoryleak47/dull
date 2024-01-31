mod ast;
pub use ast::*;

mod tokenize;
pub use tokenize::*;

mod assemble;
pub use assemble::*;

use std::fs::File;
use std::io::Read;

fn load_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn main() {
    let s = load_file("file.dull");
    let tks = tokenize(s);
    let ast = assemble(&tks);
    dbg!(ast);
}
