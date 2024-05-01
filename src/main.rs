use faster_hex::{hex_encode, hex_decode, Error};

fn main() {
    let bytes = [0xDE, 0xAD, 0xBE, 0xEF];

    // Encode bytes to hexadecimal string
    let mut encoded = [0u8; 8]; // Allocate enough space to store the encoded result
    let encoded_str = match hex_encode(&bytes, &mut encoded) {
        Ok(encoded_str) => encoded_str,
        Err(err) => {
            eprintln!("Encoding failed: {}", err);
            return;
        }
    };
    println!("Encoded: {}", encoded_str);

    // Decode hexadecimal string to bytes
    let mut decoded = [0u8; 4]; // Allocate enough space to store the decoded result
    match hex_decode(encoded_str.as_bytes(), &mut decoded) {
        Ok(_) => println!("Decoded: {:?}", decoded),
        Err(err) => eprintln!("Decoding failed: {}", err),
    }
}
