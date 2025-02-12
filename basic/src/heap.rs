
pub fn test1 () {
	println!("\n\nHeap");

	let n1 = Box::new(1);
	let n1 = Box::new(2);
	test2();
	println!();
}

fn test2() {
	let n3 = Box::new(3);
	println!();
}

// สร้าง stack ที่จัดเก็บ Heap address ไว้ 
// Heap เป็นตัวจัดการ Memory ที่ยืดหยุ่น ไม่ตายตัว