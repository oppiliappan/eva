/*
 *  eva - an easy to use calculator REPL similar to bc(1)
 *  Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 */

/* imports */
// std
use std::fs::create_dir_all;
use std::path::PathBuf;

// modules
mod error;
mod fmt;
mod lex;
mod parse;
mod readline;
use crate::error::{handler, CalcError};
use crate::lex::*;
use crate::parse::*;
use crate::readline::*;

// extern crates
use clap::builder::RangedU64ValueParser;
use clap::{Arg, ArgAction, Command};
use directories::{ProjectDirs, UserDirs};
use once_cell::sync::Lazy;
use rustyline::error::ReadlineError;

/* end of imports */

pub struct Configuration {
    radian_mode: bool,
    fix: usize,
    base: u8,
    input: String,
}

#[cfg(not(test))]
static CONFIGURATION: Lazy<Configuration> = Lazy::new(parse_arguments);

#[cfg(test)]
static CONFIGURATION: Lazy<Configuration> = Lazy::new(|| Configuration {
    radian_mode: false,
    fix: 10,
    base: 10,
    input: "".to_string(),
});

fn main() {
    if !CONFIGURATION.input.is_empty() {
        // command mode //
        let evaled = eval_math_expression(&CONFIGURATION.input[..], Some(0.));
        match evaled {
            Ok(ans) => fmt::pprint(ans),
            Err(e) => {
                eprintln!("{}", handler(e));
                std::process::exit(1);
            }
        };
    } else {
        // REPL mode //
        // create fancy readline
        let mut rl = create_readline();

        // previous answer
        let mut prev_ans = None;

        // handle history storage
        let eva_dirs = ProjectDirs::from("com", "NerdyPepper", "eva").unwrap();
        let eva_data_dir = eva_dirs.data_dir();
        let eva_cache_dir = eva_dirs.cache_dir();
        let mut history_path = PathBuf::from(eva_data_dir);
        let mut previous_ans_path = PathBuf::from(eva_cache_dir);

        if create_dir_all(eva_data_dir).is_err() {
            history_path = PathBuf::from(UserDirs::new().unwrap().home_dir());
        }
        if create_dir_all(eva_cache_dir).is_err() {
            previous_ans_path = PathBuf::from(UserDirs::new().unwrap().home_dir());
        }
        history_path.push("history.txt");
        previous_ans_path.push("previous_ans.txt");

        if let Err(err) = std::fs::write(&previous_ans_path, "0") {
            println!("Could not write to previous_ans_path");
            println!("{:?}", err);
            std::process::exit(1);
        }

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
                            use std::fs::OpenOptions;
                            use std::io::Write;
                            prev_ans = Some(ans);
                            fmt::pprint(ans);
                            match OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(&previous_ans_path)
                            {
                                Ok(mut file) => {
                                    if let Err(err) = writeln!(file, "{}", ans) {
                                        println!(
                                            "Error while writing previous answer to file: {}",
                                            err
                                        )
                                    }
                                }
                                Err(err) => {
                                    println!("Error while writing previous answer to file: {}", err)
                                }
                            }
                        }
                        Err(e) => println!("{}", handler(e)),
                    };
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history(history_path.as_path()).unwrap();
    }
}

fn cmd() -> Command<'static> {
    clap::command!()
        .arg(
            Arg::new("input")
                .value_name("INPUT")
                .help("Optional expression string to run eva in command mode"),
        )
        .arg(
            Arg::new("fix")
                .short('f')
                .long("fix")
                .value_parser(RangedU64ValueParser::<usize>::new().range(1..=64))
                .default_value("10")
                .value_name("FIX")
                .help("Number of decimal places in output (1 - 64)"),
        )
        .arg(
            Arg::new("base")
                .short('b')
                .long("base")
                .value_parser(RangedU64ValueParser::<u8>::new().range(1..=36))
                .default_value("10")
                .value_name("RADIX")
                .help("Radix of calculation output (1 - 36)"),
        )
        .arg(
            Arg::new("radian")
                .short('r')
                .long("radian")
                .action(ArgAction::SetTrue)
                .help("Use radian mode"),
        )
}

pub fn parse_arguments() -> Configuration {
    let matches = cmd().get_matches();

    Configuration {
        radian_mode: *matches.get_one("radian").unwrap(),
        fix: *matches.get_one("fix").unwrap(),
        base: *matches.get_one("base").unwrap(),
        input: matches.get_one("input").cloned().unwrap_or_default(),
    }
}

