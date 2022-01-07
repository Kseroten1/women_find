use std::io;

fn main() {
    loop {
        let cut_input = get_input_from_user();

        let woman_name = get_woman_name_from_text(&cut_input);

        if woman_name.eq(&Some("Laura".to_string())) {
            println!("Laura is my bro <3");
            break;
        }

        print_message_based_on_name_presence(woman_name);
    }
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.trim().to_string()
}

fn get_woman_name_from_text(trimmed_input: &str) -> Option<String> {
    let words = trimmed_input.split_whitespace();
    for word in words {
        let first = word.chars().nth(0).unwrap();
        let last = word.chars().nth_back(0).unwrap();
        if first.is_uppercase() && last == 'a' {
            return Some(word.to_string());
        }
    }

    return None;
}

fn print_message_based_on_name_presence(name: Option<String>) {
    match name {
        None => println!("No woman no cry"),
        Some(word) => println!("Found problem, name is {}", word),
    }
}
