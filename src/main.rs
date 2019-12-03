mod util;
mod day1;
mod day2;

use std::env;

fn main()
{
  let args:Vec<String> = env::args().collect();

  if args.len()<=1
  {
    println!("enter day...");
  } 
  else
  { 
    if args[1]=="day1" || args[1]=="all"
    {
      day1::go("./data/day1_input.txt");
    }

    if args[1]=="day2" || args[1]=="all"
    {
      day2::go("./data/day2_input.txt");
    }
  }
}
