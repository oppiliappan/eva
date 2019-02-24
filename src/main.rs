use std::str;

#[derive(Debug)]
struct Token {
    kind: String,
    val: String,
}

fn main() {
    let input: &str = "2y + 11(1 + 2 + 3) + 12 /    4";
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
            drain_buffer("Literal", &num_vec[..], &mut result);
            char_vec.push(letter);
            num_vec.clear();
        } else if is_operator(letter) {
            drain_buffer("Literal", &num_vec[..], &mut result);
            num_vec.clear();
            drain_buffer("Variable", &char_vec[..], &mut result);
            char_vec.clear();
            result.push(Token { kind: "Operator".into(), val: letter.to_string() });
        } else if letter == '(' {
            if char_vec.len() > 0 {
                drain_buffer("Function", &char_vec[..], &mut result);
                char_vec.clear();
            } else if num_vec.len() > 0 {
                drain_buffer("Literal", &num_vec[..], &mut result);
                result.push( Token{kind: "Operator".into(), val: "*".to_string()} );
                num_vec.clear();
            }
            result.push( Token{kind: "Left Paren".into(), val: letter.to_string()} );
        } else if letter == ')' {
            drain_buffer("Literal", &num_vec[..], &mut result);
            num_vec.clear();
            drain_buffer("Variable", &char_vec[..], &mut result);
            char_vec.clear();
            result.push(Token { kind: "Right Paren".into(), val: letter.to_string() });
        } else if letter == ',' {
            drain_buffer("Literal", &num_vec[..], &mut result);
            num_vec.clear();
            drain_buffer("Variable", &char_vec[..], &mut result);
            char_vec.clear();
            result.push(Token { kind: "Function Arg Separator".into(), val: letter.to_string() });
        }
    }
    drain_buffer("Literal", &num_vec[..], &mut result);
    num_vec.clear();
    drain_buffer("Variable", &char_vec[..], &mut result);
    char_vec.clear();
    for token in &result {
        println!("{} => {}", token.kind, token.val);
    }
}

fn drain_buffer(k: &str, v: &str, result: &mut Vec<Token>) {
    if v.len() > 0 {
        let token = Token {
            kind: k.into(),
            val: v.chars().collect::<String>()
        };
        result.push(token);
    }
}

fn is_operator(x: char) -> bool {
    match x {
        '+' | '-' | '/' | '*' | '^' => true,
        _ => false
    }
}
