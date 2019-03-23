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
        ("sin",   Function::token_from_fn("sin".into(), |x| x.to_radians().sin())),
        ("cos",   Function::token_from_fn("cos".into(), |x| x.to_radians().cos())),
        ("tan",   Function::token_from_fn("tan".into(), |x| x.to_radians().tan())),
        ("csc",   Function::token_from_fn("csc".into(), |x| x.to_radians().sin().recip())),
        ("sec",   Function::token_from_fn("sec".into(), |x| x.to_radians().cos().recip())),
        ("cot",   Function::token_from_fn("cot".into(), |x| x.to_radians().tan().recip())),
        ("sinh",  Function::token_from_fn("sinh".into(), |x| x.sinh())),
        ("cosh",  Function::token_from_fn("cosh".into(), |x| x.cosh())),
        ("tanh",  Function::token_from_fn("tanh".into(), |x| x.tanh())),
        ("ln",    Function::token_from_fn("ln".into(), |x| x.ln())),
        ("log",   Function::token_from_fn("log".into(), |x| x.log10())),
        ("sqrt",  Function::token_from_fn("sqrt".into(), |x| x.sqrt())),
        ("ceil",  Function::token_from_fn("ceil".into(), |x| x.ceil())),
        ("floor", Function::token_from_fn("floor".into(), |x| x.floor())),
        // single arg functions can be added here
    ].iter().cloned().collect();
}

fn get_operators() -> HashMap<char, Token> {
    return [
        ('+', Operator::token_from_op('+', |x, y| x + y, 2, true)),
        ('-', Operator::token_from_op('-', |x, y| x - y, 2, true)),
        ('*', Operator::token_from_op('*', |x, y| x * y, 3, true)),
        ('/', Operator::token_from_op('/', |x, y| x / y, 3, true)),
        ('^', Operator::token_from_op('^', |x, y| x.powf(y) , 4, false)),
    ].iter().cloned().collect();
}

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let functions: HashMap<&str, Token> = get_functions();
    let operators: HashMap<char, Token> = get_operators();

    let mut num_vec: String = String::new();
    let mut char_vec: String = String::new();
    let mut result: Vec<Token> = vec![];
    let mut last_char_is_op = true;

    for letter in input.chars() {
        match letter {
            '0'...'9' | '.' => {
                num_vec.push(letter);
                last_char_is_op = false; // 5 - 6
                                         //   ^---- binary
            },
            'a'...'z' | 'A'...'Z' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(operators.get(&'*').unwrap().clone());
                    num_vec.clear();
                }
                char_vec.push(letter);
                last_char_is_op = false; // + or - can never be encountered right after a character
            },
            '+' | '-' => {
                let op_token = operators.get(&letter).unwrap().clone();
                let parse_num = num_vec.parse::<f64>().ok();
                if !last_char_is_op {
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        num_vec.clear();
                        last_char_is_op = true;
                    }
                    result.push(op_token);
                } else if last_char_is_op { // act as unary only if an operator was parsed previously
                    result.push(Token::LParen);
                    result.push(Token::Num((letter.to_string() + "1").parse::<f64>().unwrap()));
                    result.push(Token::RParen);
                    result.push(Operator::token_from_op('*', |x, y| x * y, 10, true));
                }
            },
            '/' | '*' | '^' => {
                drain_num_stack(&mut num_vec, &mut result);
                let operator_token: Token = operators.get(&letter).unwrap().clone();
                result.push(operator_token);
                last_char_is_op = true; // 5 / -5
                                        //     ^---- unary
                                        // TODO: parse right associative followed by unary properly
                                        // 2^+5 is parsed as 2^1*5 = 10
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
                last_char_is_op = true; // unary + or - if a lparen was encountered
                                        // (-5 + 6) or (+6 + 7)
                                        //  ^           ^
                                        //   `-----------`----unary
            },
            ')' => {
                drain_num_stack(&mut num_vec, &mut result);
                result.push(Token::RParen);
                last_char_is_op = false; // binary + or - if rparen was encountered 
                                         // (1 + 2) - (3 + 4)
                                         //         ^ binary minus
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
