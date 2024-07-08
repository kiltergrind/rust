use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(0..=100);
    let mut guess = String::new();
    

    loop {
        println!("Please type your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Error happened when trying to read from stdin");
        
        
        match guess.trim().parse::<u32>() {
            Ok(g) => {
                println!("----------------");
                match g.cmp(&random_number) {
                    Ordering::Less => println!("Too low"),
                    Ordering::Greater => println!("Too high"),
                    Ordering::Equal => {
                        println!("Congratulations");
                        break;
                    },
                };
                println!("----------------\n");
            }
            Err(_) => println!("\nONLLY WHOLE NUMBER FORMAT IS ACCEPTED\n"),
        };
        
        guess.clear(); // Clear string buffer
    }

}