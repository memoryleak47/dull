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

    assert_eq!(tokens[0], Token::LParen);
    tokens = &tokens[1..];

    let mut idents = 0;
    loop {
        assert!(matches!(tokens[0], Token::Ident(_)));
        tokens = &tokens[1..];
        idents += 1;

        if tokens[0] == Token::RParen {
            tokens = &tokens[1..];
            break;
        }

        assert_eq!(tokens[0], Token::Comma);
        tokens = &tokens[1..];
    }
    assert_eq!(tokens[0], Token::Semicolon);
    tokens = &tokens[1..];

    ast.datas.push(Data {
        name, arity: idents as u32
    });

    tokens
}

fn assemble_fn<'t>(tokens: &'t [Token], ast: &mut Ast) -> &'t [Token] {
    &tokens[1..]
    // panic!()
}
