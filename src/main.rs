use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;
use clearscreen::ClearScreen;

fn main() {
    let mut length: u8 = 0;
    let mut features = [false; 3];

    length = get_length(&mut length);
    features[0] = get_yes_no("Should the password contain numbers (0-9)?");
    features[1] = get_yes_no("Should the password contain CAPITAL LETTERS?");
    features[2] = get_yes_no("Should the password contain special characters (@, %, !, etc)?");

    let password: String = generate_password(length, features);

    clear_screen();
    println!("Length: {length}");
    println!("Using numbers (0-9): {}", features[0]);
    println!("Using CAPTIAL LETTERS: {}", features[1]);
    println!("Using special characters (@, %, !, etc): {}", features[2]);
    println!("Password: {password}");
}

fn generate_password(length: u8, features: [bool; 3]) -> String {
    let lowercase_range = 'a'..='z';
    let numbers_range = '0'..='9';
    let uppercase_range = 'A'..='Z';
    let special_range = "!@#$%^&*()_+-=[]{}|;':,./<>?".chars();

    let mut chars_pool:Vec<char> = lowercase_range.collect();

    if features[0] {
        chars_pool.extend(numbers_range);
    }
    if features[1] {
        chars_pool.extend(uppercase_range);
    }
    if features[2] {
        chars_pool.extend(special_range);
    }

    let mut password: Vec<char> = Vec::new();
    password.resize(length.into(), '�');

    for char in password.iter_mut() {
        let character_option = chars_pool.choose(&mut thread_rng());
        match character_option {
            None => panic!("Failed to return a random character during password generation."),
            Some(rand_char) => *char = *rand_char
        }
    }

    password.into_iter().collect()
}

fn clear_screen() {
    ClearScreen::default()
        .clear()
        .expect("Failed to clear the screen.");
}

fn get_yes_no(question: &str) -> bool {
    let choice: bool;

    loop {
        clear_screen();
        println!("YES/NO: {}", question);
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input line.");

        let buffer = buffer.to_lowercase();
        let input = buffer.trim();

        choice = match input {
            "y" | "yes" | "true" => true,
            "n" | "no" | "false" => false,
            _                    => continue,
        };

        break;
    };

    choice
}

fn get_length(length: &mut u8) -> u8 {
    loop {
        clear_screen();
        println!("Desired password length? [5-128 characters]");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");

        *length = match buffer.trim().parse() {
            Ok(number) => number,
            Err(_)        => continue,
        };
        break
    };

    if *length < 5 {*length = 5}
    else if *length > 128 {*length = 128};

    *length
}