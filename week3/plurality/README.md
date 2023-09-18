Week 3 Problem 1 - Plurality
===

The first and mandatory problem of week 3, and another problem where I have to start by porting cs50's C code. 

# A difference of opinion, really
So I've said in some of the other challenges that I like to keep `main()` as simple as possible so that it reads almost like psuedocode. However, in this case they put quite a bit of logic in `main()`. I've changed a bit of their code to make it "better" (more in line with my style) and some changes were made because Rust simply doesn't work that way. More on that in a bit. Here's my `main()` function:

```rust
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
```

You might notice there's a for loop in there when I'd usually make it it's own function, but because I need that mutable reference to candidates I decided it would be best just to do this here instead of passing the reference through layers of functions. However, our struct and constant look pretty much the same.

```rust
const MAX: usize = 9;

struct Candidate {
    name: String,
    votes: usize,
}
```
I must admit I much prefer Rust's struct definitions, C's look so messy to me.

# Parsing args, again
Not much to see here, we check that there's more args than none, and less than MAX, or 9. We do have to put each argument into our candidate struct, then add that to a `Vec<Candidate>` but that's not really that complicated.

```rust
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
```

I also thought about making sure that you didn't double add a candidate, but that technically wouldn't have been in spec, so I opted to follow the functionality of the original code as much as possible. 

# Get voters and names
Once again I used `if let` syntax to safely unwrap and sanitize the user's input for both the number of voters you want, and their votes.

```rust 
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
```

# Election time
So before I go over the `vote()` function, I want to look back at that for loop in `main()`.

```rust
    for _ in 0..voter_count {
        let name = get_name();
        if !vote(name, &mut candidates) {
            println!("Invalid vote.")
        }
    }
```
This pretty closely mirrors the original C code. The `vote()` function should return only a bool, and if it's false then you print "Invalid vote." and move on. So the actual vote tallying must be a "side effect" of that function rather than a returned value. Since memory is more easily accessed in C this works fine, but in Rust we have much stricter scoping. I through about making `vote()` a method you called on the `Candidate` struct, but ultimately decided on this approach. Also notice that while I'm using a for loop, the "_" means it's basically a glorified counter. Now onto the actual function.

```rust
fn vote(name: String, candidates: &mut Vec<Candidate>) -> bool {
    for i in candidates {
        if name == i.name {
            i.votes += 1;
            return true;
        }
    }
    return false;
}
```

This isn't super clever of anything, but it did take me a short time to come up with. Since I pass in the mutable reference to candidates, I can iterate over all of them and compare their name to the string we pass in. If it matches, add a vote and return. If we exhaust the list without a match, then we return false. Meets requirements in about 9 lines of code (without agressive formatting) so I'm satisfied.

# How do I tell a computer to just pick the big one?
So now we have to `print_winner()`, and I'm much less satisfied with what I came up with here.

```rust
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
```

Essentially it cycles over the list twice, the first to find what the highest number of votes is, and the second time to see who got that number of votes. I really want to believe there's a better way to do this, perhaps if I gathered the "highest" variable earlier when the voting was going on... I could make my `Vec<Candidates>` into it's own struct, which contains the vector in a field, then it could have a highest field as well so that it's easier to pass that variable around without needing to carefully consider it's owner, but that would get pretty far from the original spirit of the exercise was (or at least what I think it was). But I feel like this solution is on par with what most students would provide, so I'm leaving it.

# That was fast
These challenges regularly take me longer to write this (whatever this is, blog maybe?) than doing the acutal coding. I'm not even sure this will ever be read either, which is a tad lonesome to think of. Still, this is a journey of self improvement, and putting these words to paper helps me to think about why I did things, and occasionally make improvements moments before I hit "commit". 