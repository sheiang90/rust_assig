pub struct Student {
	name: String,
	major: String,
}

impl Student {

	pub fn create_student (name: &str, major: &str) -> Student {
		Student{
			name: name.to_string(),
			major: major.to_string()
		}
	}

	pub fn change_name (&mut self, new_name: &str) {
		self.name = new_name.to_string();
	}

	pub fn change_major (&mut self, new_major: &str){
		self.major = new_major.to_string();
	}

	pub fn introduce_yourself(&self){
		println!("Hello, my name is {}, and my major is {}!", self.name, self.major)
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let s = Student::create_student("Shei Angeles", "Computer Science");
        assert_eq!(s.name, "Sheila Angeles".to_string());
		assert_eq!(s.major, "Computer Science".to_string());
    }

	#[test]
    fn test_change_major() {
        let mut s = Student::create_student("Shei Angeles", "Computer Science");
        
		s.change_major("CS");

		assert_eq!(s.major, "CS".to_string());
    }

}
