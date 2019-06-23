/*
 *  eva - an easy to use calculator REPL similar to bc(1)
 *  Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 *
 */

// std
use std::f64;
use std::borrow::Cow::{self,Owned};
use std::path::PathBuf;

// modules
mod lex;
use crate::lex::*;
mod parse;
use crate::parse::*;
mod error;
use crate::error::{ CalcError, handler };
mod format;
use crate::format::*;

// extern crates
use rustyline::error::ReadlineError;
use rustyline::{ Editor, Context, Helper };
use rustyline::config::{ Builder, ColorMode, EditMode, CompletionType };
use rustyline::hint::Hinter;
use rustyline::completion::{ FilenameCompleter, Completer, Pair };
use rustyline::highlight::Highlighter;

use clap::{Arg, App};
use lazy_static::lazy_static;

struct RLHelper {
    completer: FilenameCompleter,
    highlighter: LineHighlighter,
    hinter: AnswerHinter,
}

struct AnswerHinter { }
impl Hinter for AnswerHinter {
    fn hint(&self, line: &str, _: usize, _: &Context) -> Option<String> {
        let input = line.trim();
        let input = input.replace(" ", "");
        if input.len() == 0 {
            return Some("".into())
        }
        let dry_run = eval_math_expression(&input);
        match dry_run {
            Ok(ans) =>  return Some(format!(" = {}", ans)),
            Err(_) => return Some(format!(""))
        };
    }
}

struct LineHighlighter { }
impl Highlighter for LineHighlighter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }
    fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
        let op = eval_math_expression(line);
        match op {
            Ok(_) => Owned(line.into()),
            Err(_) => Owned(format!("\x1b[31m{}\x1b[0m", line))
        }
    }
}

impl Highlighter for RLHelper { 
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        self.highlighter.highlight_hint(hint)
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
}

impl Completer for RLHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
        ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}
 
impl Hinter for RLHelper {
    fn hint(&self, line: &str, a: usize, b: &Context) -> Option<String> {
        self.hinter.hint(line, a, b)
    }
}

impl Helper for RLHelper {}

struct Configuration {
    radian_mode: bool,
    fix: usize,
    base: usize,
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
            .completion_type(CompletionType::Circular)
            .max_history_size(1000)
            .build();
        let mut rl = Editor::with_config(config);
        let h = RLHelper {
            completer: FilenameCompleter::new(),
            highlighter: LineHighlighter {},
            hinter: AnswerHinter {}
        };
        rl.set_helper(Some(h));
        let mut history_path = PathBuf::new();
        match dirs::home_dir() {
            Some(p) => {
                history_path = p;
                history_path.push("history.txt");
            },
            None => history_path.set_file_name("history.txt"),
        };
        if rl.load_history(history_path.as_path()).is_err() {
            println!("No previous history.")
        };

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
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


fn eval_math_expression(input: &str) -> Result<f64, CalcError> {
    let input = input.trim();
    let input = input.replace(" ", "");
    if input.len() == 0 {
        return Ok(0.)
    }
    let input        = format::autobalance_parens(&input[..])?;
    let lexed        = lexer(&input[..])?;
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
