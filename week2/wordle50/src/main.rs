use cs50::get_string;
use colored::Colorize;
use rand::random;
use std::{env, io::{Result, BufRead, BufReader, self}, fs, process::exit};

const LISTSIZE: usize = 1000;

#[derive(Clone, Copy)]
enum Correctness {
    WRONG = 0,
    CLOSE = 1,
    EXACT = 2,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let wordsize = parse_args(args);

    // open correct file, each file has exactly LISTSIZE words
    let file = load_file(&wordsize);
    // load word file into an array of size LISTSIZE
    let file = match file {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error opening file {}.", format!("{}", wordsize.to_string()));
            exit(1);
        }
    };

    // pseudorandomly select a word for this game
    let choice = file[ random::<usize>() % LISTSIZE ].clone();


    // allow one more guess than the length of the word
    let guesses = wordsize + 1;
    let mut won = false;

    // print greeting, using ANSI color codes to demonstrate
    println!("{}", "This is Wordle50!".on_green());
    println!("You have {} tries to guess the {}-letter word I'm thinking of.", guesses, wordsize);

    // main game loop, one iteration for each guess
    let mut cycle = 1;
    loop {
        // obtain user's guess
        let guess = get_guess(&wordsize);

        // array to hold guess status, initially set to zero
        let mut status: Vec<Correctness> = vec![Correctness::WRONG; *&wordsize];

        // set all elements of status array initially to 0, aka WRONG

        // Calculate score for the guess
        let score: usize = check_word(&guess, &wordsize, &mut status, &choice);

        print!("Guess {}: ", cycle);

        print_word(guess, &wordsize, status);

        if score == Correctness::EXACT as usize * wordsize {
            won = true;
            break;
        }
        if cycle >= guesses {
            break;
        }

        cycle += 1;
    }

    // Print the game's results
    if won == true {
        println!("You won!");
    } else {
        println!("Sorry, you lost. The word was \"{}\".", choice);
    }

}

fn get_guess(wordsize: &usize) -> String {
    // ensure users actually provide a guess that is the correct length
    let mut guess_to_check: String;
    let message = format!("Input a {}-letter word: ", wordsize);
    loop {
        guess_to_check = match get_string(&message) {
            Ok(string) => string,
            Err(_) => continue
        };
        if guess_to_check.len() == *wordsize {
            break;
        };
    }
    return guess_to_check;
}

fn check_word(guess: &String, wordsize: &usize, status: &mut Vec<Correctness>, choice: &String) -> usize {
    // compare guess to choice and score points as appropriate, storing points in status
    let choice_vec: Vec<char> = choice.chars().collect::<Vec<_>>();
    let guess_vec: Vec<char> = guess.chars().collect::<Vec<_>>();
    let mut score = 0;
    for i in 0..*wordsize {
        if guess_vec[i] == choice_vec[i] {
            status[i] = Correctness::EXACT;
            score += Correctness::EXACT as usize;
        } else if choice_vec.contains(&guess_vec[i]) {
            status[i] = Correctness::CLOSE;
            score += Correctness::CLOSE as usize;
        } else {
            status[i] = Correctness::WRONG;
            score += Correctness::WRONG as usize;
        }
    }
    return score;
}

fn print_word(guess: String, wordsize: &usize, status: Vec<Correctness>) {
    // print word character-for-character with correct color coding, then reset terminal font to normal
    let guess_vec = guess.chars().collect::<Vec<_>>();
    for i in 0..*wordsize {
        match status[i] {
            Correctness::EXACT => print!("{}", format!("{}", guess_vec[i]).on_green() ),
            Correctness::CLOSE => print!("{}", format!("{}", guess_vec[i]).on_yellow() ),
            Correctness::WRONG => print!("{}", format!("{}", guess_vec[i]).on_red() ),
        }
    }
    print!("\n");
}

fn parse_args(args: Vec<String>) -> usize {
    if args.len() != 2 {
        println!("Usage: ./wordle wordsize");
        exit(1);
    }
    let input: usize = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: wordsize must be either 5, 6, 7, or 8");
            exit(1);
        },
    };
    if input == 5 || input == 6 || input == 7 || input == 8 {
        return input;
    } else {
        println!("Error: wordsize must be either 5, 6, 7, or 8");
        exit(1);
    }
}

fn load_file(name: &usize) -> Result<Vec<String>> {
    let file_in = fs::File::open(format!("{}.txt", name.to_string()))?;
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
}