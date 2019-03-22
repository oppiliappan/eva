use std::f64;
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

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
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
                    result.push(Operator::token_from_op('*', |x, y| x * y, 3, true));
                    num_vec.clear();
                }
                char_vec.push(letter);
            },
            '+' | '-' | '/' | '*' | '^' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
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
                if char_vec.len() > 0 {
                    let funct = char_vec.clone();
                    match &funct[..] {
                        "sin" | "sine"      => result.push(Function::token_from_fn("sin".into(), |x| x.to_radians().sin())),
                        "cos" | "cosine"    => result.push(Function::token_from_fn("cos".into(), |x| x.to_radians().cos())),
                        "tan" | "tangent"   => result.push(Function::token_from_fn("tan".into(), |x| x.to_radians().tan())),
                        "csc" | "cosec"     => result.push(Function::token_from_fn("csc".into(), |x| 1f64 / x.to_radians().sin())),
                        "sec" | "secant"    => result.push(Function::token_from_fn("sec".into(), |x| 1f64 / x.to_radians().cos())),
                        "cot" | "cotangent" => result.push(Function::token_from_fn("cot".into(), |x| 1f64 / x.to_radians().tan())),
                        "ln"                => result.push(Function::token_from_fn("ln".into(), |x| x.ln())),
                        "log"               => result.push(Function::token_from_fn("log".into(), |x| x.log10())),
                        "sqrt"              => result.push(Function::token_from_fn("sqrt".into(), |x| x.sqrt())),
                        "floor"             => result.push(Function::token_from_fn("floor".into(), |x| x.floor())),
                        "ceil"              => result.push(Function::token_from_fn("ceil".into(), |x| x.ceil())),
                        _                   => return Err(format!("Unexpected function {}", funct))
                    }
                    char_vec.clear();
                } else {
                    let parse_num = num_vec.parse::<f64>().ok();
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        result.push(Operator::token_from_op('*', |x, y| x * y, 3, true));
                        num_vec.clear();
                    }
                }

                if let Some(x) = result.last() {
                    match x {
                        Token::RParen => {
                            result.push(Operator::token_from_op('*', |x, y| x * y, 3, true));
                        },
                        _ => {}
                    };
                }
                result.push(Token::LParen);
            },
            ')' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
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
