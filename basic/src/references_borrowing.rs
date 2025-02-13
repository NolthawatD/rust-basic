#![warn(dropping_references)]

pub fn test1() {
	println!("\n\nReference and Borrowing");

	let s1 = String::from("abc");
	let s2 = &s1; // Borrowing s1 by reference from s1
	println!("{}", s2);
	println!("{}", s1); // จะสามารถ print ได้ เมื่อถูก reference เท่านั้น
}
