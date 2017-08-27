#[macro_use]
extern crate nom;

use std::str;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

//named!(lacefile<&[u8]>, )

// We should use a bytecode map at some point

fn make_varint(num: u32) -> Vec<u8> {
  let mut base_num = num;
  let num_of_bytes = ((32 - base_num.leading_zeros()) / 7);
  let mut bytes : Vec<u8> = Vec::new();
  let mut i = 0;
  while i < (num_of_bytes - 1) {
    bytes.push(((127 & base_num) | 128) as u8);
    base_num = (base_num) >> 7;
    i += 1;
  }
  bytes.push(base_num as u8);
  return bytes;
}

fn convert_utf8_to_starchar(text: &str) -> Vec<u8> {
  panic!("Not implemented.");
}

fn make_quantifier_block(quant: &str, vars: &Vec<&str>) -> Vec<u8> {
  let quant: u8 = match quant {
    "μ" => 1,
    "∀" => 2,
    "∃" => 3,
    "ı" => 4,
    "λ" => 5,
    _   => panic!("Invalid quantifier")
  };
  let mut output: Vec<u8> = vec![quant];
  // We need to add the number of variables
  let len = vars.len() as u32;
  let len = make_varint(len);
  output.extend(len);
  
  // We need to turn len into a varint
  // Convert each variable in turn, then add the varint length plus it
  for var in vars.iter() {
    let starchar = convert_utf8_to_starchar(var);
    let len = starchar.len() as u32;
    let starlength = make_varint(len);
    output.extend(starlength);
    output.extend(starchar);
  }
  return output;
}

named!(quantifier<&str, Vec<u8> >,
       // This gives us back a Vec of variables as utf-8 strings
       do_parse!(
         quant: alt!(tag_s!("μ") | tag_s!("∀") | tag_s!("∃") | tag_s!("ı") | tag_s!("λ")) >>
         vars: separated_nonempty_list_complete!(tag_s!(" "), take_until_either!(" .")) >> 
         tag_s!(".") >>
         (make_quantifier_block(quant, &vars))));


//fn assemble(&str text) -> &[u8] {
//}
