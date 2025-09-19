use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    let num1: i32 = rng.gen_range(1..=10);

    let mut input = String::new();
    
    println!("Guess a number between 1 and 10:");
    io::stdin().read_line(&mut input)
    .expect("failed to read input");


    let input_num: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
   

    if num1 == input_num {
        println!("You win!");
    } else {
        println!("Try again! The number was {}", num1);
    }
}
