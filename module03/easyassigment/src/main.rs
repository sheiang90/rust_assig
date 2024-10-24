use std::io::{self, Write}; //remove Read cause is causing an unuse error

struct Person {
	name: String,
	age: u32,
}

fn reading_from_console() {
	let mut buffer = String::new();
	
	print!("What's your name? ");	
	io::stdout().flush().unwrap();//print txt is flush to the terminal
	io::stdin().read_line(&mut buffer).unwrap(); //pass buffer as mutable
	let name = buffer.trim().to_string(); //convert input to string
	buffer.clear(); //clear buffer so we can resuse it

	print!("How old are you? ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut buffer).unwrap();
	let age = buffer.trim().parse().unwrap();	

	let person = Person { name, age};
	println!("Hi {}, you are {} years old!", person.name, person.age);
}
//main function so we can run it
fn main(){
	reading_from_console();
}
