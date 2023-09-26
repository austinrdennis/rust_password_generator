use std::io;
use rand::Rng;
use clearscreen::ClearScreen;

fn main() {
    let mut length: u8;
    
    length = loop {
        clear_screen();
        println!("Desired password length? [5-128 characters]");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        length = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if length < 5 {
            length = 5
        }
        else if length > 128 {
            length = 128
        }

        break length;
    };


    let password: String = generate_password(length).into_iter().collect();

    clear_screen();
    println!("Length: {length}");
    println!("Password: {password}");
}

fn generate_password(length: u8) -> Vec<char> {
   let mut password: Vec<char> = Vec::new();

   let mut i = 0;
   while i < length {
        let character: u32 = rand::thread_rng().gen_range(48..=122);
        let character: char = char::from_u32(character).unwrap(); 
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