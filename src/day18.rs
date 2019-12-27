use crate::util::*;
use crate::grid::*;

fn isopen(val:char) -> bool
{
  if val=='.' { return true; }
  else { return false; }
}

fn iskey(val:char) -> bool
{
  if val != '#' && val.is_lowercase() { return true; }
  else { return false; }
}

fn floodfill(grid:&mut Grid<char>) -> (i32,char)
{
  let mut key = ' ';
  let mut steps = 0;
  loop
  {
    // mark
    let mut putval = false;
    for i in 0..grid.height
    {
      for j in 0..grid.width
      {
        if *grid.get(j,i)=='@'
        {
          let up = *grid.get(j,i-1);
          let down = *grid.get(j,i+1);
          let left = *grid.get(j-1,i);
          let right = *grid.get(j+1,i);

          if isopen(up) { grid.put(j,i-1,'*'); putval = true; }
          else if iskey(up) { key = up; break; }

          if isopen(down) { grid.put(j,i+1,'*'); putval = true; }
          else if iskey(down) { key = down; break; }

          if isopen(left) { grid.put(j-1,i,'*'); putval = true; }
          else if iskey(left) { key = left; break; }

          if isopen(right) { grid.put(j+1,i,'*'); putval = true; }
          else if iskey(right) { key = right; break; }        

         
        }

      }

      if key != ' ' { break; }
    }

    if !putval { break; }

    steps+=1;
    if key != ' ' { break; }
    
    // sweep
    for i in 0..grid.height
    {
      for j in 0..grid.width
      {
        if *grid.get(j,i)=='*' { grid.put(j,i,'@'); }
      }
    }

    //grid.print();
  }

  return (steps,key);
}

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let dim = 32;
  let mut grid:Grid<char> = Grid::default();
  grid.width = dim;
  grid.height = dim;
  grid.data.resize((dim*dim) as usize,' ');

  for i in 0..lines.len()
  {
    let chars:Vec<char> = lines[i].chars().collect();
    for j in 0..chars.len()
    {
      grid.put(j as i32,i as i32, chars[j]);
    }    
  }

  grid.print();

  let mut totalsteps = 0;
  loop
  {
    let pair = floodfill(&mut grid.clone());
    totalsteps+=pair.0;
    let key:char = pair.1;
    if key != ' '
    {
  
      print!("{} ",key);

      for i in 0..grid.height
      {
        for j in 0..grid.width
        {
          if *grid.get(j,i)=='@'
          {
            grid.put(j,i,'.');
          }
        }
      }

      let chars:Vec<char> = key.to_uppercase().to_string().chars().collect();
      let door = chars[0];
      
      for i in 0..grid.height
      {
        for j in 0..grid.width
        {
          if *grid.get(j,i)==door
          {
            grid.put(j,i,'.');
          }
          if *grid.get(j,i)==key
          {
            grid.put(j,i,'@');
          }
        }
      }     
    }
    else { break; }

    //grid.print();
  }

  print!("total steps {}", totalsteps);

  return ( String::new(), String::new() );
}
