use std::collections::HashMap;

use crate::util::*;

fn count(orbit:&str, orbits:&Vec<&str>, parents:&HashMap<&str,&str>, path:&mut Vec<String>) -> i32
{
  let mut sum:i32 = 0;
  if parents.contains_key(orbit)
  { 
    path.push(parents[orbit].to_string());
    sum+=count(parents[orbit],&orbits, &parents, path);
  }

  return sum+1;
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let parts:Vec<&str> = payloadstr.split('\n').collect();

  let mut ints:Vec<i64> = Vec::new();
  ints.resize(parts.len()-1, 0);

  let mut orbits = Vec::new();
  let mut parents = HashMap::new();

  for i in 0..parts.len()-1
  {
    if parts[i].len()>0
    {
      let subparts:Vec<&str> = parts[i].split(")").collect();
      if subparts.len()>0
      {     
        orbits.push(subparts[1]);
        parents.insert(subparts[1],subparts[0]);
      }    
    }
  }

  let mut allpaths = HashMap::<&str,Vec<String>>::new();

  let mut total:i32 = 0;
  for i in 0..orbits.len()
  {
    let mut path = Vec::new();
    total+=count(orbits[i], &orbits, &parents, &mut path);
    allpaths.insert(orbits[i],path);
  }

  let sanorbits:&Vec::<String> = &allpaths["SAN"];
  let youorbits:&Vec::<String> = &allpaths["YOU"];

  let mut found = false;
  for i in 0..sanorbits.len()
  {
    for j in 0..youorbits.len()
    {
      if sanorbits[i]==youorbits[j]
      {
        println!("{} {}", sanorbits[i], i+j);
        found = true;
        break;
      }
    }
  
    if found { break; }
  }

  println!("{}", total - orbits.len() as i32);
}
