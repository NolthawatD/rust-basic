
pub fn test1() {
    let mut n1 = 1;
    let n2 = &mut n1;
    *n2 = 20;
    println!("{}", n2);

	let mut n3 = 10;
	edit(&mut n3);
	println!("{}", n3)
}

fn edit(n3: &mut i32){
	*n3 = 30;
}