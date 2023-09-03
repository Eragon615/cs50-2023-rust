Week 2 Problem 5 (Challenge) - Wordle 50
===
I'm trying something different for this challenge. I'm going to write this code walkthrough as I write it, as opposed to writing it once I'm done. I think I'll get more interesting code blocks this way, as well as maybe cast better light on my thought process as I work through these.

# Step 1 - Port the C code
So, there's a lot of code that cs50 provides for this challenge. I've thus far been trying to use any functions that they provide so that my solution looks similar to what a student might come up with. That means I should start by porting their code. 

## C Prelude
```c
#include <cs50.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

// each of our text files contains 1000 words
#define LISTSIZE 1000

// values for colors and score (EXACT == right letter, right place; CLOSE == right letter, wrong place; WRONG == wrong letter)
#define EXACT 2
#define CLOSE 1
#define WRONG 0

// ANSI color codes for boxed in letters
#define GREEN   "\e[38;2;255;255;255;1m\e[48;2;106;170;100;1m"
#define YELLOW  "\e[38;2;255;255;255;1m\e[48;2;201;180;88;1m"
#define RED     "\e[38;2;255;255;255;1m\e[48;2;220;20;60;1m"
#define RESET   "\e[0;39m"
```

## Rust Prelude
```rust 
use cs50;
use colored::Colorize;
use rand::random;
use std::env;

const LISTSIZE: usize = 1000;

const EXACT: usize = 2;
const CLOSE: usize = 1;
const WRONG: usize = 0;
```

## Prelude comments
So we'll see if I need to import some more crates as I go on, but just to start I think the cs50 crate for it's `get_string()` function for getting guesses, colored so I don't have to deal with those terminal color codes, and rand since I saw that it randomly selects from our lists of words. The C code does so psuedorandomly, and technically all random on computers is psuedorandom, but I felt like the rand crate would just be better. 

I also set up LISTSIZE as a constant, ~~and turned the EXACT, CLOSE, WRONG values into an enum. If needed, I can assign the values to them later, but I'm betting that matching the enum will be enough.~~ Turns out, I was wrong. I ended up changing them back to constants to match the C code.

## C `main()`
```c
int main(int argc, string argv[])
{
    // ensure proper usage
    // TODO #1

    int wordsize = 0;

    // ensure argv[1] is either 5, 6, 7, or 8 and store that value in wordsize instead
    // TODO #2

    // open correct file, each file has exactly LISTSIZE words
    char wl_filename[6];
    sprintf(wl_filename, "%i.txt", wordsize);
    FILE *wordlist = fopen(wl_filename, "r");
    if (wordlist == NULL)
    {
        printf("Error opening file %s.\n", wl_filename);
        return 1;
    }

    // load word file into an array of size LISTSIZE
    char options[LISTSIZE][wordsize + 1];

    for (int i = 0; i < LISTSIZE; i++)
    {
        fscanf(wordlist, "%s", options[i]);
    }

    // pseudorandomly select a word for this game
    srand(time(NULL));
    string choice = options[rand() % LISTSIZE];

    // allow one more guess than the length of the word
    int guesses = wordsize + 1;
    bool won = false;

    // print greeting, using ANSI color codes to demonstrate
    printf(GREEN"This is WORDLE50"RESET"\n");
    printf("You have %i tries to guess the %i-letter word I'm thinking of\n", guesses, wordsize);

    // main game loop, one iteration for each guess
    for (int i = 0; i < guesses; i++)
    {
        // obtain user's guess
        string guess = get_guess(wordsize);

        // array to hold guess status, initially set to zero
        int status[wordsize];

        // set all elements of status array initially to 0, aka WRONG
        // TODO #4

        // Calculate score for the guess
        int score = check_word(guess, wordsize, status, choice);

        printf("Guess %i: ", i + 1);
        
        // Print the guess
        print_word(guess, wordsize, status);

        // if they guessed it exactly right, set terminate loop
        if (score == EXACT * wordsize)
        {
            won = true;
            break;
        }
    }

    // Print the game's result
    // TODO #7

    // that's all folks!
    return 0;
}
```
That looks like a lot to port, but a lot of it is comments. 

## Rust `main()`
Okay, I'm breaking this into sections to explain things. 
```rust 
fn main() {
    let args: Vec<String> = env::args().collect();
    // ensure proper usage
    //TODO #1
    let wordsize = parse_args(args);

    // ensure argv[1] is either 5, 6, 7, or 8 and store that value in wordsize instead
    // TODO #2
    ...
}
```
First thing to address is the difference in how C and Rust deal with arguments to the program. A quick Google search revealed that the `main(int argc, string argv[])` thing is a common convention for dealing with args in C. Rust is going to use `env::args()` to put them in a vector. Luckily after that it's more of the same, where `args[1]` is still going to hold the value we care about. Also, looking at what TODO 1 and 2 are, I think I can easily combine them into a single function called `parse_args()` that takes in the args, validates that there's one, only one, and that it's between 5 and 8 inclusively. Then I can initialize wordsize as immutable with the result. 

