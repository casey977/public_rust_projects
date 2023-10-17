use std::env;
use std::io;
use rand::Rng;
use std::process.exit();

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
				if (number >= 0 && number <= 100) {
					if (number > random_number) {
						eprintln!("Your guess is too high!");
					} else if (number < random_number) {
						eprintln!("Your guess is too low!");
					} else {
						eprintln!("You guessed correct!");
						exit(0);
					}
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

