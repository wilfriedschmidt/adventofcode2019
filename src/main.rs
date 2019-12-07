mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

extern crate time;

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
      let start = time::precise_time_ns();
      day1::go("./data/day1_input.txt");
      let end = time::precise_time_ns();
      println!("day1 duration: {}ms", (end-start)/1000000);
    }

    if args[1]=="day2" || args[1]=="all"
    {
      let start = time::precise_time_ns();
      day2::go("./data/day2_input.txt");
      let end = time::precise_time_ns();
      println!("day2 duration: {}ms", (end-start)/1000000);
    }

    if args[1]=="day3" || args[1]=="all"
    {
      let start = time::precise_time_ns();
      day3::go("./data/day3_input.txt");
      let end = time::precise_time_ns();
      println!("day3 duration: {}ms", (end-start)/1000000);
    }

    if args[1]=="day4" || args[1]=="all"
    {
      let start = time::precise_time_ns();
      day4::go();
      let end = time::precise_time_ns();
      println!("day4 duration: {}ms", (end-start)/1000000);
    }

    if args[1]=="day5" || args[1]=="all"
    {
      let start = time::precise_time_ns();
      day5::go("./data/day5_input.txt");
      let end = time::precise_time_ns();
      println!("day5 duration: {}ms", (end-start)/1000000); 
    }

    if args[1]=="day6" || args[1]=="all"
    {
      let start = time::precise_time_ns();
      day6::go("./data/day6_input.txt");
      let end = time::precise_time_ns();
      println!("day6 duration: {}ms", (end-start)/1000000);
    }

    if args[1]=="day7" || args[1]=="all"
    {
      let start = time::precise_time_ns(); 
      day7::go("./data/day7_input.txt");
      let end = time::precise_time_ns();
      println!("day7 duration: {}ms", (end-start)/1000000);
    }
  }
}