```rust 
    // open correct file, each file has exactly LISTSIZE words
    let file = load_file(&wordsize);
    // load word file into an array of size LISTSIZE
    let file = match file {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error opening file {}.", format!("{}", wordsize.to_string()));
            exit(1);
        }
    };
```
This part is a little awkward, since there's a pretty big difference between C and Rust when it comes to loaing to a file. Since there's so many ways this could error, I figured it would be better to put it all in a function that could return a Result<\[String\], Error> so that I could just handle all the errors at once.
```rust
fn load_file(name: &usize) -> Result<Vec<String>> {
    let file_in = fs::File::open(format!("{}.txt", name.to_string()))?;
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
}
```
Here's the function. Not gonna lie, most was copied right off of Google. I still don't fully comprehend exactly how all the Result stuff works, like why I'm only hinting `Result<T>` instead of `Result<T, E>` or why I have a few Ok's in the last line. I did add the `format!()` to make wordsize work as a filename. 

---

So I had stepped away from them for a while (months?) and am just now picking it up. I'm looking back at my code trying to remember where I left off. Seems like the next thing to do is pick a string from that list of words. This looks like a job for the rand crate. So `cargo add rand`, and then:
```rust
use rand::random;
...
let choice = file[ random::<usize>() % LISTSIZE ].clone();
```
The compiler is telling me I should get a string out of that, so I guess it might be right. I could have also made it a reference to file instead of cloning, but I'm not sure if one is more desirable than the other. 

Next the C code sets up the number of guesses you get (the same as the wordsize plus one), and a bool for if you've won or not. No need to make a code block here. It also prints a kind of title for the game, using colored text. Time to put the colorized crate to work. 

```rust 
use colored::Colorize;
...
println!("{}", "This is Wordle50!".on_green());
println!("You have {} tries to guess the {}-letter word I'm thinking of.", guesses, wordsize);
```

### Main Game Loop
At long last we get to the main game loop, and I need to make a choice here. Cs50's code is leveraging the way C style for loops work. Do I want to just use loop, or try to force a rust style for loop to do what I want with ranges, for the sake of using the same kind of command? I ultimately decided just a plain loop was fine. 

```rust 
    let mut cycle = 0;
    loop {
        // obtain user's guess
        let guess = get_guess(&wordsize);

        // array to hold guess status, initially set to zero
        let mut status = vec![0; *&wordsize];

        // set all elements of status array initially to 0, aka WRONG
        // TODO #4

        // Calculate score for the guess
        let score = check_word(guess, wordsize, status, choice);

        print!("Guess {}:", cycle + 1);

        print_word();

        cycle += 1;

        if score == EXACT * wordsize {
            won = true;
            break;
        }

        if cycle >= guesses {
            break;
        }

    }
```
The cycle variable here works the same as 'i' does in the C code. It increment each time the loop runs and is used to check if you've run out of guesses. I did have to add a break condition at the end since the for loops work differently. The only thing left in main() is TODO #7, which is to print the results and then return. I don't need a return 0, so I'll omit that section for now. Also, the three functions included in the C code are all blank, so there's no need to port anything. 

So that's all of the porting done. Time to move on to actually solving the challenge. 

# The Solution
Phew, just porting took some doing. I'm going to solve this in order of the TODO sections in the provided C code. 

## TODO 1: Ensure proper usage
Oh, yeah, I decided earlier it would be easier to do this and #2 at the same time... 

## TODO 1 & 2: Parse the args
Okay, parsing args is really, really simple stuff. 
```rust
let wordsize = parse_args(args);

...

fn parse_args(args: Vec<String>) -> usize {
    if args.len() != 2 {
        println!("Usage: ./wordle wordsize");
        exit(1);
    }
    let input: usize = match args[1].trim().parse() {
        Ok(n) => n,
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
```
Maybe this could be a little more elegant, but it does the job. Since we set `wordsize` to the output of parse args, we've completed TODO 1 & 2 in one shot.

## TODO 3: Getting the guess
So at this point, the provided code should have loaded in the correct list of words, picked one at random, and set up the game. The player should be sitting at a prompt asking for a guess, which we'll parse and ensure it's actually the correct length. 
```rust 
fn get_guess(wordsize: &usize) -> String {
    // ensure users actually provide a guess that is the correct length
    // TODO #3
    let mut guess_to_check = String::new();
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
```
This is pretty easy, it's pretty much the same as how we've been doing all the other input sanitization with the cs50::get_* functions. The only difference is the check that it's length is the same as wordsize. 

## TODO 4: Already Complete?
So step four is to initialize the `status` variable to all 0, but... I'm pretty sure that due to the way Rust creates vectors and arrays, that's already done. So on to step 5.

