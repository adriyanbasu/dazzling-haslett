use std::vec;

enum Errors {}

enum TokenType {}

struct Token {
    pub ty: TokenType,
    pub value: String,
}

fn tokenize(code: &str) -> Result<Vec<Token>, Errors> {
    let tokens = vec::new();
    let curent_token: Option<Token> = None;
    for c in code.chars() {
        if c == ';' {}
    }
    Ok(tokens)
}

fn main() {
    let tokens = tokenize("x = 2;").unwrap();

    println!("{:?}", tokens);
}
