#[derive(Debug)]
pub enum Token {
    Operator(char),
    Num(f64),
    LParen,
    RParen
}

#[derive(Debug)]
pub struct Node {
    value: Option<Token>,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>
}

impl Node {
    fn new() -> Node {
        Node {
            value: None,
            left: Box::new(None),
            right: Box::new(None)
        }
    }
    fn set_val(&mut self, val: Token) {
        self.value = Some(val);
    }
    fn set_left(&mut self, val: Node) {
        self.left = Box::new(Some(val));
    }
    fn set_right(&mut self, val: Node) {
        self.right = Box::new(Some(val));
    }
}

fn main() {
    let input = "11 + (1 + 2(3))";
    let input = input.replace(" ", "");

    let lexed = lexer(&input);

    println!("{:?}", lexed);
}

fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut num_vec: String = String::new();
    let mut result: Vec<Token> = vec![];
    for letter in input.chars() {
        match letter {
            '0'...'9' | '.' => {
                num_vec.push(letter);
            },
            '+' | '-' | '/' | '*' | '^' => {
                // parse num buf
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
                // finish
                result.push(Token::Operator(letter));
            },
            '('  => {
                // parse num buf
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(Token::Operator('*'));
                    num_vec.clear();
                }
                // finish
                result.push(Token::RParen);
            },
            ')' => {
                // parse num buf
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
                // finish
                result.push(Token::LParen);
            }
            _ => {
                return Err(format!("Unexpected character: {}", letter))
            }
        }
    }
    Ok(result)
}


