use crate::util::*;
use crate::grid::*;
use crate::intcode::*;
extern crate num_bigint;
extern crate num_traits;

fn floodfill(program:&mut Program, grid:&mut Grid<char>, depth:i32, dir:i64, x:i32, y:i32, sensorsteps:&mut i32)
{
  let mut tx = x;
  let mut ty = y;
  let mut ndepth = depth;

  let mut found = false;

  // initialize inputs
  let mut input = Vec::new();
  input.push(fi64(dir+1));
  let mut result = fi64(0);
  program.step( &mut input, &mut result);

  let code = ti64(&result);
  if code == 0
  {
    // barrier
    if dir==0 { grid.put(tx,ty-1,'#'); }
    else if dir==1 { grid.put(tx,ty+1,'#'); }
    else if dir==2 { grid.put(tx-1,ty,'#'); }
    else if dir==3 { grid.put(tx+1,ty,'#'); }
  }
  else if code == 1 || code == 2
  {
    // moved
    if dir==0 { ty-=1; }
    else if dir==1 { ty+=1; }
    else if dir==2 { tx-=1; }
    else if dir==3 { tx+=1; }

    if code==1 { grid.put(tx,ty,'.'); }
    else 
    { 
      found = true;
      grid.put(tx,ty,'O');
      *sensorsteps = depth+1;
    }

    ndepth+=1;    
  }

  if !found
  {
    let up = grid.get(tx,ty-1);
    if *up==' ' || *up=='O' 
    { 
      floodfill(&mut program.clone(), grid, ndepth, 0, tx, ty, sensorsteps); 
    }

    let down = grid.get(tx,ty+1);
    if *down==' ' || *down=='O' 
    { 
      floodfill(&mut program.clone(), grid, ndepth, 1, tx, ty, sensorsteps); 
    }

    let left = grid.get(tx-1,ty);
    if *left==' ' || *left=='O'
    { 
      floodfill(&mut program.clone(), grid, ndepth, 2, tx, ty, sensorsteps); 
    }

    let right = grid.get(tx+1,ty);
    if *right==' ' || *right=='O' 
    { 
      floodfill(&mut program.clone(), grid, ndepth, 3, tx, ty, sensorsteps); 
    }
  }
}

pub fn go(filename:&str) -> (String,String)
{
  let mut program1 = Program::default();
  program1.load(filename);
  
  let dim = 64;

  let mut grid:Grid<char> = Grid::default();
  grid.width = dim;
  grid.height = dim;
  grid.data.resize((grid.width*grid.height) as usize,' ');

  let x = dim/2;
  let y = dim/2;

  let mut sensorsteps:i32 = 0;
  floodfill(&mut program1.clone(), &mut grid, 0, 0, x, y, &mut sensorsteps);

  grid.print();

  let mut count=0;
  loop
  {
    let mut put = false;
    for i in 0..dim
    {
      for j in 0..dim
      {
        if *grid.get(j,i)=='O'
        {
          if *grid.get(j+1,i)=='.' { grid.put(j+1,i,'o'); put = true; }
          if *grid.get(j-1,i)=='.' { grid.put(j-1,i,'o'); put = true; }
          if *grid.get(j,i+1)=='.' { grid.put(j,i+1,'o'); put = true; }
          if *grid.get(j,i-1)=='.' { grid.put(j,i-1,'o'); put = true; }
        }
      }
    }

    if put { count+=1; }        
    else { break; }

    for i in 0..dim
    {
      for j in 0..dim
      {
        if *grid.get(j,i)=='o'
        {
          grid.put(j,i,'O');        
        }
      }
    }
 
  }

  grid.print();

  return (sensorsteps.to_string(), count.to_string()); 
}
