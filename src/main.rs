use std::process::{ExitCode};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Number of characters per line
   #[arg(short, long, default_value_t = 80)]
   break_words_at: usize,

   /// Name of the person to greet
   #[arg(short, long)]
   text: String,
}

fn main() -> ExitCode {
    // Using "clap" to deal wih arguments => https://docs.rs/clap/latest/clap/
    let args = Args::parse();

    println!("\n\n/****** Split My Text ******/ \n\n");

    let map_of_lines_of_text = map_words_to_limit(&args.text, args.break_words_at.try_into().unwrap());
    for line in map_of_lines_of_text.iter() {
        println!("{}", line.join(" "))
    }

    return ExitCode::SUCCESS;
}

fn map_words_to_limit(text: &str, break_at: i32) -> Vec<Vec<&str>> {
    let text_splitted: Vec<&str> = text.split_whitespace().collect();

    let mut map_of_lines = Vec::new();
    let mut words_per_line = Vec::new();
    let mut characters_per_line: usize = 0;

    for word in text_splitted.into_iter() {
        // plus one because we need 1 for space
        characters_per_line += word.len() + 1;

        if characters_per_line > break_at.try_into().unwrap() {
            characters_per_line = word.len();

            map_of_lines.push(words_per_line);

            words_per_line = Vec::new();
            words_per_line.push(word);
        } else {
            words_per_line.push(word);
        }
    }

    // Add remaining text - last line of text hat didn't reach the limit
    map_of_lines.push(words_per_line);

    return map_of_lines;
}
