/*
    eva - an easy to use calculator REPL similar to bc(1)
    Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

 */

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
            Err(e) => eprintln!("{}", handler(e)),
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
                        Err(e) => println!("{}", handler(e)),
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
    Ok(format!("{:.*}", 5, evaled).parse::<f64>().unwrap())
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let evaled = eval_math_expression("6*2 + 3 + 12 -3").unwrap();
        assert_eq!(24., evaled);
    }
    #[test]
    fn trignometric_fns() {
        let evaled = eval_math_expression("sin(30) + tan(45").unwrap();
        assert_eq!(1.5, evaled);
    }
    #[test]
    fn brackets() {
        let evaled = eval_math_expression("(((1 + 2 + 3) ^ 2 ) - 4)").unwrap();
        assert_eq!(32., evaled);
    }
    #[test]
    fn floating_ops() {
        let evaled = eval_math_expression("1.2816 + 1 + 1.2816/1.2").unwrap();
        assert_eq!(3.3496, evaled);
    }
}
