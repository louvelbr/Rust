use std::io; //library (input/output)
use std::cmp::Ordering; //to use Less, Greater, Equal
use rand::Rng; //to use rand

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); //we choose the rng methode

    //println!("The secret number is: {}", secret_number);

loop{
    println!("Please input your guess.");

    //creates a new empty string
    let mut guess = String::new(); //mutable variable
    /*let mut guess = String::new(); -> immutable variable*/

    io::stdin().read_line(&mut guess) //stdin is use we io library
        .expect("Failed to read line");

    //trim remove : \n; parse: change string into number; u32: number type
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { //cmp : compare numbers
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
}
