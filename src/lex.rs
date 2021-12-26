/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::error::{CalcError, Math};
use crate::CONFIGURATION;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    token: char,
    pub operation: fn(f64, f64) -> f64,
    pub precedence: u8,
    pub is_left_associative: bool,
}

impl Operator {
    fn token_from_op(
        token: char,
        operation: fn(f64, f64) -> f64,
        precedence: u8,
        is_left_associative: bool,
    ) -> Token {
        Token::Operator(Operator {
            token,
            operation,
            precedence,
            is_left_associative,
        })
    }
    pub fn operate(self, x: f64, y: f64) -> Result<f64, CalcError> {
        if self.token == '/' && y == 0. {
            Err(CalcError::Math(Math::DivideByZero))
        } else {
            Ok((self.operation)(x, y))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    token: String,
    relation: fn(f64) -> f64,
}

impl Function {
    fn token_from_fn(token: String, relation: fn(f64) -> f64) -> Token {
        Token::Function(Function { token, relation })
    }
    pub fn apply(self, arg: f64) -> Result<f64, CalcError> {
        let result = (self.relation)(arg);
        if !result.is_finite() {
            Err(CalcError::Math(Math::OutOfBounds))
        } else {
            Ok(result)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Num(f64),
    Function(Function),
    LParen,
    RParen,
}

lazy_static! {
    pub static ref CONSTANTS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("e",  Token::Num(std::f64::consts::E));
        m.insert("pi", Token::Num(std::f64::consts::PI));
        m
    };

    pub static ref FUNCTIONS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("sin",   Function::token_from_fn("sin".into(),   |x| is_radian_mode(x, CONFIGURATION.radian_mode).sin()));
        m.insert("cos",   Function::token_from_fn("cos".into(),   |x| is_radian_mode(x, CONFIGURATION.radian_mode).cos()));
        m.insert("tan",   Function::token_from_fn("tan".into(),   |x| is_radian_mode(x, CONFIGURATION.radian_mode).tan()));
        m.insert("csc",   Function::token_from_fn("csc".into(),   |x| is_radian_mode(x, CONFIGURATION.radian_mode).sin().recip()));
        m.insert("sec",   Function::token_from_fn("sec".into(),   |x| is_radian_mode(x, CONFIGURATION.radian_mode).cos().recip()));
        m.insert("cot",   Function::token_from_fn("cot".into(),   |x| is_radian_mode(x, CONFIGURATION.radian_mode).tan().recip()));
        m.insert("sinh",  Function::token_from_fn("sinh".into(),  |x| x.sinh()));
        m.insert("cosh",  Function::token_from_fn("cosh".into(),  |x| x.cosh()));
        m.insert("tanh",  Function::token_from_fn("tanh".into(),  |x| x.tanh()));
        m.insert("ln",    Function::token_from_fn("ln".into(),    |x| x.ln()));
        m.insert("log",   Function::token_from_fn("log".into(),   |x| x.log10()));
        m.insert("sqrt",  Function::token_from_fn("sqrt".into(),  |x| x.sqrt()));
        m.insert("ceil",  Function::token_from_fn("ceil".into(),  |x| x.ceil()));
        m.insert("floor", Function::token_from_fn("floor".into(), |x| x.floor()));
        m.insert("rad",   Function::token_from_fn("rad".into(),   |x| x.to_radians()));
        m.insert("deg",   Function::token_from_fn("deg".into(),   |x| x.to_degrees()));
        m.insert("abs",   Function::token_from_fn("abs".into(),   |x| x.abs()));
        m.insert("asin",  Function::token_from_fn("asin".into(),  |x| x.asin()));
        m.insert("acos",  Function::token_from_fn("acos".into(),  |x| x.acos()));
        m.insert("atan",  Function::token_from_fn("atan".into(),  |x| x.atan()));
        m.insert("acsc",  Function::token_from_fn("acsc".into(),  |x| (1./x).asin()));
        m.insert("asec",  Function::token_from_fn("asec".into(),  |x| (1./x).acos()));
        m.insert("acot",  Function::token_from_fn("acot".into(),  |x| (1./x).atan()));
        m.insert("exp",   Function::token_from_fn("exp".into(),   |x| x.exp()));
        // single arg function s can be added here
        m
    };

    pub static ref OPERATORS: HashMap<char, Token> = {
        let mut m = HashMap::new();
        m.insert('+', Operator::token_from_op('+', |x, y| x + y, 2, true));
        m.insert('-', Operator::token_from_op('-', |x, y| x - y, 2, true));
        m.insert('*', Operator::token_from_op('*', |x, y| x * y, 3, true));
        m.insert('/', Operator::token_from_op('/', |x, y| x / y, 3, true));
        m.insert('%', Operator::token_from_op('%', |x, y| x % y, 3, true));
        m.insert('^', Operator::token_from_op('^', |x, y| x.powf(y) , 4, false));
        m.insert('!', Operator::token_from_op('!', |x, _| factorial(x) , 4, true));
        m
    };
}

fn factorial(n: f64) -> f64 {
    n.signum() * (1..=n.abs() as u64).fold(1, |p, n| p * n) as f64
}

pub fn lexer(input: &str, prev_ans: Option<f64>) -> Result<Vec<Token>, CalcError> {
    let mut num_vec: String = String::new();
    let mut char_vec: String = String::new();
    let mut result: Vec<Token> = vec![];
    let mut last_char_is_op = true;

    let mut chars = input.chars().peekable();
    while let Some(mut letter) = chars.next() {
        match letter {
            '0'..='9' | '.' => {
                if !char_vec.is_empty() {
                    if FUNCTIONS.get(&char_vec[..]).is_some() {
                        return Err(CalcError::Syntax(format!(
                            "Function '{}' expected parentheses",
                            char_vec
                        )));
                    } else {
                        return Err(CalcError::Syntax(format!(
                            "Unexpected character '{}'",
                            char_vec
                        )));
                    }
                }
                num_vec.push(letter);
                last_char_is_op = false;
            }
            '_' => {
                if prev_ans.is_none() {
                    return Err(CalcError::Syntax("No previous answer!".into()));
                }
                if !char_vec.is_empty() {
                    if FUNCTIONS.get(&char_vec[..]).is_some() {
                        return Err(CalcError::Syntax(format!(
                            "Function '{}' expected parentheses",
                            char_vec
                        )));
                    } else {
                        return Err(CalcError::Syntax(format!(
                            "Unexpected character '{}'",
                            char_vec
                        )));
                    }
                }
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(OPERATORS.get(&'*').unwrap().clone());
                    num_vec.clear();
                }
                last_char_is_op = false;
                result.push(Token::Num(prev_ans.unwrap()));
            }
            'a'..='z' | 'A'..='Z' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(OPERATORS.get(&'*').unwrap().clone());
                    num_vec.clear();
                }
                char_vec.push(letter);
                last_char_is_op = false;
            }
            '+' | '-' => {
                let op_token = OPERATORS.get(&letter).unwrap().clone();
                let parse_num = num_vec.parse::<f64>().ok();
                if !last_char_is_op {
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        num_vec.clear();
                        last_char_is_op = true;
                    } else if let Some(token) = CONSTANTS.get(&char_vec[..]) {
                        result.push(token.clone());
                        char_vec.clear();
                        last_char_is_op = true;
                    }
                    result.push(op_token);
                } else if last_char_is_op {
                    result.push(Token::LParen);
                    result.push(Token::Num(
                        (letter.to_string() + "1").parse::<f64>().unwrap(),
                    ));
                    result.push(Token::RParen);
                    result.push(Operator::token_from_op('*', |x, y| x * y, 10, true));
                }
            }
            '/' | '*' | '%' | '^' | '!' => {
                drain_stack(&mut num_vec, &mut char_vec, &mut result);
                if letter == '*' && chars.peek() == Some(&'*') {
                    // Accept `**` operator as meaning `^` (exponentation).
                    let _ = chars.next();
                    letter = '^';
                }
                let operator_token: Token = OPERATORS.get(&letter).unwrap().clone();
                result.push(operator_token);
                last_char_is_op = true;
                if letter == '!' {
                    result.push(Token::Num(1.));
                    last_char_is_op = false;
                }
            }
            '(' => {
                if !char_vec.is_empty() {
                    if let Some(res) = FUNCTIONS.get(&char_vec[..]) {
                        result.push(res.clone());
                    } else {
                        return Err(CalcError::Syntax(format!(
                            "Unknown function '{}'",
                            char_vec
                        )));
                    }
                    char_vec.clear();
                } else {
                    let parse_num = num_vec.parse::<f64>().ok();
                    if let Some(x) = parse_num {
                        result.push(Token::Num(x));
                        result.push(OPERATORS.get(&'*').unwrap().clone());
                        num_vec.clear();
                    }
                }

                if let Some(Token::RParen) = result.last() {
                    result.push(OPERATORS.get(&'*').unwrap().clone());
                }
                result.push(Token::LParen);
                last_char_is_op = true;
            }
            ')' => {
                drain_stack(&mut num_vec, &mut char_vec, &mut result);
                result.push(Token::RParen);
                last_char_is_op = false;
            }
            ' ' => {}
            _ => return Err(CalcError::Syntax(format!("Unexpected token: '{}'", letter))),
        }
    }
    // println!("{:?}", result);
    drain_stack(&mut num_vec, &mut char_vec, &mut result);
    Ok(result)
}

fn drain_stack(num_vec: &mut String, char_vec: &mut String, result: &mut Vec<Token>) {
    let parse_num = num_vec.parse::<f64>().ok();
    if let Some(x) = parse_num {
        result.push(Token::Num(x));
        num_vec.clear();
    } else if let Some(token) = CONSTANTS.get(&char_vec[..]) {
        result.push(token.clone());
        char_vec.clear();
    }
}

fn is_radian_mode(x: f64, is_radian: bool) -> f64 {
    if is_radian {
        x
    } else {
        x.to_radians()
    }
}
