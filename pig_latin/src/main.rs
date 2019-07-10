extern crate unicode_segmentation;
extern crate unidecode;

use unidecode::unidecode;
use unicode_segmentation::UnicodeSegmentation;
use std::io;

fn get_first_character(word: &str) -> Option<char> {
    for c in word.chars() {
        return Some(c);
    }
    None
}
    
fn main() {
    let mut text = String::new();
    loop {
        text.clear();
        println!("Please input some text.");


        io::stdin().read_line(&mut text)
            .expect("Failed to read line");

        let words = &text.unicode_words().collect::<Vec<&str>>();
        for word in words {
            let graphemes = UnicodeSegmentation::graphemes(&word[..], true)
                .collect::<Vec<&str>>();

            if graphemes.len() == 0 {
                print!(" ");
            } else {
                let c = get_first_character(&unidecode(&graphemes[0]).to_lowercase()).expect("Empty graphemes.");
                if c.is_alphabetic() {
                    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                            print!("{}-way ", word);
                    } else {
                        for i in 1..graphemes.len() {
                            print!("{}", graphemes[i]);
                        }
                        print!("{}ay ", graphemes[0]);
                    }
                } else {
                    print!("{} ", word)
                }
            }
        }
        println!("");
    }
}
