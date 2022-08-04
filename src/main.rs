use crate::word::{Letters, Word};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use clap::Parser;

mod word;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Path to the input file
    #[clap(short = 'i', long = "input", default_value = "./words_alpha.txt")]
    input_file: String,
    /// Path to the output file
    #[clap(short = 'o', long = "output", default_value = "./output.txt")]
    output_file: String,
}

fn main() {
    let args = Cli::parse();

    println!("Initializing...");

    let word_list: Vec<String> = read_file(&args.input_file);

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
        "\nDone after {:.2} seconds.\n",
        start_time.elapsed().as_secs_f64()
    );
    println!("Found {} word pairs", word_pairs.len());

    write_file(&args.output_file, word_pairs);
    println!("Saved to {}", &args.output_file);

}

fn find_matches(words: Vec<Word>) -> Vec<[Word; 5]> {
    let len = words.len();
    let mut word_pairs: Vec<[Word; 5]> = Vec::new();
    if len <= 5 {
        println!("Too few distinct, valid words were found ({}, to be exact). Please expand your word list!", len);
        println!("Words:");
        for w in words.iter() {
            println!("  {}", w);
        }
    } else {
        let pb = ProgressBar::new(len as u64 - 4);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {percent}% [{wide_bar:.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )
            .unwrap()
        );
        for i in (4..len).progress_with(pb) {
            let word1 = &words[i];
            let til1 = Letters::from(word1);

            for i in 3..i {
                let word2 = &words[i];
                if !til1.shares_letters(Letters::from(word2)) {
                    let til2 = til1 | Letters::from(word2);

                    for i in 2..i {
                        let word3 = &words[i];
                        if !til2.shares_letters(Letters::from(word3)) {
                            let til3 = til2 | Letters::from(word3);

                            for i in 1..i {
                                let word4 = &words[i];
                                if !til3.shares_letters(Letters::from(word4)) {
                                    let til4 = til3 | Letters::from(word4);

                                    for i in 0..i {
                                        let word5 = &words[i];
                                        if !til4.shares_letters(Letters::from(word5)) {
                                            //println!(
                                            //    " - - -\n {}\n {}\n {}\n {}\n {}",
                                            //    word1, word2, word3, word4, word5
                                            //);
                                            word_pairs.push([
                                                word1.clone(),
                                                word2.clone(),
                                                word3.clone(),
                                                word4.clone(),
                                                word5.clone(),
                                            ]);
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
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .collect(),
        Err(error) => {
            let url = "https://gist.githubusercontent.com/cfreshman/cdcdf777450c5b5301e439061d29694c/raw/b8375870720504ecf89c1970ea4532454f12de94/wordle-allowed-guesses.txt";
            panic!("Could not read word list from file. Please get the list and save it, then change the path in /src/.\nError: {}\nURL: {})", error, url);
        }
    }
}

fn write_file(path: &str, word_pairs: Vec<[Word; 5]>) {
    match File::options().write(true).create(true).open(path) {
        Err(err) => {
            panic!("An error occurred while write to output file: {}", err)
        },
        Ok(f) => {
            let mut f = BufWriter::new(f);
            for pair in word_pairs {
                writeln!(
                    f,
                    "{}\n{}\n{}\n{}\n{}\n",
                    pair[0], pair[1], pair[2], pair[3], pair[4]
                ).unwrap();
            }
        },
    };
}

/// Convert strings to the abstract word::Word type.
fn abstractify_words(word_list: Vec<String>) -> Vec<Word> {
    let mut words: HashSet<Word> = HashSet::new();
    let mut ignored = 0;
    for word_str in word_list.iter() {
        match Word::try_from(word_str) {
            Ok(word) => {
                // the word consisted of 5 unique, recognized letters

                let x = words
                    .take(&word)
                    .map(|mut x| {
                        x.add_word(&word);
                        x
                    })
                    .unwrap_or(word);

                words.insert(x);
            }
            Err(_) => {
                //println!("Ignored word '{}'.", word_str);
                ignored += 1;
            }
        };
    }

    if ignored != 0 {
        println!("Warning: Ignored {} words", ignored);
    }
    println!("Using {} words (+ {} anagrams/duplicates)", words.len(), word_list.len()-ignored-words.len());
    std::thread::sleep(std::time::Duration::from_secs(2));
    words.into_iter().collect()
}
