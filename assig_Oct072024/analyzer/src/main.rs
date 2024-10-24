fn is_even(n: i32) -> bool{
	n % 2 == 0
}

fn main(){
	let numbers: [i32;10] = [3,9,1,15,23,10,55,8,100,7];
	
	for &num in numbers.iter(){
		if num % 3 == 0 && num % 5 == 0 {
			println!("{} FizzBuzz", num);
		}
		else if num % 3 == 0 {
			println!("{} Fizz", num);
		}
		else if num % 5 == 0 {
			println!("{} Buzz", num);
		}
		else if is_even(num) {
			println!("{} Even", num);
		}
		else{
			println!("{} Odd", num);
		}
	}
	
	let mut sum = 0; 
	let mut index_w = 0;

	while index_w < numbers.len(){
		sum += numbers[index_w];
		index_w += 1;
	}
	println!();
	println!("Sum of all number in the array: {}", sum);

	let mut largest = numbers[0];
	let mut i = 1;
	while i < numbers.len() {
		if numbers[i] > largest {
			largest = numbers[i];
		}
		i +=1;
	}
	println!("The largest array number is: {} ", largest);
}

