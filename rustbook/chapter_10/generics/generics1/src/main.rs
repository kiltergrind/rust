
fn main() {
    let somelist = vec![100, 32, 11, 52, 1];
    let largest = find_largest(&somelist);

    println!("The largest one is {}", largest);
}




fn find_largest<T: PartialOrd>(list: &[T]) -> &T {

	let mut largest: &T = &list[0];

	for item in &list[1..] {
		// here will be an error
		// cause generic type can be basically any type,
		// including that one 
		// that does not implement comparsion trait
		// So we need to add a type restriction	
		if item > largest {
			largest = item;
		}
	}

    largest
}