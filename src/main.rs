use std::str;

#[derive(Debug)]
struct Token {
    kind: String,
    val: String,
}

fn main() {
    let input: &str = "2y + 11 ^ x";
    let input = input.replace(" ", "");
    println!("{}", input);

    let mut num_vec: String = String::new();
    let mut char_vec: String = String::new();

    let mut result: Vec<Token> = vec![];

    for letter in input.chars() {
        if letter.is_digit(10) {
            num_vec.push(letter);
        } else if letter == '.' {
            num_vec.push(letter);
        } else if letter.is_alphabetic() {
            let literal = buffer_to_token("Literal", &num_vec[..]);
            if let Some(x) = literal {
                result.push(x);
            }
            char_vec.push(letter);
            num_vec.clear();
        } else if is_operator(letter) {
            let literal = buffer_to_token("Literal", &num_vec[..]);
            let variable = buffer_to_token("Variable", &char_vec[..]);
            if let Some(x) = literal {
                result.push(x);
            }
            if let Some(x) = variable {
                result.push(x);
            }
            num_vec.clear();
            char_vec.clear();
            result.push(Token { kind: "Operator".into(), val: letter.to_string() });
        }
        println!("{}", letter);
        for token in &result {
            println!("{:?}", token);
        }
    }
}

fn buffer_to_token(k: &str, v: &str) -> Option<Token> {
    if v.len() > 0 {
        let token = Token {
            kind: k.into(),
            val: v.chars().collect::<String>()
        };
        Some(token)
    } else {
        None
    }
}

fn is_operator(x: char) -> bool {
    match x {
        '+' | '-' | '/' | '*' | '^' => true,
        _ => false
    }
}
