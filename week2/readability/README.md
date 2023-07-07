Week 2 Problem 1 - Readability
===

It took a while for my wife to get through Scrabble, so I got a bit of an extended break. When she finally completed it and I got to move on to this, I was pretty ready for something new and rushed through it in about 45 minutes. Honestly most of that time was spent learning how to cast unsigned integers as floats. I also played a little bit with structs here, even though it was completely unnecessary. I was just having some fun.

The goals this time were:
1. Take in a string. It will contain a passage from a book, such as a few sentences or a paragraph.
2. Calculate the number of letters, words, and sentences in that string.
    * They let us make a few assumptions here to make it easier:
        1. All words are separated by spaces. Essentially words equals spaces plus one.
        2. All sentences end with a period, question mark, or exclaimation mark. 
    * Nice of them to not throw us any curveballs like double spaces or semicolons.
3. Using a formula called The Coleman-Liau Index, we can calculate the grade level of school a person might need to be in to read the passage.

# The Coleman-Liau Index
So this formula was given to us:

> index = 0.0588 * L - 0.296 * S - 15.8

The **L** is the number of letters per 100 words, and the **S** is the number of sentences per 100 words. So if we can find the number of letters, number of words, and number of sentences in the passage given, we'll be able to do some math to find **L** and **S**, then plug in those values and be pretty close to our answer.

# Feels like I'm floating
This is the first problem to ask us to use floats (this will be fun to explain to the wife), but they also ask us to use integers, such as when printing the grade level. Also, they ask us to round the index to the nearest grade level, so casting the float back to an integer and just truncating the decimals off won't be enough.

With Rust, casting back and forth isn't too difficult with one caveat; any float you cast to must be one size bigger than the integer you're casting in. So if you pass it a u32, it must go to an f64. This is so the float can contain the value without overflowing, which makes sense if you know how floats work in memory (and if you don't, there's some great YouTube videos out there explaining it better than I can). Also, the standard library has the `.round()` method on floats, so we can round our value to the nearest whole number before casting it back to an integer. 

# The Code
## Input
So the first think I do is prompt for input, just like I've done in all the other challenges and am likely to keep doing for a lot of this course.
```rust
fn get_input() -> String {
    let input;
    loop {
        input = match get_string("Text: ") {
            Ok(i) => i,
            Err(_) => continue,
        };
        break;
    }
    return input;
}
```
There's no checking needed here, and honestly I'm not even sure how the program could fail to get a valid string from the user... But just in case, it will loop until it does get a valid string and then return that string.

## Strings as an Array of Bytes
Since week 2 is called Arrays, it should come as no surprise that we're using the same technique as in scrabble where we iterate over the characters in a string as if they were an array. In fact, it was pretty much a copy paste of the function I wrote there.
```rust
fn get_text_stats(text: String) -> TextStats {
    let mut words = 1;
    let mut letters = 0;
    let mut sentences = 0;
    for char in text.chars() {
        match char {
            'a'..='z' | 'A'..='Z' => letters += 1,
            ' ' => words += 1,
            '.' | '!' | '?' => sentences += 1,
            _ => continue
        }
    }
    let stats = TextStats { 
        words: (words), 
        letters: (letters), 
        sentences: (sentences) 
    };
    return stats;
}
```
As mentioned before, I load the data (letters, sentences, and words) into a struct. This is just for my own fun, I could have just as easily returned the variables in a tuple. 

The match block is doing the work of deciding when to increase each value. If it finds any letter, "A" through "Z", whether capital or lowercase, it increases letters by one. If if finds punctuation, it increases sentences by one. And if it finds a space, it increase words by one. Notice that words starts at one instead of zero. Since you always start your text with a word, and never end with a space, the total words is always going to be spaces plus one. The course said this was safe assumption to make as well. Finally, if it finds anything else, it just moves to the next character. In reality, this is kinda bad. The means that numbers, accented characters and non-roman characters, etc. would all get skipped, but for this challenge it's okay.

## A dash of math
Finally we have the counts, so we can do the calculations. Since I have the data in a struct, I implemented a method for it. Again, this could have just as easily (maybe more easily) been a function, but I was having fun with it.
```rust
impl TextStats {
    fn calculate_grade(self) -> usize {
        //letters / words * 100 = letters_per_words
        let letters_per_word: f64 = self.letters as f64 / self.words as f64 * 100.0;

        //sentences / words * 100 = sentences_per_words
        let sentences_per_words: f64 = self.sentences as f64 / self.words as f64 * 100.0;

        //index = 0.0588 * letters_per_words - 0.296 * sentences_per_words - 15.8
        let index = (0.0588 * letters_per_word - 0.296 * sentences_per_words -15.8).round() as usize;

        return index;
    }
}
```
The math in the comments is pulled straight out of the instructions from the course, nothing special or creative here. This is where I use the `as` keyword a lot to cast my unsigned integers to floats, and back. 

As a side note, a u32 holds a max value 4,249,967,295. That's 4.3 trillion. Since we're using a u32 to hold the letters, it could hold the entirety of the Encylopedia Britannica which according to Google has about 300 million characters of text. Now, I don't think our match logic is sufficient to actually calculate the grade level for it, but at least our variables are of sufficient size!

## In plain English please
So we have our index returned, but it can actually be as low as 0, or higher than 16. I think 0 could be called Kindergarden, and 13, 14, 15, and 16 could be Freshman, Sophomore, Junior, and Senior in college respectively, anything about 16 starts to get a tad hard to quantify. The course wants us to use the words "Before Grade 1" for 0, and "Grade 16+" for anything 16 or higher. 
```rust
    if grade < 1 {
        println!("Before Grade 1");
    } else if grade >= 1 && grade < 16 {
        println!("Grade {}", grade);
    } else {
        println!("Grade 16+");
    }
```
Nothing fancy here. Just some basic if then logic to print what they want to see. 

# Done
So that's it. I tested the code with a few of their examples and got the expected output. There's another problem set required for this week, but you get to choose from four options. Since I'm following along with my wife I'll have to wait and see which she decides to do. Although honestly I'm having so much fun doing this in Rust that I might just go back and do them all. We'll see. 