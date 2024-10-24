fn most_frequent_word(text: &str) -> (String, usize) {

	let words: Vec<&str> = text.split_whitespace().collect();
	
	let mut max_count = 0;
	let mut max_word = String::new();
	let mut count: usize;

	for word in &words {
		count = 0;
		for other_word in &words {
			if word == other_word {
				count += 1;
			}
		}

		if count > max_count {
			max_count = count;
			max_word = word.to_string();
		}
	}

	(max_word, max_count)
}

fn main () {
	let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
	let (word, count) = most_frequent_word(text);
	println!("Most frequent word: \"{}\" ({} times)", word, count); 
}
