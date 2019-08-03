/*
 *  eva - an easy to use calculator REPL similar to bc(1)
 *  Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 */

/* imports */
// std
use std::f64;
use std::path::PathBuf;
use std::fs::create_dir_all;

// modules
mod lex;
mod parse;
mod error;
mod format;
mod readline;
use crate::lex::*;
use crate::parse::*;
use crate::error::{ CalcError, handler };
use crate::format::*;
use crate::readline::*;

// extern crates
use rustyline::error::ReadlineError;
use clap::{Arg, App};
use lazy_static::lazy_static;
use directories::{ ProjectDirs, UserDirs };

/* end of imports */

struct Configuration {
    radian_mode: bool,
    fix: usize,
    base: usize,
    input: String
}

lazy_static! {
    static ref CONFIGURATION: Configuration = parse_arguments();
}

fn main() {
    if CONFIGURATION.input.len() > 0 {
        // command mode //
        let evaled = eval_math_expression(&CONFIGURATION.input[..], 0f64);
        match evaled {
            Ok(ans) => pprint(ans),
            Err(e) => {
                eprintln!("{}", handler(e));
                std::process::exit(1);
            },
        };
    } else {
        // REPL mode //
        // create fancy readline 
        let mut rl = create_readline();

        // previous answer
        let mut prev_ans = 0f64;

        // handle history storage
        let eva_dirs = ProjectDirs::from("com", "NerdyPepper", "eva").unwrap();
        let eva_data_dir = eva_dirs.data_dir();
        let mut history_path = PathBuf::from(eva_data_dir);
        match create_dir_all(eva_data_dir) {
            Ok(_) => history_path.push("history.txt"),
           Err(_) => history_path = PathBuf::from(UserDirs::new().unwrap().home_dir()),
        };

        if rl.load_history(history_path.as_path()).is_err() {
            println!("No previous history.")
        };

        // repl loop begins here
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let evaled = eval_math_expression(&line[..], prev_ans);
                    match evaled {
                        Ok(ans) => {
                            prev_ans = ans;
                            pprint(ans);
                        }
                        Err(e) => println!("{}", handler(e)),
                    };
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        rl.save_history(history_path.as_path()).unwrap();
    }
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
        .arg(Arg::with_name("base")
             .short("b")
             .long("base")
             .takes_value(true)
             .value_name("RADIX")
             .help("set the radix of calculation output (1 - 36)"))
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
            base: config.value_of("base")
                .unwrap_or("10")
                .parse()
                .unwrap(),
                input,
    }
}

pub fn eval_math_expression(input: &str, prev_ans: f64) -> Result<f64, CalcError> {
    let input = input.trim();
    let input = input.replace(" ", "");
    if input.len() == 0 {
        return Ok(0.)
    }
    let input        = format::autobalance_parens(&input[..])?;
    let lexed        = lexer(&input[..], prev_ans)?;
    let postfixed    = to_postfix(lexed)?;
    let evaled       = eval_postfix(postfixed)?;
    let evaled_fixed = format!("{:.*}", CONFIGURATION.fix, evaled).parse::<f64>().unwrap();
    Ok(evaled_fixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let evaled = eval_math_expression("6*2 + 3 + 12 -3", 0f64).unwrap();
        assert_eq!(24., evaled);
    }
    #[test]
    fn trignometric_fns() {
        let evaled = eval_math_expression("sin(30) + tan(45", 0f64).unwrap();
        assert_eq!(1.5, evaled);
    }
    #[test]
    fn brackets() {
        let evaled = eval_math_expression("(((1 + 2 + 3) ^ 2 ) - 4)", 0f64).unwrap();
        assert_eq!(32., evaled);
    }
    #[test]
    fn floating_ops() {
        let evaled = eval_math_expression("1.2816 + 1 + 1.2816/1.2", 0f64).unwrap();
        assert_eq!(3.3496, evaled);
    }
    #[test]
    fn inverse_trignometric_fns() {
        let evaled = eval_math_expression("deg(asin(1) + acos(1))", 0f64).unwrap();
        assert_eq!(90., evaled);
    }
    #[test]
    fn prev_ans() {
        let evaled = eval_math_expression("_ + 9", 9f64).unwrap();
        assert_eq!(18.0, evaled);
    }
}
