mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod coord;

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
    type Gofn = fn(&str) -> (String,String);

    let mut functions = Vec::<Gofn>::new();
    functions.push(day1::go);  
    functions.push(day2::go);  
    functions.push(day3::go);  
    functions.push(day4::go);  
    functions.push(day5::go);  
    functions.push(day6::go);  
    functions.push(day7::go);  
    functions.push(day8::go);
    functions.push(day9::go);
    functions.push(day10::go);
    functions.push(day11::go);
    functions.push(day12::go);  

    for i in 0..functions.len()
    {
      if args[1]=="all" || args[1].parse::<usize>().unwrap()==i+1
      {
        let start = time::precise_time_ns();
        let pair = functions[i](&format!("./data/day{}_input.txt",i+1));
        let end = time::precise_time_ns();

        println!("{} {} {}ms", pair.0, pair.1, (end-start)/1000000);
      }
    }
  }
}
