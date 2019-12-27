use crate::util::*;

fn dealintonewdeck(stack:&mut Vec<i32>)
{
  stack.reverse();
}

fn cutdeck(stack:&mut Vec<i32>, n:i32)
{
  let mut newdeck:Vec<i32> = Vec::new();
  newdeck.resize(stack.len(),0);

  let mut offset = n;
  if n < 0 { offset = (stack.len() as i32) + n; }

  for i in 0..stack.len() 
  {
    newdeck[i] = stack[ (i+(offset as usize)) % stack.len() ];
  }

  *stack = newdeck;
}

fn dealwithincrement(stack:&mut Vec<i32>, n:i32)
{
  let mut newdeck:Vec<i32> = Vec::new();
  newdeck.resize(stack.len(),0);

  let mut offset = 0;
  for i in 0..stack.len()
  {
    newdeck[offset] = stack[i];
    offset = (offset+(n as usize))%stack.len();
  }

  *stack = newdeck;
}

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let decklen = 10007;
  let mut deck:Vec<i32> = Vec::new();
  deck.resize(decklen,0);
  for i in 0..decklen { deck[i as usize] = i as i32; }

  for i in 0..lines.len()
  {
    if lines[i].starts_with("deal into new stack")
    {
      dealintonewdeck(&mut deck);
    }
    if lines[i].starts_with("cut")
    {
      let parts:Vec<&str>  = lines[i].split(' ').collect();
      let n = parts[1].parse::<i32>().unwrap();
      
      cutdeck(&mut deck, n);
    }
    if lines[i].starts_with("deal with increment")
    {
      let parts:Vec<&str>  = lines[i].split(' ').collect();
      let n = parts[3].parse::<i32>().unwrap();
  
      dealwithincrement(&mut deck,n);
    }
    
   // println!("{}",lines[i]);    
  }

  let mut output1 = 0;
  for i in 0..deck.len()
  {
    if deck[i] == 2019 { output1 = i; break; }
  }

  return ( output1.to_string(), String::new() );
}
