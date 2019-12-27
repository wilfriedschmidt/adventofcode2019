use std::collections::HashMap;
use crate::util::*;
use crate::intcode::*;
extern crate num_bigint;
extern crate num_traits;

fn robot(program:&mut Program, inputvalue:i64) -> usize
{
  // initialize inputs
  let mut input = Vec::new();
  input.push(fi64(inputvalue));
  
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
    let offset = y*10000+x;
    if panels.contains_key(&offset)
    {
      input[0] = fi64(panels[&offset]);
    }
    else
    {
      input[0] = fi64(0);
    }

    let mut paint = fi64(0);
    program.step( &mut input, &mut paint);

    if paint==fi64(0)
    {
      // black
      panels.insert(offset,0);
    }
    else if paint==fi64(1)
    {
      panels.insert(offset,1);
    }
    
    let mut rot = fi64(0);
    let retval = program.step( &mut input, &mut rot);

    if rot==fi64(0)
    {
      // turn left
      if dir==0 { dir = 3; }
      else if dir==1 { dir = 0; }
      else if dir==2 { dir = 1; }
      else if dir==3 { dir = 2; }
    }
    else if rot==fi64(1)
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

  for i in miny..maxy+1
  {
    for j in minx..maxx+1
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
  let mut program1 = Program::default();
  program1.load(filename);
  let output1 = robot(&mut program1, 0);

  let mut program2 = Program::default();
  program2.load(filename);
  let output2 = robot(&mut program2, 1);

  return (output1.to_string(),output2.to_string()); 
}
