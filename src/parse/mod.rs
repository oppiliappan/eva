use crate::lex::Token;

pub fn to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut postfixed: Vec<Token> = vec![];
    let mut op_stack: Vec<Token> = vec![];
    for token in tokens {
        match token {
            Token::Num(_) => {
                postfixed.push(token);
            },
            Token::Function(_) => {
                op_stack.push(token);
            }
            Token::Operator(current_op) => {
                while let Some(top_op) = op_stack.last() {
                    match top_op {
                        Token::LParen => {
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
                            return Err(format!("{:?} must not be on operator stack", top_op))
                        }
                    }
                }
                op_stack.push(token);
            },
            Token::LParen => {
                op_stack.push(token);
            },
            Token::RParen => {
                let mut push_until_paren: bool = false;
                while let Some(token) = op_stack.pop() {
                    if token == Token::LParen {
                        push_until_paren = true;
                        break;
                    }
                    postfixed.push(token)
                }
                if !push_until_paren {
                    return Err(String::from("Mismatched ')'"));
                }
            }
        }
    }
    while let Some(op) = op_stack.pop() {
        postfixed.push(op);
    }
    Ok(postfixed)
}

pub fn eval_postfix(postfixed: Vec<Token>) -> Result<f64, String> {
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
            },
            Token::Function(funct) => {
                if let Some(arg) = num_stack.pop() {
                    num_stack.push(funct.apply(arg))
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