pub fn eval_math_expression(input: &str, prev_ans: Option<f64>) -> Result<f64, CalcError> {
    let input = input.trim().replace(' ', "");
    if input == "help" {
        return Err(CalcError::Help);
    }
    if input.is_empty() {
        return Ok(0.);
    }
    let input = fmt::autobalance_parens(&input[..])?;
    let lexed = lexer(&input[..], prev_ans)?;
    let postfixed = to_postfix(lexed)?;
    let evaled = eval_postfix(postfixed)?;
    let evaled_fixed = format!("{:.*}", CONFIGURATION.fix, evaled)
        .parse::<f64>()
        .unwrap();
    Ok(evaled_fixed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Math;

    pub fn eval(input: &str, prev_ans: Option<f64>) -> Result<f64, CalcError> {
        let ans = eval_math_expression(input, prev_ans)?;
        Ok(format!("{:.*}", CONFIGURATION.fix, ans).parse().unwrap())
    }

    #[test]
    fn verify_app() {
        cmd().debug_assert();
    }
    #[test]
    fn basic_ops() {
        let evaled = eval("6*2 + 3 + 12 -3", Some(0f64));
        assert_eq!(evaled, Ok(24.));
    }
    #[test]
    fn trignometric_fns() {
        let evaled = eval("sin(30) + tan(45", Some(0f64));
        assert_eq!(evaled, Ok(1.5));
    }
    #[test]
    fn brackets() {
        let evaled = eval("(((1 + 2 + 3) ^ 2 ) - 4)", Some(0f64));
        assert_eq!(evaled, Ok(32.));
    }
    #[test]
    fn exponentiation() {
        let evaled = eval("2 ** 2 ** 3", None);
        assert_eq!(evaled, Ok(256.)); // 2^(2^3), not (2^2)^3
    }
    #[test]
    fn floating_ops() {
        let evaled = eval("1.2816 + 1 + 1.2816/1.2", Some(0f64));
        assert_eq!(evaled, Ok(3.3496));
    }
    #[test]
    fn inverse_trignometric_fns() {
        let evaled = eval("deg(asin(1) + acos(1))", Some(0f64));
        assert_eq!(evaled, Ok(90.));
    }
    #[test]
    fn sigmoid_fns() {
        let evaled = eval("1 / (1 + e^-7)", Some(0f64));
        assert_eq!(evaled, Ok(0.9990889488));
    }
    #[test]
    fn prev_ans() {
        let evaled = eval("_ + 9", Some(9f64));
        assert_eq!(evaled, Ok(18.0));
    }
    #[test]
    fn eval_with_zero_prev() {
        let evaled = eval("9 + _ ", Some(0f64));
        assert_eq!(evaled, Ok(9.));
    }
    #[test]
    fn eval_const_multiplication() {
        let evaled = eval("e2", None);
        assert_eq!(evaled, Ok(5.4365636569));
    }
    #[test]
    fn eval_round() {
        let evaled = eval("round(0.5)+round(2.4)", None);
        assert_eq!(evaled, Ok(3.));
    }
    #[test]
    fn eval_exp2() {
        let evaled = eval("exp2(8)", None);
        assert_eq!(evaled, Ok(256.));
    }
    #[test]
    fn eval_exp() {
        let evaled = eval("exp(3)", None);
        assert_eq!(evaled, Ok(20.0855369232));
    }
    #[test]
    fn eval_e_times_n() {
        let evaled = eval("e0", None);
        assert_eq!(evaled, Ok(0.));
    }
    #[test]
    fn eval_factorial_large() {
        let evaled = eval("21!", None);
        assert_eq!(evaled, Ok(51_090_942_171_709_440_000.0));
    }
    #[test]
    fn eval_nroot() {
        let evaled = eval("nroot(27, 3)", None);
        assert_eq!(evaled, Ok(3.));
    }
    #[test]
    fn eval_log_n_base() {
        let evaled = eval("log(2^16,4)", None);
        assert_eq!(evaled, Ok(8.));
    }
    #[test]
    fn eval_log_n_brackets() {
        let evaled = eval("log(1+(2^16),4)", None);
        assert_eq!(evaled, Ok(8.0000110068));
    }
    #[test]
    fn eval_mismatched_parens_in_multiarg_fn() {
        let evaled = eval("log(1+(2^16, 4)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Syntax("Mismatched parentheses!".to_string()))
        );
    }
    #[test]
    fn eval_comma_without_multiarg_fn() {
        let evaled = eval("1+(2^16, 4)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Syntax("Mismatched parentheses!".to_string()))
        );
    }
    #[test]
    fn eval_unexpected_comma() {
        let evaled = eval("(1+1,2+2)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Parser(
                "Too many operators, too few operands".to_string()
            ))
        );
    }
    #[test]
    fn eval_nroot_expr_on_both_sides() {
        let evaled = eval("nroot(2+2,4+e^2)", None);
        assert_eq!(evaled, Ok(1.1294396449));
    }
    #[test]
    fn eval_comma_left_paren_mixup() {
        let evaled = eval("exp 2,3)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Syntax("Mismatched parentheses!".to_string()))
        );
        let evaled = eval("exp,2,3)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Syntax("Mismatched parentheses!".to_string()))
        );
    }
    #[test]
    fn eval_log10() {
        let evaled = eval("log10(1000)", None);
        assert_eq!(evaled, Ok(3.));
    }
    #[test]
    fn eval_empty_argument() {
        let evaled = eval("log(2,,3)", None);
        assert_eq!(evaled, Err(CalcError::Syntax("Empty argument".to_string())));
    }
    #[test]
    fn eval_mismatched_args() {
        let evaled = eval("nroot(23,3,4)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Parser(
                "Too many operators, too few operands".to_string()
            ))
        );
        let evaled = eval("nroot(23)", None);
        assert_eq!(
            evaled,
            Err(CalcError::Parser(
                "To few arguments for function, need 2".to_string()
            ))
        );
    }
    #[test]
    fn eval_negative_factorial() {
        let evaled = eval_math_expression("-1!", None);
        assert_eq!(Err(CalcError::Math(Math::OutOfBounds)), evaled);
    }
}
