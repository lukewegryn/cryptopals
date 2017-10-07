extern crate edit_distance;
use std::fs::File;
use std::io::Read;
extern crate base64;
use std::collections::HashMap;


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
				if byte < &0 || byte > &128 {
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

pub fn decrypt_string(to_decrypt: String) -> String {
  let mut key: u8 = 0;

  let hex_a = hex_string_to_hex(to_decrypt);
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
	  	//println!("hex_string_a ^ {}: {} --> {}", key, xored_string_unwrap, score);
	  	if score > 0 {
	  			//println!("hex_string_a ^ {}: {} --> {}", key, xored_string_unwrap, score);
	  			if score > decrypted_score {
	  		  		decrypted_score = score;
	  		  		decrypted_value = xored_string_unwrap;
	  		  }
	  	}
	  }
  	key += 1;
  }
  decrypted_value
}

pub fn get_decrypt_key (content: &Vec<u8>) -> u8 {
	let mut key: u8 = 0;
	let mut decrypt_key: u8 = 0;

  let mut decrypted_score = 0;

  while key < std::u8::MAX {
  	let mut xored_bytes = Vec::new();
		for i in 0..content.len() {
			let res = &content[i] ^ key;
			xored_bytes.push(res);
		}
	  let xored_string = String::from_utf8(xored_bytes);
	  if !xored_string.is_err() {
	  	//println!("hex_string_a ^ {}: {}", key, xored_string.unwrap());
	  	let xored_string_unwrap = xored_string.unwrap();
	  	let score = calc_english_score(&xored_string_unwrap);
	  	//println!("hex_string_a ^ {}: {} --> {}", key, xored_string_unwrap, score);
	  	if score > 0 {
	  			//println!("hex_string_a ^ {}: {} --> {}", key, xored_string_unwrap, score);
	  			if score > decrypted_score {
	  		  		decrypted_score = score;
	  		  		decrypt_key = key;
	  		  }
	  	}
	  }
  	key += 1;
  }
  return decrypt_key;
}

pub fn next_key(curr_key: u8) -> u8 {
	let next_key = match curr_key {
		b'I' => b'C',
		b'C' => b'E',
		b'E' => b'I',
		_ => panic!("There was an error")
	};

	next_key
}

pub fn repeating_key_xor(bytes: Vec<u8>, seed_key: u8) -> (Vec<u8>, u8) {
	let mut xored_bytes = Vec::new();
	let mut curr_key = seed_key;
	for i in 0..bytes.len() {
		let res = &bytes[i] ^ curr_key;
		xored_bytes.push(res);
		curr_key = next_key(curr_key);
	}
	(xored_bytes, next_key(curr_key))
}

pub fn hamming_distance(hex_a: Vec<u8>, hex_b: Vec<u8>) -> u32 {
	let xor_res = hex_xor(hex_a, hex_b);
	let mut distance = 0;
	for byte in xor_res {
		distance = distance + byte.count_ones();
	}
	return distance;
}

pub fn get_n_bytes(bytes: &Vec<u8>, lower_n: u32, upper_n: u32) -> Vec<u8> {
	let mut n_bytes = Vec::new();
	for i in lower_n..upper_n {
		n_bytes.push(bytes[(i as usize)])
	}
	return n_bytes;
}

pub fn transpose(contents: &Vec<u8>, block_size: usize) -> HashMap<usize, Vec<u8>> {
	let mut transpose = HashMap::new();
	for i in 0..contents.len() {
		let curr_block = i % block_size;
		if !transpose.contains_key(&curr_block) {
			transpose.insert(curr_block, Vec::new());
		}

		transpose.get_mut(&curr_block).unwrap().push(contents[i])
	}

	//println!("{:?}", transpose);
	return transpose;
}

pub fn crack_xor_key(contents: &Vec<u8>, block_size: usize) -> HashMap<usize, Vec<u8>> {
	let mut transpose = transpose(&contents, block_size);
	assert_eq!(block_size, transpose.len());

	return transpose;
}

fn main() {
		//let test1 = "this is a test".to_string();
		//let test2 = "wokka wokka!!!".to_string();
		let mut key_distance = Vec::new();
		let mut f = File::open("encrypted.txt").expect("Unable to open file");
	 	let mut contents = String::new();
	 	f.read_to_string(&mut contents).expect("Unable to read");
	 	//let decoded_contents = base64::decode(&contents);
		//println!("{}",edit_distance::edit_distance("kitten","sitting"));

		let bytes = contents.into_bytes();
		//println!("{:?}",bytes );

		//let mut smallest_distance: f32 = std::f32::MAX;
		//let mut smallest_key = 2;

		for key_size in 1..40 {
			let bytes_key_size = get_n_bytes(&bytes, 0, key_size);
			let bytes_key_size_plus_one = get_n_bytes(&bytes, key_size, 2*key_size);
			//println!("{:?} <HAM> {:?}", bytes_key_size, bytes_key_size_plus_one);
			let edit_distance = hamming_distance(bytes_key_size, bytes_key_size_plus_one);
			//println!("{}", edit_distance);
			let normalized_distance = (edit_distance as f32) / (key_size as f32);
			/*if normalized_distance < smallest_distance {
				smallest_key = key_size;
				smallest_distance = normalized_distance;
			}*/
			key_distance.push((key_size, normalized_distance));
			//println!("key_size: {}, normalized_distance: {}", key_size, edit_distance);
			//println!("key_size: {}, normalized_distance: {}", key_size, normalized_distance);
		}

		key_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

		//println!("{:?}", key_distance);

		crack_xor_key(&bytes,(key_distance[0].0 as usize));


		//println!("{}",hamming_distance(test1.into_bytes(), test2.into_bytes()));*/

	 	/*let mut f = File::open("to_encrypt.txt").expect("Unable to open file");
	 	let mut contents = String::new();
	 	f.read_to_string(&mut contents).expect("Unable to read");
	 	let mut encrypted_output = String::new();
    let mut curr_key = b'I';
    let answer = hex_string_to_hex(String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));
    let line_bytes = contents.into_bytes();
    let (res, next_key) = repeating_key_xor(line_bytes, curr_key);
    assert_eq!(answer,&*res);
    encrypted_output = hex_vec_to_string(res).to_string();
    println!("{}", encrypted_output);*/
}



