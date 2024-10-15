// Primitive data types
// int, float, bool, char

// Interger
// Rust has signed (+ and -) and unsigned
// integer (only+) types of different sizeds.
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: Unsigned integers.

fn main() {
	let x: i32 = -42;
	let y: u64 = 100;
	println!("Signed Interger: {}", x);
	println!("Unsigned Interger: {}", y);

	let e: i32 = 2147483647;
	let i: i64 = 9223372036854775807;
	println!("Maximum value for i32: {}", e);
	println!("Maximum value for i64: {}", i);

	let pi: f64 = 3.14;
	println!("Value of pi: {}", pi);

	// Boolean Values: true, false
	let is_snowing: bool = true;
	println!("Is it snowing ? {}", is_snowing)

	// Character Type - char
	let letter: char = 'a';
	println!("First letter of the alphabet: {}", letter)
}