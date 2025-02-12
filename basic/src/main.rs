// no display warning
#![allow(unused_variables)]

mod ownership;
mod heap;

fn main() {
	println!("Hello, World");
	ownership::test1();
	heap::test1();
}	


// cargo new basic
// cargo check 
// cargo build [build binary file in target]
// cargo run [build and run]
// cargo build --release [optimized code]