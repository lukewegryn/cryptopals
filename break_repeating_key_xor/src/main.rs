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
					    &32 => score += 2320, //SPACE
						&b'E' | &b'e' => score += 1260, //E or e
						&b'T' | &b't' => score += 937, //T or t
						&b'A' | &b'a' => score += 834, //A or a
						&b'O' | &b'o' => score += 770, //O or o
						&b'I' | &b'i' => score += 671, //I or i
						&b'N' | &b'n' => score += 680, //N or n
						&b'S' | &b's' => score += 611, //S or s
						&b'H' | &b'h' => score += 611, //H or h
						&b'R' | &b'r' => score += 568, //R or r
						&b'D' | &b'd' => score += 414, //D or d
						&b'L' | &b'l' => score += 424, //L or l
						&b'U' | &b'u' => score += 285, //U or u
						&b'C' | &b'c' => score += 273, //C or c
						&b'M' | &b'm' => score += 253, //M or m
						&b'F' | &b'f' => score += 203, //F or f
						&b'W' | &b'w' => score += 234, //W or w
						&b'Y' | &b'y' => score += 204, //Y or y
						&b'P' | &b'p' => score += 166, //P or p
						&b'V' | &b'v' => score += 106, //V or v
						&b'B' | &b'b' => score += 154, //B or b
						&b'G' | &b'g' => score += 192, //G or g
						&b'K' | &b'k' => score += 87, //K or k
						&b'J' | &b'j' => score += 23, //J or j
						&b'Q' | &b'q' => score += 9, //Q or q
						&b'X' | &b'x' => score += 20, //X or x
						&b'Z' | &b'z' => score += 6, //Z or z
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

pub fn decrypt(to_decrypt: &Vec<u8>, key: Vec<u8>) -> Vec<u8> {
	let mut curr_index:usize = 0;
	let mut decrypted_result = Vec::new();

	for byte in to_decrypt {
		decrypted_result.push(byte ^ &key[curr_index]);
		curr_index = next_key(curr_index, &key);
	}
	return decrypted_result;
}

pub fn next_key(curr_index: usize, key_vec: &Vec<u8>) -> usize {
	if curr_index >= key_vec.len()-1 {
		return 0;
	} else {
		return curr_index + 1;
	}
}

pub fn hamming_distance(hex_a: Vec<u8>, hex_b: Vec<u8>) -> u32 {
	let xor_res = hex_xor(hex_a, hex_b);
	let mut distance = 0;
	for byte in xor_res {
		distance = distance + byte.count_ones();
	}
	return distance;
}

pub fn get_n_bytes(bytes: &Vec<u8>, lower_n: usize, upper_n: usize) -> Vec<u8> {
	let mut n_bytes = Vec::new();
	for i in lower_n..upper_n {
		n_bytes.push(bytes[i])
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

pub fn crack_xor_key(contents: &Vec<u8>, block_size: usize) -> Vec<u8> {
	let transpose = transpose(&contents, block_size);
	assert_eq!(block_size, transpose.len());
	let mut curr_key = Vec::new();
	for i in 0..transpose.len() {
		//println!("{:?}", transpose[&i]);
		//println!("{:?}",get_decrypt_key(&transpose[&i]));
		curr_key.push(get_decrypt_key(&transpose[&i]))
	}
	return curr_key;
}

pub fn normalized_hamming_distance (bytes: &Vec<u8>, key_size: usize) -> f32{
	let mut ham_sum = 0;
	for i in 0..(bytes.len()/key_size)-1{
		ham_sum += hamming_distance(get_n_bytes(&bytes, (i+0)*key_size, (i+1)*key_size), get_n_bytes(&bytes, (i+1)*key_size, (i+2)*key_size));
		//println!("{}", edit_distance);
	}

	let ham_avg = (ham_sum as f32) / ((bytes.len()/key_size -1) as f32);

	return ham_avg / (key_size as f32);
}

fn main() {
		//let test1 = "this is a test".to_string();
		//let test2 = "wokka wokka!!!".to_string();
		let mut key_distance = Vec::new();
		let mut f = File::open("encrypted.txt").expect("Unable to open file");
	 	let mut encoded_contents = String::new();
	 	f.read_to_string(&mut encoded_contents).expect("Unable to read");
	 	//let decoded_contents = base64::decode(&contents);
		//println!("{}",edit_distance::edit_distance("kitten","sitting"));

		let bytes = base64::decode_config(&encoded_contents, base64::MIME).unwrap();
		//let bytes = contents.into_bytes();
		//println!("{:?}",bytes );

		//let mut smallest_distance: f32 = std::f32::MAX;
		//let mut smallest_key = 2;

		for key_size in 2..40 {
			let normalized_distance = normalized_hamming_distance(&bytes, key_size);
			key_distance.push((key_size, normalized_distance));
		}

		key_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

		//println!("Key Distance: {:?}", key_distance);

		//let mut key_vec = Vec::new();

		/*
		if you want to try the top few key sizes
		for i in 0..5 {
			println!("KEYSIZE: {}", key_distance[i].0);
			//println!("----------------");
			println!("{:?}", String::from_utf8(decrypt(&bytes,crack_xor_key(&bytes,(key_distance[i].0 as usize)))).unwrap());
		}*/
		println!("Key Size: {:?}", key_distance[0].0);
		println!("{:?}", String::from_utf8(decrypt(&bytes,crack_xor_key(&bytes,(key_distance[0].0 as usize)))).unwrap());
}



