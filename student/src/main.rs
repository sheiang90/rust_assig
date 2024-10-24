mod student;
use student::Student;

fn main(){
	let mut student = Student::create_student("Sheila", "Computer Science");
	student.change_name("Shei Angeles");
	student.introduce_yourself();

	student.change_major("Computer Science");
	student.introduce_yourself();
}
