/* YOUR CODE GOES HERE */

fn sum_boxes<T>(a: Box<T>, b: Box<T>) -> Box<T>
where
    T: PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>

{
    let result: T;

    if a < b {
        result = *a + *b
    } else {
        result = *a - *b
    }

    Box::new(result)
}


fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);
    
    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!((*sum_boxes(pi, e) * 10000.0) as usize as f64 / 10000.0, 0.4233);
    
    println!("Tests passed!");
}