pub fn main() {
	let mut count = 0;
	//loop{
		//count +=1;
		//println!("{}", count);
	//	if count == 20 {
			//break;
		//}

	//}

	// while loop fizzbuz
	while count <= 100 {
		if count % 15 == 0 {
			println!("fizzbuzz")
		} else if count % 5 == 0 {
			println!("buzz")
		} else if count % 3 == 0 {
			println!("fizz")
		}
		else {
			println!("{}", count)
		}
		count += 1;
	}
	// for range

	for x in 0..100{
		if x % 15 == 0 {
			println!("fizzbuzz")
		} else if x % 5 == 0 {
			println!("buzz")
		} else if x % 3 == 0 {
			println!("fizz")
		}
		else {
			println!("{}", x)
		}
	}

	}
