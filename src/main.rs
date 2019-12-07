mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

    if args[1]=="day5" || args[1]=="all"
    {
      day5::go("./data/day5_input.txt");
    }

    if args[1]=="day6" || args[1]=="all"
    {
      day6::go("./data/day6_input.txt");
    }

    if args[1]=="day7" || args[1]=="all"
    {
      day7::go("./data/day7_input.txt");
    }
  }
}
