use std::cmp::Ordering;
use rand::Rng;
use std::io::stdin;
use std::io;

fn main() {
    let mut input = String::with_capacity(100);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let mut attempt = 0;

    loop{
        println!("Please input your guess between 1 and 100: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        attempt = attempt+1;

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Guess higher!"),
            Ordering::Greater => println!("Guess lower!"),
            Ordering::Equal => {
                println!("You win! In {attempt} attempts");
                println!("Press [enter] to close");      
                stdin().read_line(&mut input).unwrap(); 
                break;                
            }
        }
    }
}
