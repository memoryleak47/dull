use crate::*;

pub fn assemble(mut tokens: &[Token]) -> Ast {
    let mut ast = Ast {
        fns: Vec::new(),
    };

    while !tokens.is_empty() {
        tokens = assemble_fn(tokens, &mut ast);
    }

    ast
}

fn assemble_varlist(mut tokens: &[Token]) -> (Vec<String>, &[Token]) {
    let mut vars = Vec::new();

    assert_eq!(tokens[0], Token::LParen);
    tokens = &tokens[1..];

    if tokens[0] == Token::RParen {
        tokens = &tokens[1..];
        return (vars, tokens);
    }

    loop {
        let Token::LowerIdent(v) = &tokens[0] else { panic!() };
        vars.push(v.to_string());
        tokens = &tokens[1..];

        if tokens[0] == Token::RParen {
            tokens = &tokens[1..];
            return (vars, tokens);
        }

        assert_eq!(tokens[0], Token::Comma);
        tokens = &tokens[1..];
    }
}

fn assemble_pattern(mut tokens: &[Token]) -> (Pattern, &[Token]) {
    if let Token::UpperIdent(i) = &tokens[0] {
        let i = i.to_string();
        tokens = &tokens[1..];
        if tokens[0] == Token::LParen {
            let (vars, t2) = assemble_varlist(tokens);
            tokens = t2;
            (Pattern::Data(i, vars), tokens)
        } else {
            (Pattern::Data(i, vec![]), tokens)
        }
    } else if let Token::LowerIdent(i) = &tokens[0] {
        tokens = &tokens[1..];
        (Pattern::Var(i.to_string()), tokens)
    } else { panic!() }
}

fn assemble_match(mut tokens: &[Token]) -> (Match, &[Token]) {
    assert_eq!(tokens[0], Token::Match);
    tokens = &tokens[1..];

    let (expr, t2) = assemble_expr(tokens);
    tokens = t2;

    assert_eq!(tokens[0], Token::LBrace);
    tokens = &tokens[1..];

    let mut arms = Vec::new();

    loop {
        if tokens[0] == Token::RBrace {
            tokens = &tokens[1..];
            return (Match { head: Box::new(expr), arms }, tokens);
        }

        let (pattern, t2) = assemble_pattern(tokens);
        tokens = t2;

        assert_eq!(tokens[0], Token::Arrow);
        tokens = &tokens[1..];

        let (arm_expr, t2) = assemble_expr(tokens);
        tokens = t2;

        assert_eq!(tokens[0], Token::Comma);
        tokens = &tokens[1..];

        arms.push(Arm { pattern, result: arm_expr });
    }
}

fn assemble_expr_list(mut tokens: &[Token]) -> (Vec<Expr>, &[Token]) {
    let mut exprs = Vec::new();

    assert_eq!(tokens[0], Token::LParen);
    tokens = &tokens[1..];

    if tokens[0] == Token::RParen {
        tokens = &tokens[1..];
        return (exprs, tokens);
    }

    loop {
        let (expr, t2) = assemble_expr(tokens);
        exprs.push(expr);
        tokens = t2;

        if tokens[0] == Token::RParen {
            tokens = &tokens[1..];
            return (exprs, tokens);
        }

        assert_eq!(tokens[0], Token::Comma);
        tokens = &tokens[1..];
    }
}

fn assemble_expr(mut tokens: &[Token]) -> (Expr, &[Token]) {
    if tokens[0] == Token::Match {
        let (m, t2) = assemble_match(tokens);
        (Expr::Match(m), t2)
    } else if let Token::UpperIdent(name) = &tokens[0] {
        let name = name.to_string();
        tokens = &tokens[1..];
        if tokens[0] == Token::LParen {
            let (exprs, t2) = assemble_expr_list(tokens);
            tokens = t2;
            (Expr::DataConstr(name, exprs), tokens)
        } else {
            (Expr::DataConstr(name, vec![]), tokens)
        }
    } else if let Token::LowerIdent(name) = &tokens[0] {
        tokens = &tokens[1..];
        if tokens[0] == Token::LParen {
            // function call!
            let (exprs, t2) = assemble_expr_list(tokens);
            tokens = t2;
            (Expr::FnCall(name.to_string(), exprs), tokens)
        } else {
            (Expr::Var(name.to_string()), tokens)
        }
    } else { panic!() }
}

fn assemble_fn<'t>(mut tokens: &'t [Token], ast: &mut Ast) -> &'t [Token] {
    assert_eq!(tokens[0], Token::Fn);
    tokens = &tokens[1..];

    let Token::LowerIdent(name) = &tokens[0] else { panic!() };
    let name = name.to_string();
    tokens = &tokens[1..];

    let (args, t2) = assemble_varlist(tokens);
    tokens = t2;

    assert_eq!(tokens[0], Token::LBrace);
    tokens = &tokens[1..];

    let (expr, t2) = assemble_expr(tokens);
    tokens = t2;

    assert_eq!(tokens[0], Token::RBrace);
    tokens = &tokens[1..];

    ast.fns.push(FnDef {
        name,
        args,
        expr
    });

    tokens
}
