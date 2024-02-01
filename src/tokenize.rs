#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LowerIdent(String), UpperIdent(String), Comma, LParen, RParen, LBrace, RBrace, Arrow,
    Match, Fn
}

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut current_ident: Option<String> = None;

    let ident_to_token = |id: String| {
        if id == "match" { Token::Match }
        else if id == "fn" { Token::Fn }
        else if id.chars().next().unwrap().is_uppercase() { Token::UpperIdent(id) }
        else { Token::LowerIdent(id) }
    };

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_alphanumeric() || c == '_' {
            if current_ident.is_none() { current_ident = Some(String::new()); }
            current_ident.as_mut().unwrap().push(c);
            i += 1;
            continue;
        }

        if let Some(id) = current_ident {
            tokens.push(ident_to_token(id));
            current_ident = None;
        }

        if c.is_whitespace() { }
        else if c == ',' { tokens.push(Token::Comma); }
        else if c == '(' { tokens.push(Token::LParen); }
        else if c == ')' { tokens.push(Token::RParen); }
        else if c == '{' { tokens.push(Token::LBrace); }
        else if c == '}' { tokens.push(Token::RBrace); }
        else if c == '=' && chars[i+1] == '>' {
            tokens.push(Token::Arrow);
            i += 1;
        } else {
            panic!("unsupported character: {}", c);
        }

        i += 1;
    }

    if let Some(id) = current_ident {
        tokens.push(ident_to_token(id));
    }

    tokens
}
