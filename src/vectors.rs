
use std::mem;
pub fn run() {
	let mut numbers: Vec<i32> = vec![1,2,4,5];

	// reassign 
	numbers[2] = 39;
	println!("{:?}", numbers);

	// get one 
	println!("{}", numbers[3]);
// get Value
	println!("{}", mem::size_of_val(&numbers));
// get ==
	let slice : &[i32] = &numbers[1..3];
	println!("{:?}", slice);

}