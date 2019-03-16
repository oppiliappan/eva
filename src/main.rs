#[derive(Debug)]
enum Token {
    Operator(char),
    Num(f64),
    Paren(char)
}

fn main() {
    let input = "11(12)";
    let input = input.replace(" ", "");

    let lexed = lexer(&input);

    println!("{:?}", lexed);
}

fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut num_vec: String = String::new();
    let mut result: Vec<Token> = vec![];
    for letter in input.chars() {
        match letter {
            '0'...'9' | '.' => {
                num_vec.push(letter);
            },
            '+' | '-' | '/' | '*' | '^' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
                result.push(Token::Paren(letter));
            },
            '('  => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    result.push(Token::Operator('*'));
                    num_vec.clear();
                }
                result.push(Token::Paren(letter));
            },
            ')' => {
                let parse_num = num_vec.parse::<f64>().ok();
                if let Some(x) = parse_num {
                    result.push(Token::Num(x));
                    num_vec.clear();
                }
                result.push(Token::Paren(letter));
            }
            _ => {
                return Err(format!("Unexpected character: {}", letter))
            }
        }
    }
    Ok(result)
}
