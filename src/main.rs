use std::env;
use std::process::exit;
use std::str::FromStr;

fn main() {
    // first arg is text, second one is limit
    let args: Vec<String> = env::args().collect();

    let default_character_break_limit = 80;
    let break_at: usize;
    let text: &str;

    if args.len() < 2 {
        println!("\n\nYou must provide at least the text you need. eg: EXECUTABLE \"The text you need\"");
        exit(1);
    }

    match args.get(1) {
        None => {
            println!("\n\nYou must provide at least the text you need. eg: EXECUTABLE \"The text you need\"");
            exit(1)
        },
        Some(provided_text) => {
            text = provided_text;
        }
    }

    match args.get(2) {
        None => {
            println!("\n\nNo break at limit set, fallback to default");
            break_at = default_character_break_limit;
        },
        Some(break_at_option) => {
            break_at = i32::from_str(break_at_option).unwrap() as usize;
        }
    }

    let map_of_lines_of_text = map_words_to_limit(text, break_at);

    print!("\n\n{} lines \n\n\n OUTPUT:\n", map_of_lines_of_text.len());

    for line in map_of_lines_of_text.iter() {
        println!("{}", line.join(" "))
    }
}

fn map_words_to_limit(text: &str, break_at: usize) -> Vec<Vec<&str>> {
    let text_splitted: Vec<&str> = text.split_whitespace().collect();
    let text_size = text.len();

    let number_of_lines: f32 = (text_size / break_at) as f32;

    println!("\n\nNumber of predicted lines of text {}", number_of_lines);

    let mut map_of_lines = Vec::new();
    let mut words_per_line = Vec::new();

    let mut characters_per_line = 0;

    for word in text_splitted.into_iter() {
        // plus one because we need 1 for space
        characters_per_line += word.len() + 1;

        if characters_per_line > break_at {
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
