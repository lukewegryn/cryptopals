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

pub fn calc_english_score(ref buffer: &String) -> (u32) {
		for byte in buffer.as_bytes() {
				if byte < &32 || byte > &126 {
					return 0;
				}
		}

		let mut score: u32 = 1;

		for byte in buffer.as_bytes() {
			  //ETAOIN SHRDLU cmfwyp vbgkjq xz
				match byte {
					  &32 => score += 27, //SPACE
						&b'E' | &b'e' => score += 26, //E or e
						&b'T' | &b't' => score += 25, //T or t
						&b'A' | &b'a' => score += 24, //A or a
						&b'O' | &b'o' => score += 23, //O or o
						&b'I' | &b'i' => score += 22, //I or i
						&b'N' | &b'n' => score += 21, //N or n
						&b'S' | &b's' => score += 20, //S or s
						&b'H' | &b'h' => score += 19, //H or h
						&b'R' | &b'r' => score += 18, //R or r
						&b'D' | &b'd' => score += 17, //D or d
						&b'L' | &b'l' => score += 16, //L or l
						&b'U' | &b'u' => score += 15, //U or u
						&b'C' | &b'c' => score += 14, //C or c
						&b'M' | &b'm' => score += 13, //M or m
						&b'F' | &b'f' => score += 12, //F or f
						&b'W' | &b'w' => score += 11, //W or w
						&b'Y' | &b'y' => score += 10, //Y or y
						&b'P' | &b'p' => score += 8, //P or p
						&b'V' | &b'v' => score += 8, //V or v
						&b'B' | &b'b' => score += 7, //B or b
						&b'G' | &b'g' => score += 6, //G or g
						&b'K' | &b'k' => score += 5, //K or k
						&b'J' | &b'j' => score += 4, //J or j
						&b'Q' | &b'q' => score += 3, //Q or q
						&b'X' | &b'x' => score += 2, //X or x
						&b'Z' | &b'z' => score += 1, //Z or z
						_ => score += 0 //everything else
				}
		}

		return score;
}

fn main() {
	  let mut hex_string_a = String::new();
	  let mut key: u8 = 0;

	  println!("Enter encoded hex string:");
    get_line(&mut hex_string_a);
    hex_string_a.pop();

    let hex_a = hex_string_to_hex(hex_string_a);
    let mut decrypted_value = String::new();
    let mut decrypted_score = 0;

    while key < std::u8::MAX {
    	let mut xored_bytes = Vec::new();
			for i in 0..hex_a.len() {
				let res = &hex_a[i] ^ key;
				xored_bytes.push(res);
			}
		  let xored_string = String::from_utf8(xored_bytes);
		  if !xored_string.is_err() {
		  	//println!("hex_string_a ^ {}: {}", key, xored_string.unwrap());
		  	let xored_string_unwrap = xored_string.unwrap();
		  	let score = calc_english_score(&xored_string_unwrap);
		  	if score > 0 {
		  		  if score > decrypted_score {
		  		  		decrypted_score = score;
		  		  		decrypted_value = xored_string_unwrap;
		  		  }
		  			//println!("hex_string_a ^ {}: {} --> {}", key, xored_string_unwrap, score);
		  	}
		  }
    	key += 1;
    }
    println!("{}", decrypted_value) 
}