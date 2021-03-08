use std::io::{self, BufReader, prelude::BufRead};
use std::fs::File;

/** Reads wordlist.txt into Vec<String>. */
fn read_wordlist_file() -> Vec<String> {
    // Open field handler for wordlist.txt.
    let file = File::open("wordlist.txt")
        .expect("File \"wordlist.txt\" not found.")
    ;
    // Create buffer for wordlist.txt.
    let buffer = BufReader::new(file);
    
    // Output all lines in wordlist.txt as a Vec<String>
    buffer.lines()
        .map(|l| l.expect("Couldn't parse line."))
        .collect()
}

/** Reads 10 lines from STDIN into Vec<String> */
fn read_scrambled_words_from_stdin() -> Vec<String> {
    let mut scrambled_words: Vec<String> = Vec::with_capacity(10);
    let mut stdin_buffer = String::new();
    
    // Read 10 scrambled words.
    for index in 0..10 {
        // Read a scrambled word from STDIN.
        io::stdin()
            .read_line(&mut stdin_buffer)
            .expect(&format!("Failed to read line {} of 10.", index + 1))
        ;
        
        // Add scrambled word to scrambled_words
        scrambled_words.push(stdin_buffer.trim().to_string());

        // Clear stdin_buffer since we are reusing it.
        stdin_buffer.clear();
    }

    // Return scrambled_words.
    scrambled_words
}

fn main() {
    // Read wordlist.txt.
    let wordlist = &read_wordlist_file();

    // Read 10 words from STDIN.
    let scrambled_words = &read_scrambled_words_from_stdin();

    // Create vector for unscrambled words.
    let mut unscrambled_words: Vec<String> = Vec::with_capacity(10);

    // For every scrambled word.
    for scrambled_word in scrambled_words {
        // Sort the letters in the scrambled word.
        let mut letters: Vec<char> = scrambled_word.chars().collect();
        letters.sort_by(|a, b| a.cmp(b));
        let sorted_scrambled_word: String = letters.into_iter().collect();

        // For every word in wordlist.txt.
        for word in wordlist {
            // Sort the letters in the word.
            let mut letters: Vec<char> = word.chars().collect();
            letters.sort_by(|a, b| a.cmp(b));
            let sorted_word: String = letters.into_iter().collect();

            // If the sorted words match.
            if sorted_scrambled_word == sorted_word {
                // Add the unsorted word to unscrambled_words.
                unscrambled_words.push(word.to_string());
                break;
            }
        }
    }

    println!("");
    println!("Scrambled words: {}", scrambled_words.join(","));
    println!("");
    println!("Unscrambled words: {}", unscrambled_words.join(","));
}