## TODO 5: Check the word
Looking at this function, it takes in the guess, the wordsize, the status, and the choice, and uses this to calculate how many letters you got right. Since I'm typing this as I solve the challenge, I haven't really thought about how this is going to work yet, but I'm wondering if I'm going to have issues with needing to return a bunch of the variables in a tuple, or use mutable references, or something. And this is another great opportunity for me to express some thoughts about porting software into new languages. There's always this decision I have to make about whether it's better to use and exploit the features of the language I'm porting to (rust in this case) versus trying to keep it as close to the original code as possible. And I don't know that it should always be one of those choices over the other. But it makes me think of all the ports of video games to new platforms where we complain something is a "bad port". Well, I feel a little more sympathetic towards the devs. But I digress.

```rust 
fn check_word(guess: &String, wordsize: &usize, status: &mut Vec<usize>, choice: &String) -> usize {
    let choice_vec: Vec<char> = choice.chars().collect::<Vec<_>>();
    let guess_vec: Vec<char> = guess.chars().collect::<Vec<_>>();
    let mut score = 0;
    for i in 0..*wordsize {
        if guess_vec[i] == choice_vec[i] {
            status[i] = EXACT;
            score += EXACT;
        } else if choice_vec.contains(&guess_vec[i]) {
            status[i] = CLOSE;
            score += CLOSE;
        } else {
            status[i] = WRONG;
            score += WRONG;
        }
    }
    return score;
}
```
Okay, I knew what I wanted to do, but I did have to look up how to do it. I originally tried doing something like `choice_vec = vec![choice.chars().collect()]` but that doesn't work the way I thought. So a quick google reveals you can collect directly into the vector. Then I just use a mutable reference to status so it can be updated outside the function for later use, and references to `wordsize`, `guess` and `choice` since those will remain the same. 

Finally, the logic is simply to use a for loop to turn `i` into an index into those two vectors and then compare them. If the index into both is the same, you get exact. If that's not true but choice_vec at least contains it, you get close, and finally if neither of those are true then you get wrong. Score is used as a win condition, since if all the letters letters are right, then score will equal `wordlength` * 2 (the value EXACT represents). Anything lower, and you haven't won yet. 

## TODO 6: Print the word
So this is the function where we display back the word the player guessed with the color codes to represent if it was WRONG, CLOSE, or EXACT. That info is in status, and I can use the same trick as before to get an iterable vector of the chars, so I'm expecting this to be easy. 

```rust 
fn print_word(guess: String, wordsize: &usize, status: Vec<usize>) {
    // print word character-for-character with correct color coding, then reset terminal font to normal
    let guess_vec = guess.chars().collect::<Vec<_>>();
    for i in 0..*wordsize {
        match status[i] {
            EXACT => print!("{}", format!("{}", guess_vec[i]).on_green() ),
            CLOSE => print!("{}", format!("{}", guess_vec[i]).on_yellow() ),
            WRONG => print!("{}", format!("{}", guess_vec[i]).on_red() ),
            _ => unreachable!("How'd you do that?"),
        }
    }
    print!("\n");
}
```
As expected, that came together quickly. ~~Now, I'd like to once again harp on the fact that I'm trying to keep the code as close to the original as possilbe. You can see that earlier my instincts were to make EXACT, CLOSE and WRONG enums instead of constants. Had I done that, I wouldn't have that annoying bit of (hopefully) unreachable code. In fact, I'm seriously considering going back and changing it again...~~ Okay, I changed it back to an enum. What's the point of using rust if you aren't leveraging the type safety, etc.? Most of the time that just meant changing `EXACT`, etc. to `Correctness::EXACT as usize`, and now I don't need the unreachable arm of that match statement. The enum looks like this:
```rust
#[derive(Clone, Copy)]
enum Correctness {
    WRONG = 0,
    CLOSE = 1,
    EXACT = 2,
}
```
I needed that derive macro so that I could copy my values when I called them with `as usize`, otherwise it would have consumed the enum. 

## TODO 7: Print results
We're on the home stretch now! After the main game loop, there's just this last little bit of logic.

```rust
    // Print the game's results
    if won == true {
        println!("You won!");
    } else {
        println!("Sorry, you lost. The word was \"{}\".", choice);
    }
```
That's really it. In C they return 0, but that's not needed here. So let's test it!

# Dang my eyes!
I try testing my game and my colorblindness rears it's ugly head. The yellow and green look identical to me! Fortunately, my wife assures me that it's working as expected, so I guess all is well. I could change the CLOSE value to something like blue to make it easier to see, then you'd have the nice base RGB color scheme, but that's not the app's spec, so I'll leave it as is. I don't really want to play wordle all day anyway.

# Summary
Well that was fun. THere was a little hiatus there in the middle, but once I picked it back up the challenge moved along quickly and smoothly. With each challenge my Rust skills improve, so on to the next challenge!