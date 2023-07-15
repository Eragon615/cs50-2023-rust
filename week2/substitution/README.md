Week 2 Problem 4 (Challenge) - Substitution
===
Still waiting on the wife to wrap up "readability", so I decided to tackle another labeled as "more comfortable". I was hoping this would keep me busy for a little while, but it only took about an hour to hack out. Also, I feel like it's a much cleaner implementation than when I did "credit". I wasn't happy at all with my code there (despite it working just fine) but this I'm very happy with. 

The goals:
1. Accept a cipher key as an arguement to the program.
2. Validate the cipher. It should be 26 non repeating alphabetical characters. 
3. Prompt the user for some plain text that will get enciphered. 
4. Perform the enciphering, and print the output.

# A tad argumentative
This is the first time I've encountered a CS50 problem that asked to take in an arguement, however this might just be because I'm doing them out of order. My wife is still working on building her confidence, so she's taking the easiest of the problems, so I only do these harder ones while I let her work out the problem she's on.

I've used the clap crate in the past when I was building a CLI app with rust, but that seems like overkill for this. I actually wasn't sure how to accept arguments in rust, but quickly learned it wasn't like in C. Turns out you use `let args: Vec<String> = env::args().collect();` to get a vector of strings. The only trick to remember is that the name of the program is the 0th element, so one argument to the program means two items in the vector. 

# Validating the cipher
So now that we have `args`, it's time to validate it. I sort of waffled here a bit, originally my `validate_args()` function would just make sure it was valid, but as I was adding the checks I decided to make it also return the formatted cipher. Perhaps I should have changed the name, but meh.
```rust
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
```
So we start by taking in the args, which as we stated is a vector of strings. Then our checks, in order, are:
1. Check that there are exactly 2 items. The first should be the name of the app (substitution) and the second should be our key. The instructions say any more or less should just remind the user how to use the app, so we print the instructions and exit.
2. Next we check the length of the key. We know that `input[0]` is the name and therefore `input[1]` should always be our key. If it's not 26 characters, then we need to remind the user it should be and exit.
3. For the next few checks we need to actually iterate over the key provided to us, and since we're doing that we might as well load it into a vector of chars so we can kill two birds with one stone. So we create output, which is an empty (for now) vector of chars.
4. Now we can use a for loop to iterate over the chars in our key. Our first check is to make sure the character is a letter. In C, I think it only uses ASCII anyway, but in Rust the `.is_alphabetic()` function will return true for a lot of characters, like 東 for instance. So we combine that with `.is_ascii()`. I didn't do a ton of checking, but this seems to catch most cases. And of course, we only want to catch when those two **aren't** true, so we use the `!`(not) operator. Since we're using `||`(or), we can say "if the charcter is not ascii or not an alphabetic letter, then inform the user and exit".
5. Our final check before adding it to the vector is check that it's not already in the vector. Another internal ffor loop that compares it to each item already in the vector does the trick nicely. I needed to make sure that upper and lower letters would still match (and thus fail) so I used the `.to_ascii_uppercase()` function here, which also had the nice effect of dereferencing `item`. 
6. If all of this passes, the character is added to the vector.

# Getting input, again...
Now that we have the cipher and we're sure it's valid, we can prompt the user for the text to encipher. This is the same as every other time I've done it. 
```rust
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
```
I probably shouldn't even include this, but I like the way the code blocks break up my walls of text.

# Enciphering the text
This function looks remarkably similar to the one in "scrabble" and indeed I copied a good chunk from it. 
```rust
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
```
The biggest difference is that I take in the cipher as one of the inputs to the function. Then, we iterate over the chars in input, and set the `char_to_add` to the vector using a match statemtent. We match on the char as a usize, which should get us the utf-8 code, and since ascii is compatible with utf-8 we can pick out the alphabetical letters with match arms of `65..=90` for the uppercase and `97..=122` for the lowercase. Then we subtract "A" of the appropriate case to get an index into our cipher, and in the case of the lowercase letters, we use `.to_ascii_lowercase()` to adjust them. Any other character, be it a space, punctuation, or non alphabetic or English (Roman?) character. I actually tried this with some Japanese, and it worked without crashing. 

# Just a quick look at `main()`
I realized that I've written a few of these code walkthroughs and didn't show `main()`. I like to keep it as simple as possible, and sort of use it like a blueprint for my code doing all the heavy lifting in my functions. But if you want to know and you're too lazy to click into the src folder and click main.rs, then here you go:
```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let cipher = validate_args(args);
    let text = get_input();
    let text = encipher_text(text, cipher);
    println!("ciphertext: {}", text);
}
```
In my mind this is super easy to use as instructions. Get the args, validate them into a cipher, get the text from the user, encipher that text, then print it. 

# Wrapping up
And with that, we're done. I wish I had a way to run check50 against these so I could be more sure they didn't throw me a curveball I missed, but I'm confident this code meets the goals they laid out.