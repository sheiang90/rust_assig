use std::io::{self, Write};
fn check_guess (guess: i32, secret: i32) -> i32 {
	if guess == secret {
		0 
	}
	else if guess > secret {
		1
	}
	else {
		-1
	}
}

fn main () {
	let secret_number: i32 = 32; //is not mutable cause is hard-core & doesnt change
	let mut guess_count: i32 = 0;
	let mut guess: i32;
	let mut input = String::new();

	loop {
		guess_count += 1;

		println!("Enter you guess #: ");
		io::stdout().flush().unwrap();
		input.clear();
		io::stdin().read_line(&mut input).unwrap();

		guess = input.trim().parse().unwrap();
		
		let result = check_guess(guess, secret_number);

		if result == 0{
			println!("You guessed the secret number: {}", guess);
			break;
		}
		else if result == 1 {
			println!("{} is too high", guess);
		}
		else{
			println!("{} is too low", guess);
		}
	}

	println!("It took you {} guesses.", guess_count);
}
