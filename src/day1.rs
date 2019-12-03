use crate::util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut sumformass = 0;
  let mut fuelforeverything = 0;

  let mut i=0;
  loop
  {
    if lines[i].len()>1
    {
      let mass = &lines[i].parse::<i32>().unwrap();
      let fuelformass = mass/3 - 2;

      // compute fuel requirements for just the mass
      sumformass+=fuelformass;

      // do the rest including the fuel
      let mut fuelformassandfuel = *mass;
      loop
      {
        fuelformassandfuel = fuelformassandfuel/3 - 2;
        if fuelformassandfuel>0
        {
          fuelforeverything+=fuelformassandfuel;
        }
        else
        {
          break;
        }
      } 
    }
    i+=1;

    if i>=lines.len()
    {
      break;
    }
  }

  println!("sum for mass: {} fuel for everything: {}", sumformass, fuelforeverything);
  if (sumformass==3404722) && (fuelforeverything==5104215)
  {
    println!("PASSED");
  }
  else
  {
    println!("FAILED");
  }
}
