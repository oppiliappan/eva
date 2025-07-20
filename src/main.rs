/*
 *  eva - an easy to use calculator REPL similar to bc(1)
 *  Copyright (C) 2019  Akshay Oppiliappan <nerdypepper@tuta.io>
 */
use clap::builder::{EnumValueParser, RangedU64ValueParser};
use clap::{Arg, Command};
use directories::{ProjectDirs, UserDirs};
use eva::lex::{AngleUnit, FunctionContext};
use eva::{eval_expr};
use once_cell::sync::Lazy;
use rustyline::error::ReadlineError;
use std::fs::create_dir_all;
use std::path::PathBuf;

mod fmt;
mod readline;

#[derive(Clone, Copy, Default)]
struct ClapAngleUnit(AngleUnit);

impl clap::ValueEnum for ClapAngleUnit {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ClapAngleUnit(AngleUnit::Degree),
            ClapAngleUnit(AngleUnit::Radian),
            ClapAngleUnit(AngleUnit::Gradian),
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            AngleUnit::Degree => Some(clap::builder::PossibleValue::new("degree")),
            AngleUnit::Radian => Some(clap::builder::PossibleValue::new("radian")),
            AngleUnit::Gradian => Some(clap::builder::PossibleValue::new("gradian")),
        }
    }
}

static CONFIGURATION: Lazy<Configuration> = Lazy::new(parse_arguments);

struct Configuration {
    angle_unit: AngleUnit,
    fix: usize,
    base: u8,
    input: String,
}

fn main() {
    let ctx = FunctionContext {
        angle_unit: CONFIGURATION.angle_unit,
    };

    if !CONFIGURATION.input.is_empty() {
        // command mode //
        let evaled = eval_expr(&ctx, CONFIGURATION.fix, &CONFIGURATION.input[..], Some(0.));
        match evaled {
            Ok(ans) => fmt::pprint(CONFIGURATION.base, CONFIGURATION.fix, ans),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
    } else {
        // REPL mode //
        // create fancy readline
        let mut rl = readline::create_readline(ctx.clone(), CONFIGURATION.fix);

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
                    let _ = rl.add_history_entry(line.as_str());
                    let evaled = eval_expr(&ctx, CONFIGURATION.fix, &line[..], prev_ans);
                    match evaled {
                        Ok(ans) => {
                            use std::fs::OpenOptions;
                            use std::io::Write;
                            prev_ans = Some(ans);
                            fmt::pprint(CONFIGURATION.base, CONFIGURATION.fix, ans);
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
                        Err(e) => println!("{}", e),
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

fn cmd() -> Command {
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
            Arg::new("angle_unit")
                .short('a')
                .long("angle_unit")
                .default_value("degree")
                .value_parser(EnumValueParser::<ClapAngleUnit>::new())
                .help("Angle unit"),
        )
}

fn parse_arguments() -> Configuration {
    let matches = cmd().get_matches();

    Configuration {
        angle_unit: matches.get_one::<ClapAngleUnit>("angle_unit").unwrap().0,
        fix: *matches.get_one("fix").unwrap(),
        base: *matches.get_one("base").unwrap(),
        input: matches.get_one("input").cloned().unwrap_or_default(),
    }
}

#[test]
fn verify_app() {
    cmd().debug_assert();
}
