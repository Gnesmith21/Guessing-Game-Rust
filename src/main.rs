use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, please select a number!");
    loop {
        println!("Please enter the number");

        let mut guess = String::new();
        let the_number = rand::thread_rng().gen_range(1..=100);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line\r\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed : {guess} ");
        match guess.cmp(&the_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
