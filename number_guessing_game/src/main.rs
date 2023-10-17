use std::env;
use std::io;
use rand::Rng;

fn main() {
	let cmd_args:Vec<String> = env::args().collect();
	let mut input = String::new();
	let mut number:i8 = 99;

	let mut rng = rand::thread_rng();
	let random_number:i8 = rng.gen_range(0..101);

	loop {
		println!("Guess a number between 0 and 100 (both inclusive): ");

		io::stdin()
			.read_line(&mut input)
			.expect("ERROR: Failed to read line!");
		
		match input.trim().parse() {
			Ok(parsed_num) => {
				number = parsed_num;
				if(number >= 0 && number <= 100) {
					break;
				} else {
					continue;
				}
			},
			Err(_) => {
				eprintln!("Invalid input!");
				break;
			}
		};
	}

	println!("You entered: {}", number);
}

