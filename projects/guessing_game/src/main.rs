use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess which number I'm thinking of!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Input a number from 1-100.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Line read failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // let a = [0; 5];
        // println!("{}", a[6]);
        // Errors at compile time
    }
}
