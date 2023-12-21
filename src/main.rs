use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    let mut secret_number = rand::thread_rng().gen_range(1..101);

    println!("The Secret Number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}","You Win".yellow());
                break;
            }
        }
    }
}
