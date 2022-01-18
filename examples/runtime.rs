use std::{
	io::{Read, Write},
	sync::{Arc, Mutex},
};

fn main() {
	let file = std::env::args().nth(1).unwrap();
	let file = std::fs::read(file).unwrap();
	let mmap_ = Arc::new(Mutex::new([0u8; 65536]));
	let mut mmap = mmap_.lock().unwrap();
	file.iter().enumerate().for_each(|(i, b)| mmap[i] = *b);
	drop(mmap);
	let mmap = mmap_.clone();
	let mut write = |addr: u64, val: u8| {
		let addr = addr as u16;
		// Protect ROM
		if addr < 0x8000 {
			println!("Attempt to write to ROM at {:x}", addr);
			return;
		}
		// Console register
		if addr == 0x8000 {
			print!("{}", val as char);
			std::io::stdout().lock().flush().unwrap();
		}
		mmap.lock().unwrap()[addr as usize] = val;
	};
	let mmap = mmap_.clone();
	let mut read = |addr: u64| {
		if addr == 0x8000 {
			let mut buf = [0u8];
			std::io::stdin().lock().read(&mut buf).unwrap();
			return buf[0];
		}
		mmap.lock().unwrap()[addr as usize]
	};
	let mut vm = fantasm::VM::new(&mut read, &mut write);
	vm.instruction_pointer = 0;
	loop {
		vm.tick();
	}
}
