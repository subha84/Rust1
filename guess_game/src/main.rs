use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Crash the Program");

        let guess:u32 = guess.trim().parse().expect("PROGRAM CRASHED!");
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big" ),
            Ordering::Less => println!( "Too small "),
            Ordering::Equal => {
                println!(" Jackpot ");
                break;
            }
        }
    }
}
