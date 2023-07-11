use cs50::get_string;

const POINTS: [usize; 26] = [1 ,3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];

fn main() {
    // Get input words from both players
    let word1 = get_string("Player 1: ").unwrap();
    let word2 = get_string("Player 2: ").unwrap();

    // Score both words
    let score1 = compute_score(word1);
    let score2 = compute_score(word2);

    //Show winner
    if score1 > score2 {
        println!("Player 1 wins!");
    } else if score1 < score2 {
        println!("Player 2 wins!");
    } else {
        println!("It's a tie!");
    }
}

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
