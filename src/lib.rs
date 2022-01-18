use std::io::Read;

#[non_exhaustive]
pub struct VM<'a, R, W>
where
	R: FnMut(u64) -> u8,
	W: FnMut(u64, u8) -> (),
{
	pub stack: Vec<u64>,
	pub reader: &'a mut R,
	pub writer: &'a mut W,
	pub reg: [u8; 64],
	pub arithmetic_register: u8,
	pub instruction_pointer: u64,
}
impl<'a, W, R> VM<'a, R, W>
where
	R: FnMut(u64) -> u8,
	W: FnMut(u64, u8) -> (),
{
	pub fn new(reader: &'a mut R, writer: &'a mut W) -> VM<'a, R, W> {
		VM {
			stack: Vec::new(),
			reader,
			writer,
			reg: [0; 64],
			arithmetic_register: 0,
			instruction_pointer: 0,
		}
	}
	pub fn tick(&mut self) {
		let opcode = self.read(self.instruction_pointer, 1)[0];
		match opcode {
			0x00 => {
				let from = self.read_8(self.instruction_pointer + 1);
				let to = self.read(self.instruction_pointer + 9, 1)[0];
				self.reg[to as usize] = self.read(from, 1)[0];
				self.instruction_pointer += 10;
			}
			0x01 => {
				let from = self.read(self.instruction_pointer + 1, 1)[0];
				let to = self.read_8(self.instruction_pointer + 2);
				(self.writer)(to, self.reg[from as usize]);
				self.instruction_pointer += 10;
			}
			0x02 => {
				let from = self.read(self.instruction_pointer + 1, 1)[0];
				let to = self.read(self.instruction_pointer + 2, 1)[0];
				self.reg[to as usize] = self.reg[from as usize];
				self.instruction_pointer += 3;
			}
			0x03 => {
				let from = self.read_8(self.instruction_pointer + 1);
				let to = self.read_8(self.instruction_pointer + 9);
				if let [size, step] = self.read(self.instruction_pointer + 17, 2)[0..2] {
					let mut source = from;
					let mut pointer = to;
					for _ in 0..size {
						let src = self.read(source, 1)[0];
						(self.writer)(pointer, src);
						pointer += step as u64;
						source += 1;
					}
				};
				self.instruction_pointer += 19;
			}
			0x1C => {
				let target = self.read_8(self.instruction_pointer + 1);
				self.instruction_pointer = target
			}
			_ => unimplemented!("Opcode {:x}", opcode),
		}
	}
	pub fn read(&mut self, addr: u64, length: u64) -> Vec<u8> {
		(0..length).map(|o| (self.reader)(addr + o)).collect()
	}
	pub fn read_8(&mut self, addr: u64) -> u64 {
		let from = self.read(addr, 8);
		let mut from_ = [0; 8];
		from.as_slice().read_exact(&mut from_).unwrap();
		let from = from_;
		drop(from_);
		u64::from_be_bytes(from)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
