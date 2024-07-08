//! Check if a file contains a specific name

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};


fn main() {
    let mut args = env::args().skip(1).take(2);

    if args.len() != 2 {
        println!("Provide exactly two arguments: <filename> <name>");
        return;
    }
    
    let (filename, search_name) = (args.next().unwrap(), args.next().unwrap());
    let mut file = File::options().read(true).append(true).open(&filename)
        .expect(&format!("Cannot open {}", &filename));
    let mut linesbuf = BufReader::new(&file).lines();
    let mut found_at_pos = 1;

    while let Some(Ok(line)) = linesbuf.next() {
        if line.eq(&search_name) {
            println!(
                "This file contains given item ({}) in the list \
                found at position: {}",
                search_name,
                found_at_pos);
            return;
        }
        found_at_pos += 1;
    }

    println!("This item ({}) not found in list", search_name);


    if let Ok(_) = write!(&file, "\n{}", &search_name) {
        println!("{} - is successfully written to the file", search_name);
    } else {
        eprintln!("Couldn't save >> {} << to the file", search_name);
    }

}
