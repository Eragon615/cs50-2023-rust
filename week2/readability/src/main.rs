use cs50::get_string;

#[derive(Debug)]
struct TextStats {
    words: u32,
    letters: u32,
    sentences: u32
}

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

fn main() {
    let text = get_input();
    let stats = get_text_stats(text);
    let grade = stats.calculate_grade();
    if grade < 1 {
        println!("Before Grade 1");
    } else if grade >= 1 && grade < 16 {
        println!("Grade {}", grade);
    } else {
        println!("Grade 16+");
    }
}

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