extern crate symspell;
use symspell::{AsciiStringStrategy, SymSpell, Verbosity};
use std::time::{Instant};
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let mut words: Vec<String> = Vec::new();
    let file = File::open("C:/misspelled.csv").expect("File not found");
    let reader = io::BufReader::new(file);

    
    let re = Regex::new(r"[a-zA-Z]+").unwrap();
    println!("Now Reading Words from dictionary");
    for line in reader.lines() {
        let line = line.expect("Invalid UTF-8");
        
        for word in re.find_iter(&line) {
            words.push(word.as_str().to_string());
        }
    }
    println!("Now setting up symspell");

    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    symspell.load_dictionary("C:/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    symspell.load_bigram_dictionary(
        "C:/frequency_bigramdictionary_en_243_342.txt",
        0,
        2,
        " ",
    );

    let start = Instant::now();

    
    

    for i in 0..words.len(){
      symspell.lookup(&words[i], Verbosity::Top, 2);
    }

    let end = start.elapsed().as_secs_f64();

    println!(
        "Correction Through {} words completed in {} seconds with average of {} seconds per thousand words",
        words.len() as f64,
        end,
        (end*(1000 as f64) / words.len() as f64)
    );

    Ok(())
}
