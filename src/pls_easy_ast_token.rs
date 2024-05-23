#[derive(Debug)]
enum Token {
    Plus,
    Multiply,
    Number(i32),
}

#[derive(Debug)]
enum AST {
    Add(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    Number(i32),
}

// "1 + 2 * 3" => "1+2*3"
// => <Token>[TokenNumber(1),Plus,TokenNumber(2),Multiply,TokenNumber(3)]
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in input.chars() {
        match c {
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Multiply),
            '0'..='9' => tokens.push(Token::Number(c.to_digit(10).unwrap() as i32)),
            _ => {}
        }
    }
    tokens
}

fn parse(tokens: &[Token]) -> AST {
    match tokens {
        [Token::Number(n), Token::Multiply, rest @ ..] => {
            AST::Multiply(Box::new(AST::Number(*n)), Box::new(parse(rest)))
        }
        [Token::Number(n), Token::Plus, rest @ ..] => {
            AST::Add(Box::new(AST::Number(*n)), Box::new(parse(rest)))
        }
        [Token::Number(n)] => AST::Number(*n),
        _ => panic!("unexpected token sequence"),
    }
}
pub fn run() {
    // wrong
    let input = "2*3+4";
    let tokens = tokenize(input);
    let ast = parse(&tokens);
    println!("{:?}", ast);
}
