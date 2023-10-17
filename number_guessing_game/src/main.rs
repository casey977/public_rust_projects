use std:env;
use std::io;

fn main() {
	let cmd_args:Vec<String> = env::args().collect();
	let mut input = String::new();

	loop {
		io::stdin()
			.read_line(&mut input)
			.expect("ERROR: Failed to read line!");

		println!("Guess a number between 0 and 100 (both inclusive): ");
		let number:i8 = match input.trim().parse()
	}
}

