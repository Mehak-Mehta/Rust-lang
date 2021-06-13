#![allow(non_snake_case)]
pub fn run() {
	let age: i8 = 21;
	let check: bool = true;

	if age >= 21{
		println!("{} = elegible", age);
	} else if age < 21 && check{
		println!("{} leave", age)
	}
	else {
		println!("{} = not elegible", age);
	}

	let isOfAge = if age >= 21 {true} else {false};
	println!("{}", isOfAge)
}