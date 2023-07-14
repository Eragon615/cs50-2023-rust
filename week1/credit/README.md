Week 1 Problem (Challenge) - Credit
===
I got a wild hare and decided to go do the harder problem sets while I wait for my wife to catch up. Besides, I wanted a little more challenge that I had been getting so far. Hoo boy, I got what I wanted. I have no clue how you're "supposed" to solve this, but I'm nearly positive it wasn't the way I did it. In fact, I plan to go look up other solutions now that I've completed this. My code is not pretty, and I'm not proud of it. Still, it does work. So there's that.  
Here were the goals:
1. Accept a credit card number as an integer. Since the class wants you to use a long, a u64 should suffice.
2. Apply Luhn’s Algorithm to validate the card number. 
3. Match the length and starting digit(s) to the scheme used by VISA, AmEx, and Mastercard, or else return INVALID. 

It sounds so simple when you put it like that. 

# So uh, how?
The reason I'm sure my solution wasn't the intended solution is that I really, **really** wanted to turn the number into an array of digits I could iterate over, but arrays are week 2 and this is still week 1. So I'm thinking that they didn't want / expect you to turn the card number into an array. Yet I did.  

# An array of digits
So this doesn't seem like something most people do since I didn't immediately find any advice while Googling, nor any crates. No matter, I trudged on determined to do it this way.  

I figured, if I could make a mutable vector of zeroes, I could work backwards to fill it up using the modulo method they give you as a hint in the instructions. This is where I started overthinking way too much. I forgot that integer division in computers is different than regular human math. Remainders and decimals are straight up dropped. So when I wanted to get the 5 out of 1234567 for example, I could have done `1234567 % 1000` to get 567, then `567 / 100` to get 5, then load that into the vector. But uh, I forgot all about all of that at first. So I had this monster of a function that would recombine all of the trailing digits (I was working backwards, so I had the last digits loaded first) so that I could subtract them, e.g. I would have multiplied 6 by 10 and added 7 so that I could do `567 - 67` so that I could have a nice clean `500 / 100`. Ugh.  

Even after I realized all of that was dumb, which I only realized because of a bug in my code, the final product is still most likely way more complex than it has any right to be.
```rust
fn split_number(number: u64, length: &usize) -> Vec<u64> {
    let mut number_array: Vec<u64> = vec![0; *length];
    for position in 1..=*length {
        number_array[length - position] = (number % TEN.pow(position as u32)) / TEN.pow(position as u32 - 1);
    }
    return number_array;
}
```
So it takes the number and the length I calculated (I'll cover that in a sec), and uses the to create the array. We start by making an empty (that is, full of zeroes) vector using the length. Then a for loop lets me increment the position of my pointer until we get to "lenght". So if the card number is 13 digits, we'll go from position 1 to 13. Since I'm working backwards, position 1 would be the end of the number, the last digit. You can see that with `number_array[length - posistion]`. This was also a great way to deal with the off by one property of vectors and arrays in rust, where they start at 0.  

So at this point we do some math. Since I need to use modulo of 10, 100, 1000 and so on, using an exponent seemed to be the way to go. For some reason, a constant was easier to work with so I did `const TEN: u64 = 10;` and it helped with a lot of errors. I should probably make an effort to learn why, but it worked and I was okay with that. So now, 10 to the power of my position got me the number I needed to modulo by, and 10 to the power of my position minus 1 got the number to divide by. Iterate over all the positions, et viola, I had an array of digits in that number. Simple right?

# More about that `length` variable
So with `split_number()` I passed in `length`. I wrote a little function to calculate it, which like most of this challenge, was quite likely over engineered. 
```rust
fn get_length(number: &u64) -> usize {
    let mut digits: u32 = 1;
    loop {
        if number < &TEN.pow(digits) {
            break;
        }
        digits += 1;
    }
    return digits as usize;
}
```
That's right, loop over it, comparing it to 10 to the power of digits, and increasing digits by 1 each iteration. There's a saying something like, "if it's stupid and it works, it's not stupid", but I don't know that they ever saw this code.

# Luhn's Algorithm in Rust
So now that I've worked so hard for my array of digits, it actually made implementing Luhn's Algorithm pretty easy. The instructions are to "start from the second to last digit", but really so long as we get all of the even position digits from a number with an odd number of digits total, or the opposite, all of the odd position digits from a number with an even number it all works out the same. 

So we sort that into step one, and each gets doubled. We're supposed to add the sum of the digits of the product, or in other words if a number goes about 10 we split it, like 12 becoming 1 and 2. Since we're doubling, the highest this could ever become is 18, if you double a 9. So I decided if it's over 10, add 1 and the total modulo 10 to get the last digit and add that. In hindsight, subtracting 9 would have done the trick. 

Finally we add the digits we didn't use before to the mix, and add it all up. If it's divisible by 10, or if the number modulo 10 is equal to 0, then it's valid. My code looked like this:

```rust
fn validate_array(number_array: &Vec<u64>) {
    let mut set1 = 0; //The set that gets doubled
    let mut set2 = 0; //The set that doesn't get doubled
    let positional_offset = if let 0 = number_array.len() % 2 {
        0
    } else {
        1
    };
    for position in 0..number_array.len() {
        if (position + positional_offset) % 2 == 0 {
            if number_array[position] * 2 > 9 {
                set1 += 1;
                set1 += number_array[position] * 2 % 10;
            } else {
                set1 += number_array[position] * 2;
            }
        } else {
            set2 += number_array[position];
        }
    }
    if (set1 + set2) % 10 != 0 {
        println!("INVALID");
        exit(1);
    }
}
```
As you can see, I set up set1 and set2 and even left myself notes so I could remember which is which, then iterated over my number array one time using an offset to determine which digit went into which set. Finally add them up, check if the modulo is zero, and if not, we know the card is invalid and might as well exit there. 

The only other thing worth mentioning is I don't have to modify the vector, so I pass a reference so I can use it again for the final step.

# Check what type of card
Once we validate that card's number is valid, we need to identify what type of card it is. We need to compare the length and starting digits. I actually forgot the lenght check the first time I wrote this function, so I had this really pretty match function, only to have to toss it and start over. It looked something like this:
```rust
let type = match (number_array[0], number_array[1]) {
    (4, _) => "VISA",
    (3, 4) || (3, 7) => "AMEX",
    (5, 1) || (5, 2) || (5, 3) || (5, 4) || (5, 5) => "MASTERCARD"
    _ => "INVALID"
};
println!("{}", type);
```
But since I forgot the length, that all went out the window. I really wanted to just be done at this point, so I went on to do some pretty gross if/if else/else logic and called it good enough:
```rust
fn validate_type(number_array: Vec<u64>, length: usize) {
    if (length == 13 || length == 16) && (number_array[0] == 4) {
        println!("VISA")
    } else if (length == 15) && (number_array[0] == 3 && (number_array[1] == 4 || number_array[1] == 7)) {
        println!("AMEX")
    } else if (length == 16) && (number_array[0] == 5 && 
    (number_array[1] == 1 || number_array[1] == 2 || number_array[1] == 3 || number_array[1] == 4 || number_array[1] == 5)) {
        println!("MASTERCARD")
    } else {
        println!("INVALID")
    }
}
```

# Wrap up
So that's it. I really had to spend some time on this, and I'm half expecting to look up other solutions and face palm. I really think there must a better way that I just couldn't think of, but powering through before looking up the solution should be just as useful a practice as when I go lookup how others did it. 