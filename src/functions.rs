#![allow(non_snake_case)]
pub fn run() {
	greetings("hello","koro");
	let getSum = sum(2, 4);
	println!("{}", getSum);

	let num3:i32 = 10;
	let add_n = |num1:i32, num2:i32| num1 + num2 + num3;
	println!("{}", add_n(3,3))
}

fn greetings(greet:&str, name:&str){
	println!("{}, {}", greet, name);
}

fn sum(num1: i32, num2 : i32) -> i32 {
	num1 + num2
}