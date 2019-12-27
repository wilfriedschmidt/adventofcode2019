use crate::util::*;
use crate::intcode::*;
extern crate num_bigint;
extern crate num_traits;

pub fn go(filename:&str) -> (String,String)
{
  let mut program1 = Program::default();
  program1.load(filename);

  // initialize inputs
  let mut input = Vec::new();
  input.push(fi64(1));
  
  let mut output1 = fi64(0);
  loop
  {
    let retval = program1.step( &mut input, &mut output1);
    if retval==1 { break; }
  }

  let mut program2 = Program::default();
  program2.load(filename);

  input[0] = fi64(2);

  let mut output2 = fi64(0);
  loop
  {
    let retval = program2.step( &mut input, &mut output2);
    if retval==1 { break; }
  }
  
  return (output1.to_str_radix(10),output2.to_str_radix(10));
}
