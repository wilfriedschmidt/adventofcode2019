use crate::util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();

  let chars:Vec<char> = payloadstr.chars().collect();

  let mut image = Vec::new();
  image.resize(25*6,'2');

  let mut lowestzeroes=1000000;
  let mut lowestones=0;
  let mut lowesttwos=0;

  let mut zeroes =0;
  let mut ones = 0;
  let mut twos = 0;

  let mut count=0;
  for i in 0..chars.len()
  {
    if chars[i]=='0' { zeroes+=1; }
    if chars[i]=='1' { ones+=1; }
    if chars[i]=='2' { twos+=1; }

    if image[count]=='2'
    {
      image[count] = chars[i];
    }

    count+=1;
    if count==25*6
    {
      if zeroes < lowestzeroes { lowestzeroes = zeroes; lowestones = ones; lowesttwos = twos; }
      
      zeroes=0;
      ones=0;
      twos=0;
      count=0;
    }
  }

  println!("{} {}", lowestzeroes, lowestones * lowesttwos);

  count=0;
  for y in 0..6
  {
    for x in 0..25
    {
      if image[count]=='1' { print!("{}",image[count]); } else { print!(" "); }
      count+=1;
    }
    println!();
  }
}
