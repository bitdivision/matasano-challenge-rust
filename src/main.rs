extern crate serialize;
use serialize::base64::{ToBase64, MIME};
use serialize::hex::{FromHex, ToHex};
use std::ops::BitXor;

//Tuple struct used for a newtype
struct XorVect(Vec<u8>);


impl BitXor<XorVect, Vec<u8>> for XorVect {
    fn bitxor(&self, rhs: &XorVect) -> Vec<u8> {
        let &XorVect(ref left) = self;
        let &XorVect(ref right) = rhs;

        left.iter().zip(right.iter()).map(|(&first, &second)| first ^ second).collect()
    }
}


pub fn hex_to_b64(hex_string:&str) -> String {
    // MIME is a provided base64 configuration.
    let config = MIME;

    let b64_string = hex_string.from_hex().unwrap().to_base64(config);
    //let b64_string = str::from_utf8(b64_vector.as_slice());
    //let asciiStr = str::from_utf8(byte_vector.as_slice());
    //println!("{}", asciiStr);
    //let b64Str = asciiStr.unwrap().as_bytes().to_base64(config);

    return b64_string;
}

pub fn challenge_1() {

    let hex_input_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let answer = hex_to_b64(hex_input_string);
    println!("Challenge 1: {}", answer);
}

pub fn challenge_2() {
    let input_string = "1c0111001f010100061a024b53535009181c";

    let xor_with = "686974207468652062756c6c277320657965";

    let answer = "746865206b696420646f6e277420706c6179";

    println!("Challenge 2: {}", fixed_xor(input_string,xor_with));
}

pub fn fixed_xor(input1:&str, input2:&str) -> String {
    let config = MIME;
    let input_bytes1 = input1.from_hex().unwrap();
    let input_bytes2 = input2.from_hex().unwrap();

    let xored = XorVect(input_bytes1)^XorVect(input_bytes2);
    xored.to_hex()

   /* 
    * this would output plaintext
    let decoded_string = String::from_utf8(XorVect(input_bytes1) ^ XorVect(input_bytes2)).ok().unwrap();
    decoded_string.as_slice().to_hex().unwrap()*/
}


fn main() {
    challenge_1();
    challenge_2();
}
