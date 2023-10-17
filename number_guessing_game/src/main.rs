use std::env;
use std::io;

fn main() {
	let cmd_args:Vec<String> = env::args().collect();
	let mut input = String::new();
	let mut number:i8 = 99;

	loop {
		println!("Guess a number between 0 and 100 (both inclusive): ");

		io::stdin()
			.read_line(&mut input)
			.expect("ERROR: Failed to read line!");
		
		match input.trim().parse::<i8>() {
			Ok(parsed_num) => {
				number = parsed_num;
				break;
			},
			Err(_) => {
				eprintln!("Invalid input!");
				continue;
			}
		};
	}

	println!("You entered: {}", number);
}

