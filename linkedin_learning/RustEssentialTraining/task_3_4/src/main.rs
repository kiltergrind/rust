
fn main() {
    let cel = 23;
    println!("Cel: {} \nFar: {}", cel, cels_too_far(cel));
}


fn cels_too_far(c: impl Into<f64>) -> f64 {
    1.8 * c.into() + 32.0
}

#[test]
fn test_conversion() {
    assert_eq!(cels_too_far(23), 73.4);
}