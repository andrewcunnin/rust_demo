use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to my sample guessing game!");
    const MIN_NUM: u32 = 1; // restrict to u16 because nobody wants to guess a number larger than that
    const MAX_NUM: u32 = 100;
    let mut name = String::new();
    let secret_number = rand::thread_rng()
        .gen_range(MIN_NUM, MAX_NUM+1);

    println!("What is your name?");
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    println!("Selected a random number between {} and {}", MIN_NUM, MAX_NUM);
    loop{
        println!("Please input your guess");

        let mut guess = String::new(); //shadow to convert to num later

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations, {}, you win!", name);
                break;
            }
        }
    }
}
