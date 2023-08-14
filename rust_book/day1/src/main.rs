use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("Hello, world!");

// generate a random number

    // let secret_num = rand::thread_rng().gen_range(1, 101);
    // q: why is this not working?
    
    let secret_num = rand::thread_rng().gen_range(1..101);




    
    loop{
        println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(guess) => guess,
        Err(_) => continue,
    };

    match guess.cmp(&secret_num){
        Ordering::Less => println!("{}","Too small!".red()),
        Ordering::Greater => println!("{}","Too big!".red()),
        Ordering::Equal =>{ 
            println!("{}","10
            You win!".green());
            break;
        }
    }
    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_num);
    }
}
