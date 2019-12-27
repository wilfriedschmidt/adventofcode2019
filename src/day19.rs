use crate::util::*;
use crate::grid::*;
use crate::intcode::*;
extern crate num_bigint;
extern crate num_traits;

fn mapbeam(grid:&mut Grid<char>, dim:i32, xoffset:i32, yoffset:i32, program:&mut Program) -> i32
{
  let mut count=0;
  let mut result = fi64(0);

  for y in 0..dim
  {
    for x in 0..dim
    {
      let mut program = program.clone();

     // initialize inputs
      let mut input = Vec::new();
      input.push(fi64((y+yoffset) as i64));
      input.push(fi64((x+xoffset) as i64));

      loop
      {
        let retval = program.step( &mut input, &mut result);
        if retval==1 { break; }
      }

      if result==fi64(1) 
      {
        grid.put(x as i32,y as i32,'#'); 
        count+=1; 
      }
    }
  }

  return count;
}

pub fn go(filename:&str) -> (String,String)
{
  let mut oprogram = Program::default();
  oprogram.load(filename);

  let dim = 200;
  let mut grid = Grid::default();
  grid.width = dim;
  grid.height = dim;
  grid.data.resize((dim*dim) as usize, ' ');

  let output1 = mapbeam(&mut grid, 50, 0, 0, &mut oprogram); 

  let xoffset = 700;
  let yoffset = 900;

  for i in 0..dim*dim { grid.data[i as usize] = ' '; };

  mapbeam(&mut grid, dim, xoffset, yoffset, &mut oprogram);
/*
  let mut count=0;
  for i in 0..dim
  {
    for j in 0..dim
    {
      let mut program = oprogram.clone();

     // initialize inputs
      let mut input = Vec::new();
      input.push(fi64((i+yoffset) as i64));
      input.push(fi64((j+xoffset) as i64));

      loop
      {
        let retval = program.step( &mut input, &mut result);
        if retval==1 { break; }
      }

      if result==fi64(1) 
      {
        grid.put(j as i32,i as i32,'#'); 
        count+=1; 
      }
    }
  }*/

  let testdim = 100;
  let mut horizontal = true;
  let mut vertical = true;
  let mut output2 = 0;
  for i in 0..dim - testdim
  {
    for j in 0..dim - testdim
    {
      horizontal = true;
      vertical = true;
      for k in 0..testdim
      {
        if *grid.get(j+k,i) != '#' { horizontal = false; break; }
      }

      for k in 0..testdim
      {
        if *grid.get(j,i+k) != '#' { vertical = false; break; }
      }

      if horizontal && vertical 
      { 
        grid.put(j,i,'*'); 
        output2 = (j+xoffset)*10000+(i+yoffset);
        break; 
      }
    }

    if horizontal && vertical { break; } 
  }

  grid.print();

  return (output1.to_string(), output2.to_string());
}
