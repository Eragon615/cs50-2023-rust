use std::{env, process::exit};
use cs50::get_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cipher = validate_args(args);
    let text = get_input();
    let text = encipher_text(text, cipher);
    println!("ciphertext: {}", text);
}

fn validate_args(input: Vec<String>) -> Vec<char> {
    if input.len() != 2 {
        println!("Usage: cargo run key");
        exit(1);
    };
    if input[1].len() != 26 {
        println!("Key must contain 26 characters.");
        exit(2);
    };
    let mut output: Vec<char> = vec![];
    for char in input[1].chars() {
        if !char.is_ascii() || !char.is_alphabetic() {
            println!("Key must contain alphabetic ASCII characters.");
            exit(3);
        };
        for item in &output {
            if item.to_ascii_uppercase() == char.to_ascii_uppercase() {
                println!("Key must contain no repeating characters.");
                exit(4);
            };
        }
        output.push(char.to_ascii_uppercase());
    }
    return output;
}

fn get_input() -> String {
    let input: String;
    loop {
        input = match get_string("plaintext: ") {
            Ok(s) => s,
            Err(_) => continue
        };
        break;
    }
    return input;
}

fn encipher_text(input: String, cipher: Vec<char>) -> String {
    let mut output: String= String::new();
    for char in input.chars() {
        let char_to_add = match char as usize {
            65..=90 => cipher[char as usize - 65],
            97..=122 => cipher[char as usize - 97].to_ascii_lowercase(),
            _ => char
        };
        output.push(char_to_add);
    }
    return output;
}