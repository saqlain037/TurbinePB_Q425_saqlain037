use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number (1..=100)!");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}