use crate::util::*;
use crate::grid::*;
use std::collections::HashMap;

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();


  let mut grids:[Grid<char>;2] = [Grid::default(), Grid::default()];

  let dim = 7;
  for i in 0..2
  {
    grids[i].width = dim;
    grids[i].height = dim;
    grids[i].data.resize((dim*dim) as usize,'.');
  }

  let mut gridindex = 0;

  for i in 0..lines.len()
  {
    for j in 0..lines[i].len()
    {
      let chars:Vec<char> = lines[i].chars().collect();
      grids[0].put((j+1) as i32,(i+1) as i32,chars[j]);
      grids[1].put((j+1) as i32,(i+1) as i32,chars[j]);
    }
  }

  grids[0].print();
  println!();

  let mut ratings = HashMap::new();

  let mut output1 = 0;
  for _i in 0..100
  {
    for y in 1..dim-1
    {
      for x in 1..dim-1
      {
        let mut count=0;
        if *grids[gridindex].get(x-1,y)=='#' { count+=1; }
        if *grids[gridindex].get(x+1,y)=='#' { count+=1; }
        if *grids[gridindex].get(x,y-1)=='#' { count+=1; }
        if *grids[gridindex].get(x,y+1)=='#' { count+=1; }

        let current = *grids[gridindex].get(x,y);
        if current=='#' && count != 1 { grids[1-gridindex].put(x,y,'.'); }
        else if current=='.' && (count==1 || count==2) { grids[1-gridindex].put(x,y,'#'); }
      }
    }

    let mut rating = 0;
    let base:i32 = 2;
    let mut power = 0;
    for y in 1..dim-1
    {
      for x in 1..dim-1
      {
        let val = *grids[1-gridindex].get(x,y);
        grids[gridindex].put(x,y,val);
    
        if val=='#' { rating+=base.pow(power); }
        power+=1;
      }
    }

    if ratings.contains_key(&rating)
    {
      output1 = rating;
      break;
    }
    else { ratings.insert(rating,true); }


    gridindex = 1 - gridindex;
  }

  return ( output1.to_string(), String::new() );
}
