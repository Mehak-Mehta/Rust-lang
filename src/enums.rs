enum movement{
	Up,
	Down,
	Left,
	Right
}

fn moveavatar(m :movement) {
  match m {
			movement::Up => println!("moving up"),
			movement::Right => println!("moving up"),
			movement::Left => println!("moving up"),
			movement::Down => println!("moving up"),
		}
}
pub fn run() {
   let avatar1 = movement::Left;
   let avatar2 = movement::Right;
   let avatar3 = movement::Up;
   let avatar4 = movement::Down;

			moveavatar(avatar1);
			moveavatar(avatar2);
			moveavatar(avatar3);
			moveavatar(avatar4);
}