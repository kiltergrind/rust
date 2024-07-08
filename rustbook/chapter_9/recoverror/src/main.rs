use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let greet_file = File::open("grretings.txt");
    let greefile = match greet_file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("/greetings.txt") {
                Ok(fh) => fh,
                Err(_) => panic!("Could not create file"),
            },
            other_error => panic!("Error reading file: {}", other_error),
        }
    };
}