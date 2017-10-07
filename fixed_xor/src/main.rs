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

pub fn hex_xor(hex_a: Vec<u8>, hex_b: Vec<u8>) -> Vec<u8>{
	assert!(hex_a.len() == hex_b.len());
	let mut xored_bytes = Vec::new();
	for i in 0..hex_a.len() {
		let res = &hex_a[i] ^ &hex_b[i];
		xored_bytes.push(res);
	}
	xored_bytes
}

pub fn get_line(mut buf: &mut String) {
	std::io::stdin().read_line(&mut buf)
    	.expect("Unable to read line.");
} 

pub fn pretty_print_hex_vec(hex: Vec<u8>) {
	let mut hex_string = String::new();
	for i in 0..hex.len(){
		hex_string = hex_string + &format!("{:x}",&hex[i]);
	}
	println!("{}",hex_string);
}

pub fn hex_vec_to_string(hex: Vec<u8>) -> String {
	let mut hex_string = String::new();
	for i in 0..hex.len(){
		hex_string = hex_string + &format!("{:x}",&hex[i]);
	}
	hex_string
}

fn main() {
	  let mut hex_string_a = String::new();
	  let mut hex_string_b = String::new();

	  println!("Enter hex_string_a:");
    get_line(&mut hex_string_a);

    println!("Enter hex_string_b:");
    get_line(&mut hex_string_b);

    hex_string_a.pop();
    hex_string_b.pop();
    
    println!("hex_string_a: {}\nhex_string_b: {}", hex_string_a, hex_string_b);


    //hex_string_a = "1c0111001f010100061a024b53535009181c".to_string();
    //hex_string_b = "686974207468652062756c6c277320657965".to_string();

    assert!(hex_string_a.len() == hex_string_b.len());

    println!("hex_string_a ^ hex_string_b: {}", hex_vec_to_string(hex_xor(hex_string_to_hex(hex_string_a), hex_string_to_hex(hex_string_b))));
}
