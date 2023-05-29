Week 1 Problem 2 - Cash
===
Part 2 of week 1 is Cash. Our goals are:
1. Accept a value from out user. Check that it's an integer between 1 - 99 inclusive. 
2. Calculate the number of coins it would take to provide change in US coins using the least number of coins, a "greedy" algorithm.

# Creativity stifled
When I first read this, I had some fun ideas about using floats, and leveraging rusts structs to model the change received, but in order to keep more in line with the cs50 course I'm mirroring the code they provide best I can. 

# It's not too bad
At the very least, the boilerplate they provide is pretty similar to what I'd do. I'd reuse the code by allowing the function to accept an enum of coins, but again this is in the spirit of baby steps for anyone taking the course. So we now our main looks like this:
```rust
fn main() {
    let mut cents = get_cents();
    let quarters = calculate_quarters(&cents);
    cents = cents - quarters * 25;
    let dimes = calculate_dimes(&cents);
    cents = cents - dimes * 10;
    let nickels = calculate_nickels(&cents);
    cents = cents - nickels * 5;
    let pennies = calculate_pennies(&cents);
    let coins = quarters + dimes + nickels + pennies;
    println!("{}", coins);
}
```

Pretty straight forward. I make cents mutable so that I can update it after each `calculate_whatever()` call, although just shadowing it would probably have worked fine too. 

# The functions
`get_cents()` is the same old same old as before. Accept input and sanitize it. The `calculate_whaterver()` is really simple too, they all look like this:
```rust
fn calculate_quarters(cents: &i32) -> i32 {
    let quarters = cents / 25;
    return quarters;
}
```
Each one obviously swaps out the value of coin it represents. Pennies is only slightly different:
```rust 
fn calculate_pennies(cents: &i32) -> i32 {
    let pennies = cents.clone();
    return pennies;    
}
```
Since dividing by 1 would be pointless, we just return the remaining cents. We need to clone the value since we don't want pennies to be a reference. At this point, we don't actually need cents anymore, so passing in ownership during this function would have been fine too.

# And that's all
Yeah, this one was really quick. Even my wife was surprised how easy it was once I found a good way to explain functions. We did choose the easier of the problem sets, but maybe I'll convince her to go back and also try the harder ones as practice. On to week 2!