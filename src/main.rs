use std::io;
use rand::seq::SliceRandom;
use clearscreen::ClearScreen;

fn main() {
    let mut length: u8;
    let mut feature_flags = [false; 3];
    
    length = loop {
        clear_screen();
        println!("Desired password length? [5-128 characters]");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        length = match buffer.trim().parse() {
            Ok(number) => number,
            Err(_)     => continue,
        };

        if length < 5 {
            length = 5
        }
        else if length > 128 {
            length = 128
        }

        break length;
    };

    // Use numbers?
    feature_flags[0] = loop {
        clear_screen();
        println!("YES/NO: Should the password contain numbers (0-9)?");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer = buffer.to_lowercase();
        let input = buffer.trim();

        feature_flags[0] = match input {
            "y"     => true,
            "yes"   => true,
            "true"  => true,
            "n"     => false,
            "no"    => false,
            "false" => false,
            _       => continue,
        };

        break feature_flags[0];
    };

    // Use capitol letters?
    feature_flags[1] = loop {
        clear_screen();
        println!("YES/NO: Should the password contain CAPITAL LETTERS?");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer = buffer.to_lowercase();
        let input = buffer.trim();

        feature_flags[1] = match input {
            "y"     => true,
            "yes"   => true,
            "true"  => true,
            "n"     => false,
            "no"    => false,
            "false" => false,
            _       => continue,
        };

        break feature_flags[1]
    };

    // Use special characters?
    feature_flags[2] = loop {
        clear_screen();
        println!("YES/NO: Should the password contain special characters (@, %, !, etc)?");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let buffer = buffer.to_lowercase();
        let input = buffer.trim();

        feature_flags[2] = match input {
            "y"     => true,
            "yes"   => true,
            "true"  => true,
            "n"     => false,
            "no"    => false,
            "false" => false,
            _       => continue,
        };

        break feature_flags[2];
    };

    let password: String = generate_password(length, feature_flags).into_iter().collect();

    clear_screen();
    println!("Length: {length}");
    println!("Using numbers (0-9): {}", feature_flags[0]);
    println!("Using CAPTIAL LETTERS: {}", feature_flags[1]);
    println!("Using special characters (@, %, !, etc): {}", feature_flags[2]);
    println!("Password: {password}");
}

fn generate_password(length: u8, feature_flags: [bool; 3]) -> Vec<char> {
    let lowercase_range = 'a'..='z';
    let numbers_range = '0'..='9';
    let uppercase_range = 'A'..='Z';
    let special_range = "!@#$%^&*()_+-=[]{}|;':,./<>?".chars();
    let mut password: Vec<char> = Vec::new();
    let mut chars_pool:Vec<char> = lowercase_range.collect();

    if feature_flags[0] {
        chars_pool.extend(numbers_range);  
    }
    if feature_flags[1] {
        chars_pool.extend(uppercase_range);  
    }
    if feature_flags[2] {
        chars_pool.extend(special_range);  
    }

    let mut i = 0;
    while i < length {
        let character: char = *chars_pool.choose(&mut rand::thread_rng()).unwrap();
        password.push(character);
        i += 1;
    }

    password
}


fn clear_screen() {
    ClearScreen::default()
        .clear()
        .expect("failed to clear the screen");
}