use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    let num1: i32 = rng.gen_range(1..=10);

    loop{

    let mut input = String::new();
    
    println!("Guess a number between 1 and 10:");
    io::stdin().read_line(&mut input)
    .expect("failed to read input");


    let input_num: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            continue;
        }
    };
   

  if input_num == num1 {
            println!("You win!");
            break; // exit loop
        } else if input_num < num1 {
            println!("Too low! Try again.");
        } else {
            println!("Too high! Try again.");
        }
}
}