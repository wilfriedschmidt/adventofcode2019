use std::collections::HashMap;
use crate::util::*;

fn writegrid(x:i64, y:i64, k:usize, steps:&mut i32, grid:&mut HashMap<i64,i32>, mindist:&mut i64, minsteps:&mut i32)
{
  let index = y*500000+x;
  if grid.contains_key(&index) && k==1
  {
    let dist = x.abs() + y.abs();
    if dist!=0
    {
      let totalsteps = *steps + *grid.get_mut(&index).unwrap();

      if dist < *mindist
      {
        *mindist = dist;
      }

      if totalsteps < *minsteps
      {
        *minsteps = totalsteps
      }
    }
  }
  else if k==0
  {
    grid.insert(index,*steps);
  }

  *steps+=1;
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut grid = HashMap::new();
  
  let mut mindist:i64 = 1000000;
  let mut minsteps:i32 = 1000000;

  for k in 0..2
  {
    let mut x:i64=0;
    let mut y:i64=0;
    let mut steps = 0;
    
    let lineparts:Vec<&str> = lines[k].split(',').collect();
    for i in 0..lineparts.len()
    {
      let dir = &lineparts[i][0..1];
      let length = lineparts[i][1..].parse::<i64>().unwrap();

      if dir=="U"
      {
        for _j in 0..length
        {
          writegrid(x, y, k, &mut steps, &mut grid, &mut mindist, &mut minsteps);
          y-=1;
        }
      }
      else if dir=="D"
      {
        for _j in 0..length
        {
          writegrid(x, y, k, &mut steps, &mut grid, &mut mindist, &mut minsteps);
          y+=1;
        }
      }
      else if dir=="R"
      {
        for _j in 0..length
        {
          writegrid(x, y, k, &mut steps, &mut grid, &mut mindist, &mut minsteps);
          x+=1;
        }
      }
      else if dir=="L"
      {
        for _j in 0..length
        {
          writegrid(x, y, k, &mut steps, &mut grid, &mut mindist, &mut minsteps);
          x-=1;
        }
      }
    }
  }

  println!("mindist: {}, minsteps: {}", mindist, minsteps); 
}
