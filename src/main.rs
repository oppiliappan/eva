#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    token: char,
    operation: fn(f64, f64) -> f64,
    precedence: u8,
    is_left_associative: bool,
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Num(f64),
    LParen,
    RParen
}

impl Operator {
    fn token_from_op(token: char, operation: fn(f64, f64) -> f64, precedence: u8, is_left_associative: bool) -> Token {
        Token::Operator(
            Operator {
                token,
                operation,
                precedence,
                is_left_associative
            }
        )
    }
    fn operate(self, x: f64, y: f64) -> f64 {
        (self.operation)(x, y)
    }
}

fn main() {
    let input = "1(2)";
    let input = input.replace(" ", "");
    let lexed = lexer(&input);
    println!("{:?}", lexed);
    println!("{}", lexed.unwrap().len());
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
                let operator_token: Token = match letter {
                    '+' => Operator::token_from_op('+', |x, y| x + y, 2, true),
                    '-' => Operator::token_from_op('-', |x, y| x - y, 2, true),
                    '/' => Operator::token_from_op('/', |x, y| x / y, 3, true),
                    '*' => Operator::token_from_op('*', |x, y| x * y, 3, true),
                    '^' => Operator::token_from_op('^', |x, y| x.powf(y), 4, false),
                    _ => panic!("unexpected op whuuu"),
                };
                result.push(operator_token);
            },
            '('  => {
                // parse num buf
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(Operator::token_from_op('*', |x, y| x * y, 3, true));
                    num_vec.clear();
                }
                // finish
                result.push(Token::LParen);
            },
            ')' => {
                // parse num buf
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
                // finish
                result.push(Token::RParen);
            }
            ' ' => {}
            _ => {
                return Err(format!("Unexpected character: {}", letter))
            }
        }
    }
    Ok(result)
}
