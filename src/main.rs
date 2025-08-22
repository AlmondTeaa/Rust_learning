use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Guessing Game");
    println!("Secret number is {secret_number}");
    loop {
        let mut guess = String::new();
        println!("Please guess a number");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user's input");
        let guess: u32 = guess.trim().parse().expect("Input a number pls");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!!");
                break;}
        }
    }
}