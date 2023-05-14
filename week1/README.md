Week 1
====
Week 1 is population. Our goals are:  
1. Take input from the user for our starting population
   * Must be greater than or equal to 9, lest the population stagnate.
2. Take input from the user for our target for the ending population
3. Each year we should take our current population "n" and raise it by "n / 3", and lower it by "n / 4"
   * Floats need to be truncated, not rounded
4. Calculate how many years it will take to reach or exceed the target population

# The cs50.h library
~~Our first big challenge is that cs50 provides a library for getting and sanitizing user input. We'll need to recreate that library in rust. We'll likely reuse it in later weeks, so I'm going to make it it's own crate.~~  
It turns out, someone has actually written a [cs50 crate](https://lib.rs/crates/cs50). So that saves me quite a bit of time. 

# Lots of functions
I like to write lots of functions since it makes my main() easier to read. So we'll crate a starting population, and an ending population function to collect that information.

## get_starting_pop
So we want to get an integer, make sure it's equal to or higher than 9, and return it. Since cs50::get_int() returns a result, we also need to safely unwrap it.  
I create a mutable variable called `input` then start a loop so I can keep the user entering input until they enter a satisfactory integer. I used the .expect() method to quickly build out the logic, then came back and used a `match` statement to unwrap the result a bit more nicely.  
```rust
fn get_starting_pop() -> i32 {
    let mut input;
    loop {
        input = get_i32("Starting population: ").expect("Not an integer.");
        if input > 8 {
            break;
        }
        else {
            println!("{input} is lower than 9.");
        }
    }
    return input;
}
```
This will crash if you enter something other than a integer, but it will at least accept an integer and check that it's above 8.  

```rust
fn get_starting_pop() -> i32 {
    let mut input;
    loop {
        input = match get_i32("Starting population: "){
            Ok(s) => s,
            Err(_) => continue
        };
        if input > 8 {
            break;
        }
        else {
            println!("{input} is lower than 9.");
        }
    }
    return input;
}
```
This is a bit nicer, and should just loop if you enter something other than an integer.  

## get_ending_pop
So now I use the same technique to get your ending population. There's a few differences, such as needing to check that the number is higher than your first input, the starting population.
```rust
fn get_ending_pop(start: & i32) -> i32 {
    let mut input;
    loop {
        input = match get_i32("Ending population: "){
            Ok(s) => s,
            Err(_) => {
                println!("That's not an integer!");
                continue
            }    
        };
        if input > *start {
            break;
        }
        else {
            println!("{input} isn't higher than the starting population of {start}.");
        }
    }
    return input;
}
```
Huh, those look awful similar. That means a lot of code is being duplicated here. Maybe we can do it better.

## A better mousetrap
So we could swap out `start` with a `minimum`, and then just call this function twice. Once with a minimum of 9, and the second with a minimum of `starting_pop`. Let's see how that would look:
```rust
fn get_user_input(minimum: i32) -> i32 {
    let mut input;
    loop {
        input = match get_i32("Please enter an integer: "){
            Ok(s) => s,
            Err(_) => {
                println!("That's not an integer!");
                continue
            }    
        };
        if input > minimum {
            break;
        }
        else {
            println!("Your input must be higher than {minimum}.");
        }
    }
    return input;
}
```
Hey, that looks pretty good. I've changed the wording a little bit to be more agnostic of the situation, but now we need to dereference `starting_pop` as we pass it in. Luckily this is allowed in rust.
```rust
println!("Please enter a starting population of llamas, at least 9.");
let starting_pop = get_user_input(9);
println!("Great, now input your target ending population.");
let ending_pop = get_user_input(*&starting_pop);
```
This works fine, and we don't lose access to the `starting_pop` variable. Even better, we've reduced duplicated code. 

## calculate_years
So all that's left is to do the math. I've put this in a new fuction, creatively enough called `calculate_years`.
```rust
fn calculate_years(start: & i32, end: & i32) -> i32 {
    let mut years = 0;
    let mut current_pop = *start;
    loop {
        let increase = current_pop / 3;
        let decrease = current_pop / 4;
        current_pop = current_pop + increase - decrease;
        years += 1;
        if current_pop >= *end {
            break
        }
    }
    return years;
}
```
So once again we need to define two mutable variables. One, `years` needs to outlive the loop we'll be updating it in, and two `current_pop` I need to set once and update throughout the life of the loop. (Future Clayton here, actually I didn't need to. I could have passed ownership of the variable in, since I don't need it after this. Oh well, it's a single line.)  
So we start the loop, calculate how much herd will increase and decrease, then adjust `current_pop` by those two ammounts. We also add 1 to years, and finally check to see if we've reached our target or not. Quite convintently, it seems that Rust truncates integer division, so the truncation was free. 

# Summary
So that's week one. It actually took me about 2 hours, but a lot of that was spent on false starts such as implementing my own cs50 crate only to discover someone else beat me to it, cleaning up my code by unwrapping Result<>s a bit nicer, and writing this code walkthrough. Probably only took about 30 minutes of actual work.