use std::io::{BufRead, BufReader};
use std::fs::File;


const MAX_MOST_USED_WORDS: usize = 10;
const TOP_WORDS_COUNT_RANGE: usize = 3;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = std::env::args().nth(1)
        .or_else(|| panic!("A path to the file is required."))
        .unwrap();
    let file = File::open(&filepath)?;

    let mut counter_table = std::collections::HashMap::<String, usize>::new();
    let wordlist = BufReader::new(&file).lines().flatten();

    let mut top_count = 0;
    let mut top_words = std::collections::HashSet::<String>::new();

    // Record amount of times each word is met in text
    for line in wordlist {
        line.split_whitespace()
            .map(|w| {
                let wc = counter_table.entry(w.to_owned()).or_insert(0);
                *wc += 1;

                // Record the most top occured word count
                if *wc > top_count {
                    top_count = *wc;
                }
            })
            .count();
    }

    // Extract only most occured words within 3 range
    for (word, &count) in &counter_table {
        if top_words.len() >= MAX_MOST_USED_WORDS {
            break;
        }
        if count >= top_count.abs_diff(TOP_WORDS_COUNT_RANGE) {
            top_words.insert(word.clone());
        }
    }

    println!("Most used words are:");
    for word in &top_words {
        println!("({}) => {}", &word, counter_table.get(word).unwrap());
    }

    Ok(())
}
