Week 2 Problem 3 - Caesar
===

This time, I decided to live code the exercise with my wife watching, so I could try to explain thinking programatically a little better. She gets overwhelmed when starting these problems because she doens't know how to do it all at once. I'm not sure that I was successful, but I was trying to demonstrate how to break the problem into smaller chuncks and work on each one independantly. 

The goals are as follows:
1. Accept a single argument, that is a non-negative integer. 
    * If used incorrectly, it should print it's usage and exit with an exit code of 1.
    * The arguement **can** be more than 26, and should "wrap around" in this case. 
2. The program should accept a string with the hint "plaintext: ".
3. Perform the enciphering. 
    * Capital letters should remain capital, and lowercase should remain lower. 
    * Non-alphabetic characters should be unchanged. This includes numbers, spaces, and punctuation.
4. Print "ciphertext: " followed by the enciphered text.

# `main()`
I like my `main()` function to be as simple as possible, for several reasons. 
1. By breaking it into a sequence of functions that have meaningful names, it becomes almost as easy to read as psuedocode.
2. It lends to concept of breaking the code into smaller blocks of work.
3. Modular code is easier for me to refactor or change later.

For that reason, my `main()` function looks like this:
```rust 
fn main() {
    let key = parse_args(args().collect());
    let plaintext = get_input();
    let ciphertext = encipher(plaintext, key);
    println!("ciphertext: {}", ciphertext);
}
```

You'll notice that my `main()` function lines up exactly with the goals for the assignment. Get the key from the command line args, get the text to encipher by prompting, encipher said text, then print it out. All of my heavy lifting is done in the functions.

# Parsing the arguments
So I've gotten command line arguments in other challenges, but as a refresher, `std::env::args()` gives you a iterator over the arguments as strings. I've run the `.collect()` function on them to turn them into a `Vec<String>`, which I then pass to my `parse_args()` function. And since I know that I want to get a positive integer back to use as the key, I can `let key = ` the result. 

```rust 
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
```

Now that we have our `Vec<String>` in the function, we can start performing checks to make sure it's what we want. `if args.len() != 2` does a good job of weeding out when it's run with no arguments or too many (remember that the program name is always argument 1, so our key value should always be arguement 2) so the only thing to do after that is make sure it's a positive integer. 

So we can use the `parse()` function on the second arg to ensure it's a number, however by telling the compiler that `key` is a `usize` we also ensure it's positive and a whole integer. Then we just match it to `Ok()` and return it, or print the usage message and exit. That's two checks to meet all of our criteria. At this point, I made sure to let my wife know that there's nothing wrong with doing a bunch of checks, like parse the arg into a number, then do `if number >= 0` to make sure it's positive, etc. But the more you practice, the more you'll be able to do things in less steps.


# Getting input, but intermediate
I've been using Rust for as many projects as I can so I can get more and more familiar with it. So sometimes instead of reusing the same block of code again and again, sometimes I'll look to see if I can rewrite something. We've been doing `get_input()` more or less the same way, but this time I changed it slightly.

```rust
fn get_input() -> String {
    loop {
        if let Ok(i) = get_string("plaintext: ") {
            return i;
        }
    }
}
```

`if let` was one of those things I knew was in the language, but I had no idea what it did and since I never needed it I never bothered to look it up and understand it. Well, I was reading someone elses code the other day and it used `if let` so I decided it was time I understood it. Basically it lets you match a single thing for an enum, especially useful for `Result<>` and `Option` types. I'll try to explain it better...

So we have the left and right sides of the statement, separated by the equals (=) sign. On the left, `if let` is followed by the desired enum varient. In this case I want the `Ok()` varient of `Result<>`. We also assign the value in `Ok()` to a variable, in this case `i`. </br>
Meanwhile on the right hand side, we use whatever results in an enum, in this case the `get_string()` function from the cs50 crate, which returns a `Result<>`.</br>
Finally, we use our curly braces to define the code that should be executed if the result of `get_string()` is `Ok()`, which in our case is return the string. </br> 
Basically, this is just unwrapping it, but instead of panicing if we get an `Err()` instead it will just loop and try again. So we get a safe function that won't crash, and it's very simple and short code. 

# Perform the encipher
This part was fun. I was trying to do it fast enough my wife wouldn't lose interest, and I ended up screwing up a couple of times. I *think* I could clean this up more, but since the goal was to show my wife how to move through one of these challenges programatically, I think I'll leave it how it is.

The plan is, we'll turn our string `plaintext` into a `Vec<Char>` that we can iterate over. Then we can detect if each `Char` is alphabetic or not, and upper or lowercase. We'll translate it to a new letter based on the key we got from the argument, and then add it to a new vector for the output string. 

```rust
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
```

Okay, full disclosure, I was looking at this code and hated it, so I rewrote it. The original code worked fine, but was pretty messy. This is still messy, but better.

So to start, we take in to the text to encipher and the key. Next, we create an empty vector called `output`, that we'll construct our enciphered string into. Now we use a for loop for `plaintext.chars()` to iterate over each letter in the string. 

Here I'm using a match statement similar to the way I did in [scrabble](https://github.com/Eragon615/cs50-2023-rust/blob/main/week2/scrabble/README.md). I match the letter as a usize to get it's ASCII value. From 65 to 90 means it's a capital letter, and 97 to 122 means it's a lowercase letter. Anything else is is a space, puctuation, etc, and I don't need to do anything to it. 

So my plan was to work with an offset, or in other words, how many places away from 'A' is it? So we start by getting how far from 'A' our original text is with `i` (which should be a letter from our plaintext) as a usize to represent it's ASCII value. Then we subtract 'a' (or 'A' if it's capital) in order to get it's place in the alphabet. Next we add the key value, and modulo by 26 in order to get the offset. Doing the modulo here means that the offset will "wrap around" when it gets to 'z' and start over again at 'a', which is how we meet the requirement that it accept values of over 26 for the key. 

Now that we have the offset, we just add that to 'a' (or 'A' if capital) to get the enciphered letter's value. And of course, non letters are returned at the same value so they are unaffected. Then the enciphered character is pushed onto the vector called `output` using the `as char` function to turn a u8 back into it's corresponding ASCII charcter. You'll notice that I returned all the values as u8, I got a rust analyzer message that leads me to believe that's the only integer size that can be converted to char.

Our final step is to turn this back into a string. We have a `Vec<Char>`, so turning into an iterator and collecting it works to make a string. I'm pretty sure this is because I've already told the compiler that the function returns a `String` so the `.collect()` function just knew what to do, but I think you can also be more explicit with `collect::<Sting>()`.

# Another Challenge Down
So there was Caesar. I am actually learning new tricks each time I do these, even though they feel really simple at this point. Mostly using the syntax over and over is helping me to understand what the language is doing at a deeper level. Using `if let`, `.into_iter()`, and `.collect()` helped me better understand what they do and when to use them, so I'd recommend anyone learning to code to do simple challenges, even if they feel below your skill level. If it's really easy to complete, try to make your code use less lines, or more complex syntax. It helped me improve so it should help you too.