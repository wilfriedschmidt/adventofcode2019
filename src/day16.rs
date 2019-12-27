use crate::util::*;

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let chars:Vec<char> = payloadstr.chars().collect();

  let datalen = chars.len()-1;
  for i in 0..datalen
  {
    print!("{}",chars[i]);
  }
  println!();  

  let pattern:[i64;4] = [0,1,0,-1];

  let mut input:Vec<i64> = Vec::new();
  input.resize(datalen,0);

  for i in 0..datalen
  {
    input[i] = chars[i].to_digit(10).unwrap() as i64;
  }

  let multiplier = 1;

  let mut buffers:[Vec<i64>;2] = [Vec::new(), Vec::new()];
  let mut bufferindex = 0;
  buffers[0].resize(input.len()*multiplier,0);
  buffers[1].resize(input.len()*multiplier,0);

  for i in 0..input.len()
  {
    for j in 0..multiplier
    {
      buffers[0][j*input.len()+i] = input[i];
    }
  }

  for _p in 0..100
  {
    for i in 0..buffers[bufferindex].len()
    {
      let mut sum = 0;
      for j in i..buffers[bufferindex].len()
      {
        let patternindex = (j+1) / (i+1);
        sum += buffers[bufferindex][j] * pattern[ patternindex%4 ];
      }
  
      buffers[1-bufferindex][i] = sum.abs() % 10;
    }
  
 /*   for i in 0..buffers[bufferindex].len()
    {
      print!("{}",buffers[bufferindex][i]);
    }
    println!();  
*/
    bufferindex = 1-bufferindex;
  }

  let mut output1 = String::new();
  for i in 0..8
  {
    output1 += &buffers[bufferindex][i].to_string();
  }

  return ( output1, String::new() );
}
