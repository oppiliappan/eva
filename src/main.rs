use std::str;

struct Token {
    kind: String,
    val: String,
}

fn main() {
    let input: &str = "2y + 11 + sin(5)";
    let input = input.replace(" ", "");
    println!("{}", input);

    let mut num_vec: Vec<char> = vec![];
    let mut char_vec: Vec<char> = vec![];

    let mut result: Vec<Token> = vec![];

    for letter in input.chars() {
        if letter.is_digit(10) {
            num_vec.push(letter);
        } else if letter == '.' {
            num_vec.push(letter);
        } else if letter.is_alphabetic() {
            if num_vec.len() > 0 {
                let token = Token {
                    kind: "Literal".into(),
                    val: num_vec.iter().collect::<String>()
                };
                result.push(token);
                char_vec.push(letter);
            }
        } else if is_operator(letter) {

        }
    }

}

fn is_operator(x: char) -> bool {
    match x {
        '+' | '-' | '/' | '*' | '^' => true,
        _ => false
    }
}
