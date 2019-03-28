use std::f64;
use std::env;

mod lex;
use crate::lex::*;

mod parse;
use crate::parse::*;

mod error;
use crate::error::{ CalcError, handler };

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::config::{ Builder, ColorMode, EditMode };


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut expr = String::new();
        for arg in args[1..].iter() {
            expr.push_str(&arg[..]);
        }
        let evaled = eval_math_expression(&expr[..]);
        match evaled {
            Ok(ans) => println!("{}", ans),
            Err(e) => handler(e),
        };
    } else {
        let config_builder = Builder::new();
        let config = config_builder.color_mode(ColorMode::Enabled)
            .edit_mode(EditMode::Emacs)
            .history_ignore_space(true)
            .max_history_size(1000)
            .build();
        let mut rl = Editor::<()>::with_config(config);
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_ref());
                    let evaled = eval_math_expression(&line[..]);
                    match evaled {
                        Ok(ans) => println!("{}", ans),
                        Err(e) => handler(e),
                    };
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        rl.save_history("history.txt").unwrap();
    }
}

fn autobalance_parens(input: &str) -> Result<String, CalcError> {
    let mut balanced = String::from(input);
    let mut left_parens = 0;
    let mut right_parens = 0;
    for letter in input.chars() {
        if letter == '(' {
            left_parens += 1;
        } else if letter == ')' {
            right_parens += 1;
        }
    }

    if left_parens > right_parens {
        let extras = ")".repeat(left_parens - right_parens);
        balanced.push_str(&extras[..]);
        Ok(balanced)
    } else if left_parens < right_parens {
        return Err(CalcError::Syntax("Mismatched parentheses!".into()))
    } else {
        Ok(balanced)
    }
}

fn eval_math_expression(input: &str) -> Result<f64, CalcError> {
    let input     = autobalance_parens(&input[..])?;
    let lexed     = lexer(&input[..])?;
    let postfixed = to_postfix(lexed)?;
    let evaled    = eval_postfix(postfixed)?;
    Ok(evaled)
}

