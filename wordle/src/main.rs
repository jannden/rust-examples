use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const MAX_GUESSES: usize = 6;
const WORD_LENGTH: usize = 5;

fn read_user_guess() -> io::Result<String> {
    let mut guess = String::new();
    println!("Enter your guess:");
    io::stdin().read_line(&mut guess)?;
    Ok(guess.trim().to_lowercase())
}

fn load_words(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let words = lines
        .filter_map(Result::ok)
        .filter(|word| word.len() == WORD_LENGTH)
        .collect();
    Ok(words)
}

fn select_random_word(words: &[String]) -> Option<&String> {
    let mut rng = thread_rng();
    words.choose(&mut rng)
}

fn provide_feedback(guess: &str, target: &str) -> String {
    let mut feedback = String::new();

    for (guess_char, target_char) in guess.chars().zip(target.chars()) {
        if guess_char == target_char {
            feedback.push('✓'); // Correct letter, correct position
        } else if target.contains(guess_char) {
            feedback.push('?'); // Correct letter, wrong position
        } else {
            feedback.push('✗'); // Incorrect letter
        }
    }

    feedback
}

fn main() -> io::Result<()> {
    let words_path = Path::new("src/words.txt");
    let words = load_words(words_path)?;

    if let Some(target_word) = select_random_word(&words) {
        println!("Debug target_word: {}", target_word);
        println!("Guess the word: _ _ _ _ _"); // Assuming WORD_LENGTH is 5
        let mut attempts = 0;

        while attempts < MAX_GUESSES {
            let user_guess = read_user_guess()?;
            if user_guess.len() != WORD_LENGTH {
                println!("Please enter a {}-letter word.", WORD_LENGTH);
                continue;
            }

            if user_guess == *target_word {
                println!("Congratulations! You've guessed correctly: {}", target_word);
                return Ok(());
            } else {
                attempts += 1;
                let feedback = provide_feedback(&user_guess, &target_word);
                println!("Feedback: {}", feedback);

                if attempts < MAX_GUESSES {
                    println!("Try again! Attempts left: {}", MAX_GUESSES - attempts);
                } else {
                    println!("Game over! The correct word was: {}", target_word);
                }
            }
        }
    } else {
        println!("No suitable word found.");
    }

    Ok(())
}
