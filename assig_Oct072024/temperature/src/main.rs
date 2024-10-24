const F_POINT: f64 = 32.0;

fn f_to_c(f: f64) -> f64{
	(f- F_POINT) * 5.0 / 9.0
}
#[allow(dead_code)]
fn c_to_f(c: f64) -> f64{
	(c * 9.0 / 5.0) + F_POINT
} 

fn main() {
	let mut temp_f: f64 = 50.0;
	let mut temp_c = f_to_c (temp_f);
	println!("{:.0} F° is {:.0} C°", temp_f, temp_c);
	
	let mut count = 0;
	loop{
		temp_f += 1.0;
		temp_c = f_to_c(temp_f);
		println!("{:.0} F° is {:.0} C°", temp_f, temp_c);

		count += 1;
		if count >= 5 {
			break;
		}
	}
}
