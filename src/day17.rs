use std::collections::HashMap;
use crate::util::*;
use crate::grid::*;
use crate::intcode::*;
extern crate num_bigint;
extern crate num_traits;

pub fn go(filename:&str) -> (String,String)
{
  let mut program = Program::default();
  program.load(filename);

  let dim = 64;
  let mut grid:Grid<char> = Grid::default();
  grid.width = dim;
  grid.height = dim;
  grid.data.resize((grid.width*grid.height) as usize,' ');
  
  // initialize inputs
  let mut input = Vec::new();
  input.push(fi64(0));

  let mut result = fi64(0);
  let mut x=0;
  let mut y=0;

  let mut rx = 0;
  let mut ry = 0;

  loop
  {
    let retval = program.step( &mut input, &mut result);

    let ch = std::char::from_u32(ti64(&result) as u32).unwrap();
    if ch=='\n' || ch=='\r'
    {
      x=0;
      y+=1;
    }
    else
    {
      if ch=='^'
      {
        rx = x;
        ry = y;
      }

      grid.put(x,y,ch);
      x+=1;
    }

    if retval==1 { break; }
  }

  let mut sum = 0;
  for y in 1..dim-1
  {
    for x in 1..dim-1
    {
      if *grid.get(x,y)=='#' && 
         *grid.get(x-1,y)=='#' &&
         *grid.get(x+1,y)=='#' &&
         *grid.get(x,y-1)=='#' &&
         *grid.get(x,y+1)=='#'
      {
        grid.put(x,y,'O');
        sum += x*y;
      }
    }
  }

  let mut count = 0;
  let mut dir = 0;
  let mut lastdir = 0;

  let mut commands = Vec::new();

  loop
  {
    let mut gooddir = false;
    for _i in 0..4
    {
      let mut nx = rx;
      let mut ny = ry;

      if dir==0 { ny-=1; }
      else if dir==1 { nx+=1; }
      else if dir==2 { ny+=1; }
      else if dir==3 { nx-=1; }

      if *grid.get(nx,ny)=='#' || *grid.get(nx,ny)=='O'
      {
        let deltadir = dir - lastdir;
        if dir != lastdir
        {
          commands.push(count.to_string());
          if deltadir==1 || deltadir==-3 
          { 
            commands.push("R".to_string()); 
          }
          else if deltadir==-1 || deltadir==3 
          { 
            commands.push("L".to_string());
          }
          count = 0; 
        }

        // store direction changes
        count+=1;
        rx = nx;
        ry = ny;
        if *grid.get(nx,ny)!='O' { grid.put(rx,ry,'^'); }
        gooddir = true;

        lastdir = dir;
  
        break;
      }
      else
      {
        dir = (dir + 1)%4;
      }
    }
  
    if !gooddir { break; }
  }

  grid.print();

  commands.remove(0);
  commands.remove(commands.len()-1);

  let allcommands = commands.clone();

  /*
  allcommands.push("L".to_string());
  allcommands.push("L".to_string());

  for i in 0..commands.len()
  {
    let command = &commands[commands.len() - 1 - i];
    if command=="L" { allcommands.push("R".to_string()); }
    else if command=="R" { allcommands.push("L".to_string()); }
    else { allcommands.push(command.clone()); }
  }*/
  
  let mut overlap = Vec::new();
  overlap.resize(allcommands.len(),false);

  let mut substitutions = Vec::new();

  let mut patterns = HashMap::new();

  let startlen = 8;

  for i in 0..startlen
  {
    let testlen = startlen - i;
    println!("testing {}",testlen);

    if testlen>2
    {
      for j in 0..allcommands.len() - testlen
      {
        println!("testing:");
        let mut foundoverlap = false;
        for k in 0..testlen
        {
          print!("{},",allcommands[j+k]);
          if overlap[j+k] { foundoverlap = true; }
        }
        println!("{}", foundoverlap);

        if !foundoverlap
        {

          for l in j+1..allcommands.len() - testlen
          {

            foundoverlap = false;
            for k in 0..testlen
            {
              if overlap[l+k] { foundoverlap = true; }
            }
            //println!("{}", foundoverlap);

            if !foundoverlap
            {

              let mut matched = true;
              for k in 0..testlen
              {
                if (allcommands[j+k] != allcommands[l+k]) || overlap[j+k] || overlap[l+k] 
                { 
                  matched = false; 
                  break; 
                }
              }

              if matched 
              { 
                for k in 0..testlen
                {
                  overlap[j+k] = true;
                  overlap[l+k] = true;
                }

                for k in 0..allcommands.len()
                {
                  print!("{},",overlap[k]);
                }
                println!("{} {}",j,l);

                let mut matchstr = String::new();
                let mut sub = Vec::new();
                for k in 0..testlen
                {
                  matchstr+=&format!("{},",allcommands[j+k]);
                  sub.push(allcommands[j+k].clone());
                  print!("{},",allcommands[j+k]);
                }
                println!();
                
                if patterns.contains_key(&matchstr) { *patterns.get_mut(&matchstr).unwrap()+=1; }
                else 
                {
                  substitutions.push(sub); 
                  patterns.insert(matchstr,1); 
                }
              }
            }
          }
        }
      }
    }
  }

  let labels:[char;26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z' ];
  let mut labelindex = 0;

  let mut temp = Vec::new();

  let mut i = 0;
  loop  
  {
    if !overlap[i]
    {
      temp.push(allcommands[i].clone());
      i+=1;
    }
    else
    {
      if temp.len()>0
      {
        print!("{}: ",labels[labelindex]);
        labelindex+=1;
        for k in 0..temp.len()
        {
          print!("{},", temp[k]);
        }
        println!();
      }

      // emit temp
      temp.clear();

      // find the substitution and emit it
      
      let mut submatch = true;
      for sub in 0..substitutions.len()
      {
        submatch = true;
        for k in 0..substitutions[sub].len()
        {
          if i+k>=allcommands.len() || allcommands[i+k]!=substitutions[sub][k] { submatch = false; }
        }

        if submatch
        {
          print!("{}: ",labels[14+sub]);
        
          for k in 0..substitutions[sub].len()
          {
            print!("{},", substitutions[sub][k]);
          }
          println!();
        
          i+=substitutions[sub].len();
          break;
        }
      }

      if !submatch { i+=1; }
    }

    if i>=allcommands.len() { break; }
  }

  for sub in &substitutions
  {
    for k in 0..sub.len()
    {
      print!("{},", sub[k]);
    }
    println!();
  }

  for i in 0..allcommands.len()
  {
    print!("{},", allcommands[i]);
  }
  println!();
  
  for i in 0..overlap.len()
  {
    print!("{},", overlap[i]);
  }

  return (sum.to_string(), String::new()); 
}
