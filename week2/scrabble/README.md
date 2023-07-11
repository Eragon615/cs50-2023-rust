Week 2 Lab 1 - Scrabble
===
Our lab for Week 2 is to make a kind of simplified version of the board game Scrabble. Here's the goals.
1. Take in two strings, one for Player 1 and one for Player 2.
2. Using the provided array, score the words based on the letters they contain.
    * This will mean needing to normalize them for capitalization, and strip any symbols etc.
    * cs50 seems like it intends you to leverage the properties of ASCII here, not sure if Rust using UTF-8 is gonna make this weird or not.
3. Print which "player's" word wins based on that score.

# Times may change you
~~So far I've been relatively good about keeping my code pretty similar to the classes. But because of Rust's lifetimes and scope constraints on variables, I have to decide what to do about the array they provide. The simple solution would be to just move it into the `compute_score()` function. However, that means that the array gets recreated every time the function is called. In our case, the function is called twice, so the overhead is pretty tiny. But I'm still pretty sure it's bad form. After some deliberation though, I'm going to do it this way simply to keep my code as close to the original as possible.~~

~~How else could I do it though? My thought was a giant match statement. A closure can also read things outside of it's scope, so that could have worked too.~~

As it turns out, I was very, very wrong. `const` is a keyword, and it works just like any other language. I have no clue why I though it didn't exist in rust. So here I am, weeks later, fixing this.

```rust
const POINTS: [usize; 26] = [1 ,3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];
```
Hey, that was easy.


# Nice template
This lab, and several previous problems provide a nice starting off point for the code you're writing. You only really need to write the `compute_score()` function, and a tiny `if` statement to print the winner. Of course, since I'm not actually taking the class, and since I'm translating it to Rust, I've got a bit more to do, but it seems like a nice improvement over what I remember from when I took the course a few years ago.
Mine looks like this:
```rust
use cs50::get_string;

fn main() {
    // Get input words from both players
    let word1 = get_string("Player 1: ").unwrap();
    let word2 = get_string("Player 2: ").unwrap();

    // Score both words
    let score1 = compute_score(word1);
    let score2 = compute_score(word2);

    //Show winner
}
```
With of course the prementioned array now in the `compute_score()` function...
```rust
    let points: [i32; 26] = [1 ,3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];
```

This is actually the first time I've used an array in rust, usually I use vectors.  
Also, I'm using the `.unwrap()` method here since I really don't see any situation that the user could give an invalid string, but I guess I could turn this into a separate funciton with proper error handling.

# Computing the score
So watching my wife do this lab, I realized how nice it can be that C will just treat types like other types. Want to index into a string as an array of chars? Just use brackets on the string's variable like an array. Want to treat a char like an integer? Just use it in math. Rust is a bit more restrictive here, so I learned about the `chars()` function to turn a string into an array, and I learned about using `char as usize` to turn, well, chars into a usized integer. And since ASCII maps directly to UTF-8, there was no weirdness here.

I'm not sure if Rust has an `isupper()` and `islower()` function, and didn't even bother to look. I just matched on the ASCII values as ranges and subtracted the relevant value of "A" to get my 0 index for points. 

```rust
fn compute_score(input_word: String) -> usize {
    let mut score: usize = 0;
    for char in input_word.chars() {
        let index: usize = match char as usize {
            65..=90 => char as usize - 65,
            97..=122 => char as usize - 97,
            _ => continue
        };
    score += POINTS[index];
    }
    return score;
}
```
I forced a lot of things to be usize here, probably more than I needed to, but it helped keep all of my types compatible.  
So the word comes in, we set up the points array, create a mutable score variable, and then use a for loop to iterate over the chars in input_word.chars(). We get the index for points by matching our char and doing ASCII math as previously mentioned, then add to our score using that calculated index into the points array. Simple, right?

# Who won?
So all that's left is to announce the winner, and a nice simple `if/else if/else` block does this really well.
```rust
    if score1 > score2 {
        println!("Player 1 wins!");
    } else if score1 < score2 {
        println!("Player 2 wins!");
    } else {
        println!("It's a tie!");
    }
```
So that's the week 2 lab finished.