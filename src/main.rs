use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("--- Guess the number - Rust edition ---");

    let random_number = rand::thread_rng().gen_range(1..101);

    //println!("The random number is: {}", random_number);

    loop {
        println!("Please enter your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
