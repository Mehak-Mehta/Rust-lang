#![allow(non_snake_case)]
struct Color {
	red: u8,
	blue: u8,
	green : u8

}

struct color(u8,u8,u8);

struct Person {
	firstName: String,
	lastName : String

}

impl Person {
	fn name(first:&str, last: &str) -> Person{
		Person{
			firstName: first.to_string(),
			lastName: last.to_string()
		}
	}

	fn getname(&self) -> String{
		format!("{} {}", self.firstName, self.lastName)
	}

	fn setlastname(&mut self, last: &str) {
		self.lastName = last.to_string();
	}
}

pub fn run() {
	let mut c = Color {
		red : 233,
		blue : 0,
		green : 0
	};

	c.green = 23;
	println!("{} {} {} ", c.red, c.blue, c.green);

	let mut co = color(255,0,0);
	co.1 = 25;
	println!("{} {} {} ", co.0, co.1, co.2);

	let mut p = Person::name("koro", "sensei");
	println!("{}", p.getname());

	p.setlastname("reaper");
	println!("{}", p.getname());

}