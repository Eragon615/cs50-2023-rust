Week 1 Problem 1 - Mario
===
So the first assignment is Mario. Our goals are:
1. Accept a value from out user. Check that it's an integer between 1 - 8 inclusive. 
2. Draw a right aligned "pyramid" like the ones often found at the end of levels in the original "Super Mario Brothers". It should be the same height as the number entered by the user.

# The cs50 crate
Just like in population, I'll use the cs50 crate to mirror the cs50.h the users have access to. For this assignment I just need the `get_i32` function.

# Functions
I like to make main() read like the assignment. So I've got two steps, both of which call functions. 
* `get_input()` prompts the user for input, validates it, and returns it to our variable `pyramid_height`.
* `draw_pyramid()` takes the height and, well, draws the "pyramid".

## The catch
The part that makes this interesting is that the pyramid is supposed to be right aligned. That means we need to print blank spaces in front of our "blocks" (which are the # symbol). 

# Code Walkthrough

## Get input
We've seen me do this before. A loop in a function that checks your input and when it's valid breaks out of the loop. Nothing fancy, just checks that you enter a number between 1 and 8.
```rust
fn get_input() -> i32 {
    let mut input;
    loop {
        input = match get_i32("Pyramid height: ") {
            Ok(i) => i,
            Err(_) => continue,
        };
        if input > 0 && input < 9 {
            break;
        }
    }
    return input;
}
```
I would prefer more verbose messages, but this is what cs50 expects.

## Two paths
There were two ways that came to mind when I was working this. I could do a plain loop that counts it's cycle, and break when I got to the desired cycle, or I could use a for loop that iterates over a range. I like for loops, so I went with it.
```rust
fn draw_pyramid(input_height: i32) -> () {
    for i in 1..=input_height {
        let spaces = &input_height - i;
        print_chars(spaces, ' ');
        print_chars(i, '#');
        print!("\n");
    }
}
```
The way I see it, the number of spaces can be found using the formula "total height minus the number of blocks in that row". Since I've adjusted everything to start at 1 instead of 0, "the number of blocks in that row" is always equal to the row we're on. This makes it easy. I used `i` for my cycle counter, which is conviniently the same as the row we're on and the number of blocks.  

So now each cycle I calculate `spaces` with the height the user has entered minus the row we're on (`i`). Then we use my `print_chars()` function to print that number of spaces, and finally the "blocks" (#). Once the line is formed, we print a newline with `\n`.

## A tale of two loops
The cs50 course hints that it wants students to use a loop inside a loop, and indeed I couldn't think of a better way to do it. Iterate over each "column" inside a loop that iterates over each "row". I did at least find a good way to reuse code in my `print_chars()` funciton, which looks like this:
```rust
fn print_chars(number_of_times: i32, char_to_print: char) -> () {
    let mut cycle = 0;
    loop {
        if cycle == number_of_times {
            break;
        }
        print!("{}", char_to_print);
        cycle += 1;
    }
}
```
This takes in the number of times it should run, and the character it should print. Fun fact, before starting this I had no idea the `print!()` macro would print to the screen without an automatic newline. So that's useful. 
Also, I used the traditional loop here with a cycle counter. I have no idea why. 
