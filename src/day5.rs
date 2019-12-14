use crate::util::*;

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let parts:Vec<&str> = payloadstr.split(',').collect();

  let mut ints:Vec<i64> = Vec::new();
  ints.resize(parts.len()-1, 0);

  for i in 0..parts.len()-1
  {
    if parts[i].len()>0
    {
      let opcode = &parts[i].parse::<i64>().unwrap();
      ints[i] = *opcode;
    }
  }

  let mut program:Vec<i64> = Vec::new();
  program.resize(ints.len(), 0);

  // part 1
  for i in 0..ints.len()
  {
    program[i]=ints[i];
  }

  let mut instpointer = 0;
  let mut inputs = Vec::new();
  inputs.resize(1,1);

  let mut inputindex = 0;
  let mut output1 = 0;

  runprogram(&mut program, &mut instpointer, &inputs, &mut inputindex, &mut output1); 

  // part 2
  for i in 0..ints.len()
  {
    program[i]=ints[i];
  }

  instpointer = 0;
  inputs[0] = 5;
  inputindex = 0;
  let mut output2 = 0;
  runprogram(&mut program, &mut instpointer, &inputs, &mut inputindex, &mut output2); 

  return (output1.to_string(),output2.to_string());
}
