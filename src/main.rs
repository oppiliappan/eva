use std::io::{ stdin };
use std::f64;

mod lex;
use crate::lex::*;

mod parse;
use crate::parse::*;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let input = input.replace(" ", "");

        if input == "exit" {
            return
        }

        let lexed = lexer(&input[..]);
        let postfixed = to_postfix(lexed.unwrap());
        let evaled = eval_postfix(postfixed.unwrap());
        println!("ans: {}", evaled.unwrap());
        println!();
    }
}


