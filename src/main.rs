use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess");

    let mut guess = String::new();

    let mut secret_number = rand::thread_rng().gen_range(1.. 101);

    println!("The Secret Number is: {}", secret_number);

    
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win"),
    }
}
