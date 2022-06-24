/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use once_cell::sync::Lazy;
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
    token: &'static str,
    relation: fn(f64) -> f64,
}

impl Function {
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

impl Token {
    fn from_op(
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
}

pub static CONSTANTS: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("e", Token::Num(std::f64::consts::E));
    m.insert("pi", Token::Num(std::f64::consts::PI));
    m
});

pub static FUNCTIONS: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
    fn add_fn(map: &mut HashMap<&str, Token>, token: &'static str, relation: fn(f64) -> f64) {
        let func = Token::Function(Function { token, relation });
        map.insert(token, func);
    }
    let mut m = HashMap::new();
    add_fn(&mut m, "sin", |x| rad(x).sin());
    add_fn(&mut m, "cos", |x| rad(x).cos());
    add_fn(&mut m, "tan", |x| rad(x).tan());
    add_fn(&mut m, "csc", |x| rad(x).sin().recip());
    add_fn(&mut m, "sec", |x| rad(x).cos().recip());
    add_fn(&mut m, "cot", |x| rad(x).tan().recip());
    add_fn(&mut m, "sinh", |x| x.sinh());
    add_fn(&mut m, "cosh", |x| x.cosh());
    add_fn(&mut m, "tanh", |x| x.tanh());
    add_fn(&mut m, "ln", |x| x.ln());
    add_fn(&mut m, "log", |x| x.log10());
    add_fn(&mut m, "sqrt", |x| x.sqrt());
    add_fn(&mut m, "ceil", |x| x.ceil());
    add_fn(&mut m, "floor", |x| x.floor());
    add_fn(&mut m, "rad", |x| x.to_radians());
    add_fn(&mut m, "deg", |x| x.to_degrees());
    add_fn(&mut m, "abs", |x| x.abs());
    add_fn(&mut m, "asin", |x| x.asin());
    add_fn(&mut m, "acos", |x| x.acos());
    add_fn(&mut m, "atan", |x| x.atan());
    add_fn(&mut m, "acsc", |x| (1. / x).asin());
    add_fn(&mut m, "asec", |x| (1. / x).acos());
    add_fn(&mut m, "acot", |x| (1. / x).atan());
    add_fn(&mut m, "exp", |x| x.exp());
    add_fn(&mut m, "exp2", |x| x.exp2());
    add_fn(&mut m, "round", |x| x.round());
    // single arg function s can be added here
    m
});

pub static OPERATORS: Lazy<HashMap<char, Token>> = Lazy::new(|| {
    fn add_op(
        map: &mut HashMap<char, Token>,
        token: char,
        operation: fn(f64, f64) -> f64,
        precedence: u8,
        is_left_associative: bool,
    ) {
        let op = Token::from_op(token, operation, precedence, is_left_associative);
        map.insert(token, op);
    }
    let mut m = HashMap::new();
    add_op(&mut m, '+', |x, y| x + y, 2, true);
    add_op(&mut m, '-', |x, y| x - y, 2, true);
    add_op(&mut m, '*', |x, y| x * y, 3, true);
    add_op(&mut m, '/', |x, y| x / y, 3, true);
    add_op(&mut m, '%', |x, y| x % y, 3, true);
    add_op(&mut m, '^', |x, y| x.powf(y), 4, false);
    add_op(&mut m, '!', |x, _| factorial(x), 4, true);
    m
});

fn factorial(n: f64) -> f64 {
    n.signum() * (1..=n.abs() as u64).product::<u64>() as f64
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
                        char_vec.push(letter);
                        if FUNCTIONS.get(&char_vec[..]).is_none() {
                            return Err(CalcError::Syntax(format!(
                                "Function '{}' expected parentheses",
                                &char_vec[..char_vec.chars().count() - 1]
                            )));
                        }
                    } else if CONSTANTS.get(&char_vec[..]).is_some() {
                        result.push(CONSTANTS.get(&char_vec[..]).unwrap().clone());
                        result.push(OPERATORS.get(&'*').unwrap().clone());
                        char_vec.clear();
                        num_vec.push(letter);
                        last_char_is_op = false;
                    } else {
                        return Err(CalcError::Syntax(format!(
                            "Unexpected character '{}'",
                            char_vec
                        )));
                    }
                } else {
                    num_vec.push(letter);
                    last_char_is_op = false;
                }
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
                    result.push(Token::from_op('*', |x, y| x * y, 10, true));
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

/// Convert to radian if radian_mode is enabled.
fn rad(x: f64) -> f64 {
    if CONFIGURATION.radian_mode {
        x
    } else {
        x.to_radians()
    }
}
