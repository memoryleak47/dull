use crate::*;

pub fn assemble(mut tokens: &[Token]) -> Ast {
    let mut ast = Ast {
        datas: Vec::new(),
        fns: Vec::new(),
    };

    while !tokens.is_empty() {
        if tokens[0] == Token::Data {
            tokens = assemble_data(tokens, &mut ast);
        } else {
            tokens = assemble_fn(tokens, &mut ast);
        }
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
        let Token::Ident(v) = &tokens[0] else { panic!() };
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

fn assemble_data<'t>(mut tokens: &'t [Token], ast: &mut Ast) -> &'t [Token] {
    assert_eq!(tokens[0], Token::Data);
    tokens = &tokens[1..];

    let Token::Ident(name) = &tokens[0] else { panic!() };
    let name = name.to_string();
    tokens = &tokens[1..];

    if tokens[0] == Token::Semicolon {
        ast.datas.push(Data {
            name, arity: 0
        });
        return &tokens[1..];
    }

    let (varlist, t2) = assemble_varlist(tokens);
    tokens = t2;

    assert_eq!(tokens[0], Token::Semicolon);
    tokens = &tokens[1..];

    ast.datas.push(Data {
        name, arity: varlist.len() as u32
    });

    tokens
}

fn assemble_expr(tokens: &[Token]) -> (Expr, &[Token]) {
    panic!()
}

fn assemble_fn<'t>(mut tokens: &'t [Token], ast: &mut Ast) -> &'t [Token] {
    assert_eq!(tokens[0], Token::Fn);
    tokens = &tokens[1..];

    let Token::Ident(name) = &tokens[0] else { panic!() };
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
