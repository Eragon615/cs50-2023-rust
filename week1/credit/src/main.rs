use std::process::exit;
use cs50::get_u64;

const TEN: u64 = 10;

fn main() {
    let card_number: u64 = get_input();
    let length = get_length(&card_number);
    let number_array = split_number(card_number, &length);
    validate_array(&number_array);
    validate_type(number_array, length);
}

fn get_input() -> u64 {
    let input: u64;
    loop {
        input = match get_u64("Number: ") {
            Ok(i) => i,
            Err(_) => continue,
        };
        break;
    }
    return input;
}

fn split_number(number: u64, length: &usize) -> Vec<u64> {
    let mut number_array: Vec<u64> = vec![0; *length];
    for position in 1..=*length {
        number_array[length - position] = (number % TEN.pow(position as u32)) / TEN.pow(position as u32 - 1);
    }
    return number_array;
}

fn get_length(number: &u64) -> usize {
    let mut digits: u32 = 1;
    loop {
        if number < &TEN.pow(digits) {
            break;
        }
        digits += 1;
    }
    return digits as usize;
}

fn validate_array(number_array: &Vec<u64>) {
    let mut set1 = 0; //The set that gets doubled
    let mut set2 = 0; //The set that doesn't get doubled
    let positional_offset = if let 0 = number_array.len() % 2 {
        0
    } else {
        1
    };
    for position in 0..number_array.len() {
        if (position + positional_offset) % 2 == 0 {
            if number_array[position] * 2 > 9 {
                set1 += 1;
                set1 += number_array[position] * 2 % 10;
            } else {
                set1 += number_array[position] * 2;
            }
        } else {
            set2 += number_array[position];
        }
    }
    if (set1 + set2) % 10 != 0 {
        println!("INVALID");
        exit(1);
    }
}

fn validate_type(number_array: Vec<u64>, length: usize) {
    if (length == 13 || length == 16) && (number_array[0] == 4) {
        println!("VISA")
    } else if (length == 15) && (number_array[0] == 3 && (number_array[1] == 4 || number_array[1] == 7)) {
        println!("AMEX")
    } else if (length == 16) && (number_array[0] == 5 && 
    (number_array[1] == 1 || number_array[1] == 2 || number_array[1] == 3 || number_array[1] == 4 || number_array[1] == 5)) {
        println!("MASTERCARD")
    } else {
        println!("INVALID")
    }
}
