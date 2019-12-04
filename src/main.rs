mod util;
mod day1;
mod day2;
mod day3;
mod day4;

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

    if args[1]=="day3" || args[1]=="all"
    {
      day3::go("./data/day3_input.txt");
    }

    if args[1]=="day4" || args[1]=="all"
    {
      day4::go();
    }
  }
}
