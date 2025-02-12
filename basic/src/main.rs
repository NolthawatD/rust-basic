// no display warning
#![allow(unused_variables)]

fn main() {
	println!("Hello, World");
	let n1 = 1;
    let n2 = 2;
	ex1_1();
	println!();
	println!();
}

fn ex1_1() {
    let n3 = 3;
	ex1_2();
	println!();
	println!();
}

fn ex1_2() {
    let n4 = 4;
	println!();
}

// cargo new basic
// cargo check 
// cargo build [build binary file in target]
// cargo run [build and run]
// cargo build --release [optimized code]