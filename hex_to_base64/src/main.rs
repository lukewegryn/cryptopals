extern crate base64;

pub fn hex_string_to_hex(hex_string: String) -> Vec<u8> {
	let mut bytes = Vec::new();
	for i in 0..(hex_string.len()/2) {
		let res = u8::from_str_radix(&hex_string[2*i .. 2*i+2],16);
		match res {
			Ok(v) => bytes.push(v),
			Err(v) => println!("Problem with hex input: {}", v),
		}
	}

	bytes
}

pub fn hex_to_base64_string(hex: &Vec<u8>) -> String {
	base64::encode(&hex)
}

fn main() {
	println!("Enter a hex value:");

	let mut hex_string = String::new();

	std::io::stdin().read_line(&mut hex_string)
		.expect("Unable to read line");

	let bytes = hex_string_to_hex(hex_string);

	println!("{}", hex_to_base64_string(&bytes));
    
}
