use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        
        io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read your guess");
        let guess: u32 = match guess.trim().parse()
           {
               Ok(num) => num,
               Err(_) => continue,
           };//.expect("Please type a number!");
        println!("your guess: {}", guess);

        println!("The Secret: {}", secret_number);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } ,
        }
    }
}

// mut => Mutable, this variable can be changed after creation
// let mut apples =5; Mutable
// let apples = 5; Immutable

