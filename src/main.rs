use rand::Rng;
use std::io;

fn main() {
    println!("Please input your guess");

    let mut guess = String::new();

    let mut secret_number = rand::thread_rng().gen_range(1.. 101);

    println!("The Secret Number is: {}", secret_number);
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
