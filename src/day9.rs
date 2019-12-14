use std::collections::HashMap;
use crate::util::*;
extern crate num_bigint;
extern crate num_traits;

pub fn go(filename:&str) -> (String,String)
{
  // load program
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let parts:Vec<&str> = payloadstr.split(',').collect();

  let mut ints:Vec<i64> = Vec::new();
  ints.resize(parts.len()*10, 0);

  for i in 0..parts.len()-1
  {
    if parts[i].len()>0
    {
      let opcode = &parts[i].parse::<i64>().unwrap();
      ints[i] = *opcode;
    }
  }

  // initialize inputs
  let mut input = Vec::new();
  let mut inputindex=0;
  input.push(fi64(1));

  // initialize programs
  let mut program = HashMap::new();
  let mut instpointer = fi64(0);

  for j in 0..ints.len()
  {
    let address:num_bigint::BigInt = fi64(j as i64);
    let value:num_bigint::BigInt = fi64(ints[j]);
    program.insert(address,value);
  }
  
  let mut output1 = fi64(0);
  loop
  {
    let retval = runprogram_bigint(&mut program, &mut instpointer, &input, &mut inputindex, &mut output1);
    if retval==1 { break; }
  }

  inputindex=0;
  input[0] = fi64(2);

  // initialize programs
  program = HashMap::new();
  instpointer = fi64(0);

  for j in 0..ints.len()
  {
    let address:num_bigint::BigInt = fi64(j as i64);
    let value:num_bigint::BigInt = fi64(ints[j]);
    program.insert(address,value);
  }
  
  let mut output2 = fi64(0);
  loop
  {
    let retval = runprogram_bigint(&mut program, &mut instpointer, &input, &mut inputindex, &mut output2);
    if retval==1 { break; }
  }

  return (output1.to_str_radix(10),output2.to_str_radix(10)); 
}
