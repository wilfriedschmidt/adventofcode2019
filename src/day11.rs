use std::collections::HashMap;
use crate::util::*;
extern crate num_bigint;
extern crate num_traits;

fn robot(ints:&Vec<i64>, inputvalue:i64) -> usize
{
  // initialize inputs
  let mut input = Vec::new();
  let mut inputindex=0;
  input.push(fi64(inputvalue));

  // initialize programs
  let mut program = HashMap::new();
  let mut instpointer = fi64(0);

  for j in 0..ints.len()
  {
    let address:num_bigint::BigInt = fi64(j as i64);
    let value:num_bigint::BigInt = fi64(ints[j]);
    program.insert(address,value);
  }
  
  let mut x=0;
  let mut y=0;
  let mut panels = HashMap::new();
  panels.insert(0,inputvalue);

  let mut dir = 0;

  let mut minx = 10000;
  let mut miny = 10000;
  let mut maxx = -10000;
  let mut maxy = -10000;

  loop
  {
    inputindex=0;

    let offset = y*10000+x;
    if panels.contains_key(&offset)
    {
      input[inputindex] = fi64(panels[&offset]);
    }
    else
    {
      input[inputindex] = fi64(0);
    }

    let mut paint = fi64(0);    
    runprogram_bigint(&mut program, &mut instpointer, &input, &mut inputindex, &mut paint);

    if paint==fi64(0)
    {
      // black
      panels.insert(offset,0);
    }
    else
    {
      panels.insert(offset,1);
    }
    
    let mut rot = fi64(0);
    let retval = runprogram_bigint(&mut program, &mut instpointer, &input, &mut inputindex, &mut rot);

    if rot==fi64(0)
    {
      // turn left
      if dir==0 { dir = 3; }
      else if dir==1 { dir = 0; }
      else if dir==2 { dir = 1; }
      else if dir==3 { dir = 2; }
    }
    else
    {
      if dir==0 { dir = 1; }
      else if dir==1 { dir = 2; }
      else if dir==2 { dir = 3; }
      else if dir==3 { dir = 0; }
    }

    if dir==0 { y-=1; }
    else if dir==1 { x+=1; }
    else if dir==2 { y+=1; }
    else if dir==3 { x-=1; }

    if x < minx { minx = x; }
    if x > maxx { maxx = x; }
    if y < miny { miny = y; }
    if y > maxy { maxy = y; }

    if retval==1 { break; }
  }

  for i in miny..maxy
  {
    for j in minx..maxx
    {
      let offset = i*10000+j;
      if panels.contains_key(&offset)
      {
        if panels[&offset]==1
        {
          print!("#");
        }
        else
        {
          print!(" ");
        }
      }
      else
      {
        print!(" ");
      }     
    }
    println!();
  }

  return panels.len();
}

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

  let output1 = robot(&ints, 0);
  let output2 = robot(&ints, 1);

  return (output1.to_string(),output2.to_string()); 
}
