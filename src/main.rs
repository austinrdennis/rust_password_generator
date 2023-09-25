use rand::Rng;
use clearscreen::ClearScreen;

fn main() {
    clear();
    let password: String = generate_password(12).into_iter().collect();
    println!("{}", password);

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

fn clear() {
    ClearScreen::default().clear().expect("failed to clear the screen");
}