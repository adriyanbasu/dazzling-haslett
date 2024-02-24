use std::{string, vec};

#[derive(Debug)]
enum Errors {}

enum TokenType {
    SemiColon,
}

struct Token {
    pub ty: TokenType,
    pub value: String,
    pub errors: Errors,
}

fn tokenize(code: &str) -> Result<Vec<Token>, Errors> {
    let tokens = vec::new();
    let curent_token: Option<Token> = None;
    for c in code.chars() {
        if c == ';' {
            if let Some(tok) = curent_token {
                tokens.push(tok);
            }
            tokens.push(Token {
                ty: TokenType::SemiColon,
                value: string::from(';'),
                errors: Errors {},
            });
            curent_token = None;
        }
    }
    Ok(tokens)
}

fn main() {
    let tokens = tokenize("x = 2;").unwrap();

    println!("{:?}", tokens);
}
