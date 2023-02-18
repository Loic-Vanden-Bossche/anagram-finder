extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn parse() -> Vec<String> {
    let file = File::open("data/dic.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut in_form = false;

    let mut words = Vec::new();

    println!("Parsing...");

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "form" {
                    in_form = true;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "form" {
                    in_form = false;
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                if in_form {
                    // println!("{}", s);
                    words.push(s);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    println!("{} words parsed", words.len());

    return words;
}

fn find_anagrams(word: &str, words: &Vec<String>) -> Vec<String> {
    println!("Finding anagrams for '{}'...", word);
    let mut anagrams = Vec::new();

    for w in words {
        if w.len() == word.len() {
            let mut w = w.clone();
            let mut word = word.to_string();

            w.make_ascii_lowercase();
            word.make_ascii_lowercase();

            let mut w_chars = w.chars().collect::<Vec<char>>();
            let mut word_chars = word.chars().collect::<Vec<char>>();

            w_chars.sort();
            word_chars.sort();

            if w_chars == word_chars {
                if !anagrams.contains(&w) && w != word {
                    anagrams.push(w);
                }
            }
        }
    }

    return anagrams;
}

fn main() {
    let words = parse();

    loop {
        let mut word = String::new();

        println!("Enter word (or 'exit' to quit): ");

        std::io::stdin().read_line(&mut word).expect("Failed to read line");

        word = word.trim().to_string();

        if word == "exit" {
            break;
        }

        let anagrams = find_anagrams(&word, &words);

        println!("{} anagrams found", anagrams.len());

        for a in anagrams {
            println!("{}", a);
        }

        println!();
    }
}
