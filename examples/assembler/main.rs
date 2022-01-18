use std::{collections::HashMap, hash::Hash, io::Write};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../examples/assembler/fasm.pest"]
struct Assembler;

#[derive(Clone, Debug)]
enum Byte {
	Byte(u8),
	Label(String),
	PaddingForLabel,
}

fn main() {
	let file = std::env::args().nth(1).unwrap();
	let file = std::fs::read_to_string(file).unwrap();
	let file = Assembler::parse(Rule::FILE, &file).unwrap();
	eprintln!("{:#?}", file);
	let mut code: Vec<Byte> = Vec::new();
	let mut known_labels: HashMap<String, u64> = HashMap::new();
	eprintln!("Codegen");
	for i in file {
		match i.as_rule() {
			Rule::OPERATION => {
				let mut inner = i.into_inner();
				let op = inner.next().unwrap().as_span().as_str();
				let op = mnemonic_to_hex(op);
				code.push(Byte::Byte(op));
				for arg in inner {
					match arg.as_rule() {
						Rule::DEC | Rule::HEX => {
							let bytes = literal_to_bytes(arg);
							for i in bytes {
								code.push(Byte::Byte(i));
							}
						}
						Rule::REF => {
							let label = arg.into_inner().next().unwrap().as_span().as_str();
							code.push(Byte::Label(label.to_string()));
							code.append(&mut vec![Byte::PaddingForLabel; 7]);
						}
						_ => unreachable!(),
					}
				}
			}
			Rule::DATA => {
				for i in i.into_inner() {
					let bytes = literal_to_bytes(i);
					for i in bytes {
						code.push(Byte::Byte(i));
					}
				}
			}
			Rule::LABEL => {
				known_labels.insert(i.as_span().as_str().to_string(), code.len() as u64);
			}
			Rule::COMMENT => {}
			_ => {}
		}
	}
	eprintln!("Resolve labels");
	for i in 0..code.len() {
		let byte = &mut code[i];
		if let Byte::Label(label) = byte {
			if let Some(addr) = known_labels.get(label) {
				let mut addr_bytes = addr.to_be_bytes();
				addr_bytes.reverse();
				for offset in 0..8 {
					code[i + offset] = Byte::Byte(addr_bytes[offset]);
				}
			} else {
				panic!("undefined label: {}", label);
			}
		}
	}
	eprintln!("Reduce to bytes");
	let code = code
		.iter()
		.map(|b| match b {
			Byte::Byte(b) => *b,
			_ => unreachable!(),
		})
		.collect::<Vec<u8>>();
	eprintln!("Bytecode: {:x?}", code);
	eprintln!("Write assembled code");
	std::io::stdout().lock().write(code.as_slice()).unwrap();
}

fn mnemonic_to_hex(mnemonic: &str) -> u8 {
	match mnemonic {
		"LD" => 0x00,
		"SV" => 0x01,
		"MV" => 0x02,
		"CP" => 0x03,
		"INC" => 0x04,
		"DEC" => 0x05,
		"ADD" => 0x06,
		"ADS" => 0x07,
		"ADF" => 0x08,
		"MUL" => 0x09,
		"MLS" => 0x0A,
		"MLF" => 0x0B,
		"DIV" => 0x0C,
		"DVS" => 0x0D,
		"DVF" => 0x0E,
		"SUB" => 0x0F,
		"SBS" => 0x10,
		"SBF" => 0x11,
		"FLG" => 0x12,
		"CLR" => 0x13,
		"SR" => 0x14,
		"SL" => 0x15,
		"OR" => 0x16,
		"AND" => 0x17,
		"XOR" => 0x18,
		"NOT" => 0x19,
		"JMZ" => 0x1A,
		"JNZ" => 0x1B,
		"JMP" => 0x1C,
		"PUSH" => 0x1D,
		"POP" => 0x1E,
		"CAL" => 0x1F,
		"RET" => 0x20,
		_ => panic!("Unknown mnemonic {}", mnemonic),
	}
}

fn literal_to_bytes(literal: Pair<Rule>) -> Vec<u8> {
	let num = match literal.as_rule() {
		Rule::HEX => {
			let mut inner = literal.into_inner();
			let hex = inner.next().unwrap().as_span().as_str();
			let len: usize = inner
				.next()
				.unwrap()
				.as_span()
				.as_str()
				.parse()
				.expect("failed to parse len of hex literal");
			let hex = u128::from_str_radix(hex, 16).expect("failed to parse hex literal");
			(hex, len)
		}
		Rule::DEC => {
			let mut inner = literal.into_inner();
			let num = inner.next().unwrap().as_span().as_str();
			let num: u128 = num.parse().expect("failed to parse decimal literal");
			let len: usize = inner
				.next()
				.unwrap()
				.as_span()
				.as_str()
				.parse()
				.expect("failed to parse len of decimal literal");
			(num, len)
		}
		Rule::REF => panic!("ref not allowed in data section"),
		_ => panic!("unexpected token in data section"),
	};
	let mut bytes = num.0.to_be_bytes().to_vec();
	bytes.reverse();
	while bytes.len() > num.1 {
		bytes.pop();
	}
	while bytes.len() < num.1 {
		bytes.push(0);
	}
	bytes.reverse();
	bytes
}
