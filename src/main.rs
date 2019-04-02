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

// std
use std::f64;

// modules
mod lex;
use crate::lex::*;
mod parse;
use crate::parse::*;
mod error;
use crate::error::{ CalcError, handler };

// extern crates
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::config::{ Builder, ColorMode, EditMode };
use clap::{Arg, App};
use lazy_static::lazy_static;

struct Configuration {
    radian_mode: bool,
    fix: usize,
    input: String
}

lazy_static! {
    static ref CONFIGURATION: Configuration =  parse_arguments();
}

fn main() {
    if CONFIGURATION.input.len() > 0 {
        let evaled = eval_math_expression(&CONFIGURATION.input[..]);
        match evaled {
            Ok(ans) => pprint(ans),
            Err(e) => {
                eprintln!("{}", handler(e));
                std::process::exit(1);
            },
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
                        Ok(ans) => pprint(ans),
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

fn pprint(ans: f64) {
    let ans_string = format!("{}",ans);

    let ans_vector: Vec<&str> = ans_string.split(".").collect();
    match ans_vector.len() {
        1 => println!("{}",thousand_sep(ans_vector[0])),
        2 => println!("{}.{}",thousand_sep(ans_vector[0]),ans_vector[1]),
        _ => ()
    }
}

fn thousand_sep(inp:&str) -> String{
    let mut result_string = String::new();
    for (i,c) in inp.to_string().chars().rev().enumerate(){
        if i % 3 == 0 && i != 0{
            result_string.push_str(",");
            result_string.push(c);
            continue
        }
        result_string.push(c)
    }
    let arrange:i16 = CONFIGURATION.fix as i16 - inp.len() as i16;

    if arrange > 0 {
        result_string.push_str(" ".repeat(arrange as usize).as_str())
    }
    result_string.chars().rev().collect::<String>()
}

fn parse_arguments() -> Configuration {
    let config = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("fix")
             .short("f")
             .long("fix")
             .takes_value(true)
             .value_name("FIX")
             .help("set number of decimal places in the output"))
        .arg(Arg::with_name("INPUT")
             .help("optional expression string to run eva in command mode")
             .index(1))
        .arg(Arg::with_name("radian")
             .short("r")
             .long("radian")
             .help("set eva to radian mode"))
        .get_matches();

    let mut input = String::new();
    if let Some(i) = config.value_of("INPUT") {
        input.push_str(i);
    };
    Configuration {
        radian_mode: config.is_present("radian"),
        fix: config.value_of("fix")
            .unwrap_or("10")
            .parse()
            .unwrap(),
        input,
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
    let input = input.trim();
    let input = input.replace(" ", "");
    if input.len() == 0 {
        return Ok(0.)
    }
    let input     = autobalance_parens(&input[..])?;
    let lexed     = lexer(&input[..])?;
    let postfixed = to_postfix(lexed)?;
    let evaled    = eval_postfix(postfixed)?;
    Ok(evaled)
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
    #[test]
    fn inverse_trignometric_fns() {
        let evaled = eval_math_expression("deg(asin(1) + acos(1))").unwrap();
        assert_eq!(90., evaled);
    }
}
