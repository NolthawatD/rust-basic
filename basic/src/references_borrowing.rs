#![warn(dropping_references)]

pub fn test1() {
	println!("\n\nReference and Borrowing");

	let s1 = String::from("abc");
	let s2 = &s1; //Borrowing s1 by reference from s1
	// drop(s2);
	drop(s1); // จะสามารถ drop ได้ เมื่อ reference เท่านั้น
}
