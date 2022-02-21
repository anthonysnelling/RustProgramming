use std::io;
use rand::Rng;
// bring in the enum variants for comparing less, equal, and greater
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // println!("The secret number is {}", secret_number);

        println!("Please input your guess.");

        // mut makes a variable mutable, by default variables are immutable in rust, meaning they dont change. 
        // :: is an associated function operator, or we are using a function that is implemented on a type
        // in this case we are new, which makes a new empty string.
        let mut guess = String::new();

        io::stdin()
            // read in user input
            // & makes a reference to the guess variable, so you can access a piece of data
            // without needing to copy data in memory multiple times.
            // reference are also immutable by default so we put &mut guess instead of &guess
            .read_line(&mut guess)
            //.expect tells the compiler to when a fault happens to crash the program and print this message
            .expect("Failed to read Line");

        //this 'shadows' guess var that is made by parsing guess' string into an unsigned 32 bit integer
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} can be used for string interpolation
        println!("You guessed: {}", guess);

        //cmp() compares two values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
