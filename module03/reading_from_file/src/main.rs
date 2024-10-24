
use std::io::{self, Read, Write}; 

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

use std::fs::File;
//use std::io::prelude::*;

struct Config {
	username: String,
	api_key: String,
	port: u16,
}

impl Config{
	fn from_file(path: &str) -> Config {
		let mut file = File::open(path).unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();

		let mut lines = contents.lines();
		let username = lines.next().unwrap().to_string();
		let api_key = lines.next().unwrap().parse().unwrap();
		let port = lines.next().unwrap().parse().unwrap();

		Config { username, api_key, port }

	}
}

fn reading_from_file() {
	let config = Config::from_file("config.txt");
	println!("username: {}", config.username);
	println!("api key: {}", config.api_key);
	println!("port: {}", config.port);
}

fn main(){
	reading_from_console();
	reading_from_file();
} 
