#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    token: char,
    operation: fn(f64, f64) -> f64,
    precedence: u8,
    is_left_associative: bool,
}

#[derive(Debug, Copy, Clone)]
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
    let input = "1 + 5 * 6 + 2";
    println!("{}", input);
    let input = input.replace(" ", "");
    let lexed = lexer(&input);
    let postfixed = to_postfix(lexed.unwrap());
    let evaled = eval_postfix(postfixed.unwrap());

    println!("{:?}", evaled);
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
                if let Some(x) = result.last() {
                    match x {
                        Token::RParen => {
                            result.push(Operator::token_from_op('*', |x, y| x * y, 3, true));
                        },
                        _ => {}
                    };
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
    let parse_num = num_vec.parse::<f64>().ok();
    if let Some(x) = parse_num {
        result.push(Token::Num(x));
        num_vec.clear();
    }
    Ok(result)
}

fn to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut postfixed: Vec<Token> = vec![];
    let mut op_stack: Vec<Token> = vec![];
    for token in tokens {
        match token {
            Token::Num(_) => {
                postfixed.push(token);
            },
            Token::Operator(current_op) => {
                while let Some(top_op) = op_stack.last() {
                    match top_op {
                        Token::LParen => {
                            break;
                        }
                        Token::Operator(top_op) => {
                            let tp = top_op.precedence;
                            let cp = current_op.precedence;
                            if tp > cp || (tp == cp && top_op.is_left_associative) {
                                postfixed.push(op_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => {
                            return Err(format!("Unexpected match branch part 2"))
                        }
                    }
                }
                op_stack.push(token);
            },
            Token::LParen => {
                op_stack.push(token);
            },
            Token::RParen => {
                let mut found: bool = false;
                while let Some(top_op) = op_stack.last() {
                    match top_op {
                        Token::LParen => {
                            let _ = op_stack.pop().unwrap();
                            found = true;
                        },
                        _ => {
                            postfixed.push(op_stack.pop().unwrap());
                        }
                    }
                }
                if found == false {
                    return Err(format!("Mismatched parentheses part 2"))
                }
            }

        }
    }
    while let Some(op) = op_stack.pop() {
        postfixed.push(op);
    }
    Ok(postfixed)
}

fn eval_postfix(postfixed: Vec<Token>) -> Result<f64, String> {
    let mut num_stack: Vec<f64> = vec![];
    for token in postfixed {
        match token {
            Token::Num(n) => {
                num_stack.push(n);
            },
            Token::Operator(op) => {
                if let Some(n2) = num_stack.pop() {
                    if let Some(n1) = num_stack.pop() {
                        num_stack.push(op.operate(n1, n2))
                    } else {
                        return Err(format!("Too many operators, Too little operands"))
                    }
                } else {
                    return Err(format!("Too many operators, Too little operands"))
                }
            }
            _ => {
                return Err(format!("Yo nibba how did this get here"))
            }
        }
    }
    if num_stack.len() == 1 {
        Ok(num_stack.pop().unwrap())
    } else {
        Err(format!("Parser Error"))
    }
}
