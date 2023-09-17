use std::{env::args, process::exit};
use cs50::get_string;

fn main() {
    let key = parse_args(args().collect());
    let plaintext = get_input();
    let ciphertext = encipher(plaintext, key);
    println!("ciphertext: {}", ciphertext);
}

fn parse_args(args: Vec<String>) -> usize {
    if args.len() != 2 {
        println!("Usage: ./caesar key");
        exit(1);
    }
    let key: usize = match args[1].parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Usage: ./caesar key");
            exit(1);
        }
    };
    return key;
}

fn get_input() -> String {
    loop {
        if let Ok(i) = get_string("plaintext: ") {
            return i;
        }
    }
}

fn encipher(plaintext: String, key: usize) -> String {
    let mut output = vec![];
    for i in plaintext.chars() {
        let newchar: u8 = match i as usize {
            97..=122 => {
                let offset = (i as usize - 'a' as usize + key) % 26;
                'a' as u8 + offset as u8
            },
            65..=90 => {
                let offset = (i as usize - 'A' as usize + key) % 26;
                'A' as u8 + offset as u8
            },
            _ => {
                i as u8
            }
        };
        output.push(newchar as char);
    }
    return output.into_iter().collect();
}