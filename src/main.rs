use std::env;
use std::process::{ExitCode};

// Defaults
static BREAK_AT: usize = 80;

struct UserInputs {
    break_at: usize,
    text: String,
}

fn help() {
    println!("WE need at least one argument");
    println!("EG: splt-my-text \"SOME TEXT\" 50");
    println!("\n\n\n");
    print!("split-my-text THE_TEXT BREAK_AT");
}

fn main() -> ExitCode {
    // first arg is text, second one is limit
    let args: Vec<String> = env::args().collect();

    let user_inputs = match build_user_inputs(args) {
        Err(_) => {
            help();
            return ExitCode::FAILURE;
        },
        Ok(from_user_inputs) => from_user_inputs
    };

    let map_of_lines_of_text = map_words_to_limit(&user_inputs.text, user_inputs.break_at.try_into().unwrap());
    for line in map_of_lines_of_text.iter() {
        println!("{}", line.join(" "))
    }

    return ExitCode::SUCCESS;
}

fn build_user_inputs(arguments: Vec<String>) -> Result<UserInputs, &'static str> {
    if arguments.len() < 2 {
        return Err("We need some text");
    }

    Ok(UserInputs {
        break_at: match arguments.get(2) {
            None => BREAK_AT,
            std::option::Option::Some(user_break_at) => usize::from_str_radix(&user_break_at, 10).unwrap(),
        },
        text: arguments.get(1).unwrap().to_string()
    })
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
