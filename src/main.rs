use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let mut rng = rand::rng();
    let secert_number: u32 = rng.random_range(1..=100);
    loop {
        println!("Your guess number: (type 'quit' to exit)");
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "quit" {
            println!("You quit the game!");
            break;
        }
        // parse the guess number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // compare the guess number with the secert number
        match guess.cmp(&secert_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
