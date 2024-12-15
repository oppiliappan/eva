/* Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 * Refer to LICENCE for more information.
 * */

use crate::error::CalcError;
use crate::lex::{FunctionContext, Token};

pub fn to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut postfixed: Vec<Token> = vec![];
    let mut op_stack: Vec<Token> = vec![];
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            Token::Num(_) => {
                postfixed.push(token);
            }
            Token::Function(_) => {
                op_stack.push(token);
            }
            Token::Operator(current_op) => {
                while let Some(top_op) = op_stack.last() {
                    match top_op {
                        Token::LParen | Token::Comma => {
                            break;
                        }
                        Token::Operator(x) => {
                            let tp = x.precedence;
                            let cp = current_op.precedence;
                            if tp > cp || (tp == cp && x.is_left_associative) {
                                postfixed.push(op_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        Token::Function(_) => {
                            postfixed.push(op_stack.pop().unwrap());
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                op_stack.push(token);
            }
            Token::LParen => {
                op_stack.push(token);
            }
            Token::RParen | Token::Comma => {
                let mut push_until_paren: bool = false;
                while let Some(token) = op_stack.pop() {
                    if matches!(op_stack.last(), Some(Token::Function(_)) | None)
                        && token == Token::Comma
                        || token == Token::LParen
                    {
                        push_until_paren = true;
                        break;
                    }
                    postfixed.push(token);
                }
                if !push_until_paren {
                    return Err(CalcError::Syntax("Mismatched parentheses!".into()));
                }
                if token == Token::Comma {
                    if tokens.peek() == Some(&Token::Comma) {
                        return Err(CalcError::Syntax("Empty argument".into()));
                    }
                    op_stack.push(token);
                }
            }
        }
    }
    while let Some(op) = op_stack.pop() {
        postfixed.push(op);
    }
    // println!("{:?}", postfixed);
    Ok(postfixed)
}

pub fn eval_postfix(ctx: &FunctionContext, postfixed: Vec<Token>) -> Result<f64, CalcError> {
    let mut num_stack: Vec<f64> = vec![];
    let mut args = vec![];
    for token in postfixed {
        match token {
            Token::Num(n) => {
                num_stack.push(n);
            }
            Token::Operator(op) => {
                if let Some(n2) = num_stack.pop() {
                    if let Some(n1) = num_stack.pop() {
                        num_stack.push(op.operate(n1, n2)?);
                    } else {
                        return Err(CalcError::Parser(
                            "Too many operators, too few operands".to_string(),
                        ));
                    }
                } else {
                    return Err(CalcError::Parser(
                        "Too many operators, too few operands".to_string(),
                    ));
                }
            }
            Token::Function(func) => {
                let arity = func.arity();
                for _ in 0..arity {
                    if let Some(arg) = num_stack.pop() {
                        args.insert(0, arg);
                    } else {
                        return Err(CalcError::Parser(format!(
                            "To few arguments for function, need {arity}"
                        )));
                    }
                }
                num_stack.push(func.apply(ctx, &args)?);
                args.clear();
            }
            _ => unreachable!("wut"),
        }
    }
    if num_stack.len() == 1 {
        Ok(num_stack.pop().unwrap())
    } else {
        Err(CalcError::Parser(
            "Too many operators, too few operands".to_string(),
        ))
    }
}
