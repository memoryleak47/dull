use crate::*;

define_language! {
    enum Semi {
        Class(usize), // this usize is conceptually an `Id`!
        "term" = L(Id),
    }
}

type Sigma = Map<String, RecExpr<Semi>>;

pub fn run(ast: &Ast, sigma: Sigma, eg: &EGraph<GeneralLang, ()>) -> Vec<(Sigma, RecExpr<Semi>)> {
    todo!()
}

struct ThreadState {
    
}
