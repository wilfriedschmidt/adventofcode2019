use std::collections::HashMap;
use crate::util::*;

struct PosBearing
{
  x:i32,
  y:i32,
  angle:i32
}

impl Default for PosBearing
{
  fn default() -> PosBearing
  {
    PosBearing
    {
      x:0,
      y:0,
      angle:0
    }
  }
}


struct Pos
{
  x:i32,
  y:i32,
  bearings:Vec<PosBearing>,
}

impl Default for Pos
{
  fn default() -> Pos
  {
    Pos
    {
      x:0,
      y:0,
      bearings:Vec::new()
    }
  }
}

fn getangle(x:i32,y:i32) -> i32
{
  let fx = x as f32;
  let fy = y as f32;
  let angle = (fy.atan2(fx) * 1000.0) as i32;

  return angle;
}

fn recordbearing( bearings:&mut Vec<PosBearing>, positionsbyoffset:&HashMap<i32,bool>, x:i32, y:i32, width:i32, height:i32, sx:i32, sy:i32)
{
  let dx = x + sx;
  let dy = y + sy;

  if dx>=0 && dy>=0 && dx<width && dy<height
  {
    let offset = dy*width+dx;
    if positionsbyoffset.contains_key(&offset) && positionsbyoffset[&offset]
    {
      let angle = getangle(sx,sy);
      let mut found = false;
      for j in 0..bearings.len()
      {
        if bearings[j].angle==angle { found = true; break; }
      }
      if !found 
      {
        let mut posbearing = PosBearing::default();
        posbearing.x = dx;
        posbearing.y = dy;
        posbearing.angle = angle; 
        bearings.push(posbearing); 
      }
    }
  }
}

fn markbearings( bearings:&mut Vec<PosBearing>, positionsbyoffset:&HashMap<i32,bool>, x:i32, y:i32, width:i32, height:i32) -> usize
{
  bearings.clear();

  for r in 1..width+1
  {
    let _radius = r*2+1;
    for sy in -r..r+1
    {
      if sy==-r || sy==r
      {
        for sx in -r..r+1
        {
          recordbearing(bearings, positionsbyoffset, x, y, width, height, sx, sy);
        }
      }
      else
      {
        recordbearing(bearings, positionsbyoffset, x, y, width, height, r, sy);
        recordbearing(bearings, positionsbyoffset, x, y, width, height, -r, sy);
      }
    } 
  }

  return bearings.len()
}

pub fn go(filename:&str) -> (String,String)
{

  // load program
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let chars:Vec<char> = lines[0].chars().collect();
  let width = chars.len() as i32;
  let height = (lines.len()-1) as i32;

  let mut allpositions = Vec::new();
  let mut positionsbyoffset = HashMap::new();
  for i in 0..lines.len()
  {
    let chars:Vec<char> = lines[i].chars().collect();
    for j in 0..chars.len()
    {
      if chars[j]=='#'
      {
        let mut pos = Pos::default();
        pos.x = j as i32;
        pos.y = i as i32;

        positionsbyoffset.insert( pos.y*width+pos.x ,true );
        allpositions.push(pos);
      }
      print!("{}",chars[j]);
    }
    println!();
  }

  let mut largest = 0;
  let mut largestindex = 0;
  for i in 0..allpositions.len()
  {
    let x = allpositions[i].x;
    let y = allpositions[i].y;
    markbearings(&mut allpositions[i].bearings, &positionsbyoffset, x, y, width, height );
  
    if allpositions[i].bearings.len() > largest 
    {
      largestindex = i; 
      largest = allpositions[i].bearings.len(); 
    }
  }

  allpositions[largestindex].bearings.sort_by_key(|k| k.angle);
  let mut starti = 0;
  for i in 0..allpositions[largestindex].bearings.len()
  {
    if allpositions[largestindex].bearings[i].angle==-1570
    {
      starti = i;
    }
  }

  let endindex = 200-(allpositions[largestindex].bearings.len()+1-starti);
  return (largest.to_string(),
          (allpositions[largestindex].bearings[endindex].x*100 + allpositions[largestindex].bearings[endindex].y).to_string());
}
