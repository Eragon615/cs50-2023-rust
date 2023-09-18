use cs50::{get_usize, get_string};
use std::{env::args, process::exit};

const MAX: usize = 9;

struct Candidate {
    name: String,
    votes: usize,
}

fn main() {
    let mut candidates: Vec<Candidate> = parse_args(args().collect());
    let voter_count = get_voters();
    
    for _ in 0..voter_count {
        let name = get_name();
        if !vote(name, &mut candidates) {
            println!("Invalid vote.")
        }
    }

    print_winner(candidates);
}

fn parse_args(args: Vec<String>) -> Vec<Candidate> {
    if args.len() < 2 {
        println!("Usage: plurality [candidate ... ]");
        exit(1);
    }
    if args.len() > MAX {
        println!("Maximum number of candidates is {}", MAX);
        exit(2);
    }
    
    let mut candidates = vec![];

    for i in 1..args.len() {
        candidates.push(Candidate{
            name: args[i].clone(),
            votes: 0,
        })
    };

    return candidates;
}

fn get_voters() -> usize {
    loop {
        if let Ok(i) = get_usize("Number of voters: ") {
            return i;
        }
    }
}

fn get_name() -> String {
    loop {
        if let Ok(i) = get_string("Vote: ") {
            return i;
        }
    }
}

fn vote(name: String, candidates: &mut Vec<Candidate>) -> bool {
    for i in candidates {
        if name == i.name {
            i.votes += 1;
            return true;
        }
    }
    return false;
}

fn print_winner(candidates: Vec<Candidate>) {
    let mut highest: usize = 0;
    for i in &candidates {
        if i.votes > highest {
            highest = i.votes;
        }
    }
    for i in candidates {
        if i.votes == highest {
            println!("{}", i.name);
        }
    }
}