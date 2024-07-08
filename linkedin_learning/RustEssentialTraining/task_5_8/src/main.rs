//! Find median of array of numbers
//! 
//! 


use std::cmp::Ordering;


fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    /* YOUR CODE GOES HERE */
    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &value in numbers.iter() {
        mean += value as f64;
        
        // Ofc, there is more code in this approach
        // but still let's pay some tribute to avoiding boring solutions
        match max.cmp(&value) {
            Ordering::Less => max = value,
            _ => match min.cmp(&value) {
                Ordering::Greater => min = value,
                _ => (),
            }
        }
    }
    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
