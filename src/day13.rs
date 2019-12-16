use std::collections::HashMap;
use crate::util::*;
use crate::intcode::*;
extern crate num_bigint;
extern crate num_traits;

fn drawgrid(grid:&HashMap<i64,i64>, minx:i64, maxx:i64, miny:i64, maxy:i64)
{
  for i in miny..maxy+1
  {
    for j in minx..maxx+1
    {
      let offset = i*10000+j;
      if grid.contains_key(&offset)
      {
        if grid[&offset]==1 { print!("|"); }
        else if grid[&offset]==2 { print!("#"); }
        else if grid[&offset]==3 { print!("-"); }
        else if grid[&offset]==4 { print!("O"); }
        else { print!(" "); }      
      }
      else
      {
        print!(" ");
      }     
    }
    println!();
  }
}

fn game(program:&mut Program) -> (usize,i64)
{
  // initialize inputs
  let mut input = Vec::new();
  input.push(fi64(0));

  let mut minx = 10000;
  let mut miny = 10000;
  let mut maxx = -10000;
  let mut maxy = -10000;

  let mut grid = HashMap::new();
  let mut count = 0;

  let mut paddlex = 0;

  loop
  {
    let mut bigx = fi64(0);
    program.step( &input, &mut bigx);
    let x = ti64(&bigx);

    let mut bigy = fi64(0);
    program.step( &input, &mut bigy);
    let y = ti64(&bigy);

    let mut tile = fi64(0);
    let retval = program.step( &input, &mut tile);

    let offset = y*10000+x;
    grid.insert(offset,ti64(&tile));

    if x < minx { minx = x; }
    if x > maxx { maxx = x; }
    if y < miny { miny = y; }
    if y > maxy { maxy = y; }

    if retval==1 { break; }

    if tile == fi64(2)
    {
      count+=1;
    }      

    if tile == fi64(3)
    {
      paddlex = x;
    }

    if tile == fi64(4)
    {
      if x < paddlex { input[0] = fi64(-1); }
      else if x > paddlex { input[0] = fi64(1); }
      else { input[0] = fi64(0); }
    }
  }

  let mut score = 0;
  if grid.contains_key(&-1) { score = grid[&-1]; }

  return ( count, score );
}

pub fn go(filename:&str) -> (String,String)
{
  let mut program1 = Program::default();
  program1.load(filename);
  let pair1 = game(&mut program1);

  let mut program2 = Program::default();
  program2.load("./data/day13_input2.txt");
  let pair2 = game(&mut program2);

  return (pair1.0.to_string(),pair2.1.to_string()); 
}
