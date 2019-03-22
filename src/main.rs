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
        let input = autobalance_parens(&input[..]).unwrap();

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

fn autobalance_parens(input: &str) -> Result<String, String> {
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
        return Err(format!("Mismatched parentheses"))
    } else {
        Ok(balanced)
    }
}
