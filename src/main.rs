use crate::word::Word;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


mod word;

fn main() {
    println!("Initializing...");

    let word_list: Vec<String> = read_file("./word_list.txt");

    println!("\nAbstracting words... (timer starts here)\n");

    let start_time = std::time::Instant::now();
    let words: Vec<Word> = abstractify_words(word_list);

    println!("\nSearching for solutions...\n");
    /*
    Expected solution:
        FJORD
        GUCKS
        NYMPH
        VIBEX
        WALTZ
    Missing: Q
    */

    let word_pairs = find_matches(words);

    println!(
        "\nDone after {} seconds.\n",
        start_time.elapsed().as_secs_f64()
    );
    println!("Found the following pairs:");

    for pair in word_pairs {
        println!(
            "\n-> {}\n-> {}\n-> {}\n-> {}\n-> {}",
            pair[0], pair[1], pair[2], pair[3], pair[4]
        );
    }
}

fn find_matches(words: Vec<Word>) -> Vec<[&Word; 5]> {
    let len = words.len();
    let mut percent_previous = 0;
    let mut word_pairs: Vec<[&Word; 5]> = Vec::new();
    if len <= 5 {
        println!("Too few distinct, valid words were found ({}, to be exact). Please expand your word list!", len);
        println!("Words:");
        for w in words.iter() {
            println!("  {}", w);
        }
    } else {
        for i in 4..len {
            let percent_current = 100 * (i - 4) / (len - 4);
            if percent_current != percent_previous {
                println!("{}%", percent_current);
                percent_previous = percent_current;
            };

            let w1 = &words[i];

            for i in 0..i {
                let w2 = &words[i];
                let til1 = w1.clone_letters();

                if !til1.shares_letters(w2) {
                    let til2 = til1.combine_letters(w2);
                    for i in 0..i {
                        let w3 = &words[i];

                        if !til2.shares_letters(w3) {
                            let til3 = til2.combine_letters(w3);
                            for i in 0..i {
                                let w4 = &words[i];

                                if !til3.shares_letters(w4) {
                                    let til4 = til3.combine_letters(w4);
                                    for i in 0..i {
                                        let w5 = &words[i];

                                        if !til4.shares_letters(w5) {
                                            println!(
                                                " - - -\n {}\n {}\n {}\n {}\n {}",
                                                w1, w2, w3, w4, w5
                                            );
                                            word_pairs.push([w1, w2, w3, w4, w5]);
                                        };
                                    }
                                };
                            }
                        };
                    }
                };
            }
        }
    }
    word_pairs
}

fn read_file(path: &str) -> Vec<String> {
    match File::open(path) {
        Ok(file) => {
            let mut wl = Vec::<String>::new();
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        wl.push(line);
                    }
                    Err(_) => {}
                };
            }
            wl
        }
        Err(error) => {
            let url = "https://gist.githubusercontent.com/cfreshman/cdcdf777450c5b5301e439061d29694c/raw/b8375870720504ecf89c1970ea4532454f12de94/wordle-allowed-guesses.txt";
            println!("Could not read word list from file. Please get the list and save it, then change the path in /src/.\nError: {}\nURL: {})", error, url);
            panic!();
        }
    }
}

/// Convert strings to the abstract word::Word type.
fn abstractify_words(word_list: Vec<String>) -> Vec<Word> {
    let mut words: Vec<Word> = Vec::new();
    for word_str in word_list.into_iter() {
        match Word::from_string(word_str.clone()) {
            Some(word) => {
                // the word consisted of 5 unique, recognized letters
                let mut ok = true;
                for w in words.iter_mut() {
                    if *w == word {
                        // a word with the same signature (using the same 5 letters in any order) already exists.
                        w.add_str(word_str); //  add the string to the word object. This means that the word will be treated as one
                        ok = false; // during processing, speeding everything up, but if this word is part of a match, all possible
                        break; // arrangements of these five letters that were in the word list will be displayed to the user.
                    };
                }
                if ok {
                    // the word was valid and did not yet exist in the words vec,
                    words.push(word); //  so we have to add it. Vec::push is slow, but since the second part of my algorithm
                }; // takes so much longer, this is almost insignificant and not worth optimizing out.
            }
            None => {
                println!("Ignored word '{}'.", word_str);
            }
        };
    }
    words
}

fn find_matches {

}