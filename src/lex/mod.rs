use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    token: char,
    pub operation: fn(f64, f64) -> f64,
    pub precedence: u8,
    pub is_left_associative: bool,
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
    pub fn operate(self, x: f64, y: f64) -> f64 {
        (self.operation)(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    token: String,
    relation: fn(f64) -> f64,
}

impl Function {
    fn token_from_fn(token: String, relation: fn(f64) -> f64) -> Token {
        Token::Function(
            Function {
                token,
                relation
            }
        )
    }
    pub fn apply(self, arg: f64) -> f64 {
        (self.relation)(arg)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Num(f64),
    Function(Function),
    LParen,
    RParen
}

fn get_functions() -> HashMap<&'static str, Token> {
    return [
        ("sin", Function::token_from_fn("sin".into(), |x| x.to_radians().sin())),
        ("cos", Function::token_from_fn("cos".into(), |x| x.to_radians().cos())),
        ("tan", Function::token_from_fn("tan".into(), |x| x.to_radians().tan())),
        ("csc", Function::token_from_fn("csc".into(), |x| 1. / x.to_radians().sin())),
        ("sec", Function::token_from_fn("sec".into(), |x| 1. /  x.to_radians().cos())),
        ("cot", Function::token_from_fn("cot".into(), |x| 1. /  x.to_radians().tan())),
        ("ln", Function::token_from_fn("ln".into(), |x| x.ln())),
        ("log", Function::token_from_fn("log".into(), |x| x.log10())),
        ("sqrt", Function::token_from_fn("sqrt".into(), |x| x.sqrt())),
        ("ceil", Function::token_from_fn("ceil".into(), |x| x.ceil())),
        ("floor", Function::token_from_fn("floor".into(), |x| x.floor())),
    ].iter().cloned().collect();
}

fn get_operators() -> HashMap<char, Token> {
    return [
        ('+', Operator::token_from_op('+', |x, y| x + y, 2, true)),
        ('-', Operator::token_from_op('-', |x, y| x - y, 2, true)),
        ('*', Operator::token_from_op('*', |x, y| x * y, 3, true)),
        ('/', Operator::token_from_op('/', |x, y| x / y, 3, true)),
        ('^', Operator::token_from_op('^', |x, y| x.powf(y) , 4, true)),
    ].iter().cloned().collect();
}

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let functions: HashMap<&str, Token> = get_functions();
    let operators: HashMap<char, Token> = get_operators();

    let mut num_vec: String = String::new();
    let mut char_vec: String = String::new();
    let mut result: Vec<Token> = vec![];
    for letter in input.chars() {
        match letter {
            '0'...'9' | '.' => {
                num_vec.push(letter);
            },
            'a'...'z' | 'A'...'Z' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(operators.get(&'*').unwrap().clone());
                    num_vec.clear();
                }
                char_vec.push(letter);
            },
            '+' | '-' => {
                let op_token = operators.get(&letter).unwrap().clone();
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                    result.push(op_token);
                } else {
                    result.push(Token::LParen);
                    result.push(Token::Num((letter.to_string() + "1").parse::<f64>().unwrap()));
                    result.push(Token::RParen);
                    result.push(operators.get(&'*').unwrap().clone());
                }
            },
            '/' | '*' | '^' => {
                drain_num_stack(&mut num_vec, &mut result);
                let operator_token: Token = operators.get(&letter).unwrap().clone();
                result.push(operator_token);
            },
            '('  => {
                if char_vec.len() > 0 {
                    if let Some(res) = functions.get(&char_vec[..]) {
                        result.push(res.clone());
                    } else {
                        return Err(format!("Unexpected function {}", char_vec))
                    }
                    char_vec.clear();
                } else {
                    let parse_num = num_vec.parse::<f64>().ok();
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        result.push(operators.get(&'*').unwrap().clone());
                        num_vec.clear();
                    }
                }

                if let Some(x) = result.last() {
                    match x {
                        Token::RParen => {
                            result.push(operators.get(&'*').unwrap().clone());
                        },
                        _ => {}
                    };
                }
                result.push(Token::LParen);
            },
            ')' => {
                drain_num_stack(&mut num_vec, &mut result);
                result.push(Token::RParen);
            }
            ' ' => {}
            _ => {
                return Err(format!("Unexpected character: {}", letter))
            }
        }
    }
    drain_num_stack(&mut num_vec, &mut result);
    Ok(result)
}

fn drain_num_stack(num_vec: &mut String, result: &mut Vec<Token>) {
    let parse_num = num_vec.parse::<f64>().ok();
    if let Some(x) = parse_num {
        result.push(Token::Num(x));
        num_vec.clear();
    }
}
