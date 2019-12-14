use crate::util::*;
use crate::coord::*;

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut omoons:Vec<Coord> = Vec::new();
  
  for i in 0..lines.len()
  {
    if lines[i].len()>0
    {
      let line = &lines[i][1..lines[i].len()-1];
      let parts:Vec<&str> = line.split(' ').collect();

      let x:Vec<&str> = parts[0].split('=').collect();
      let y:Vec<&str> = parts[1].split('=').collect();
      let z:Vec<&str> = parts[2].split('=').collect();      

      let mut moon = Coord::default();
      moon.x = x[1][..x[1].len()-1].parse::<i64>().unwrap();
      moon.y = y[1][..y[1].len()-1].parse::<i64>().unwrap();
      moon.z = z[1].parse::<i64>().unwrap();
      omoons.push(moon);
    }
  }

  let mut moons:Vec<Coord> = Vec::new();
  let mut velocities:Vec<Coord> = Vec::new();

  for i in 0..omoons.len()
  {
      let mut moon = Coord::default();
      moon.x = omoons[i].x;
      moon.y = omoons[i].y;
      moon.z = omoons[i].z;
      moons.push(moon);
      
      let vel = Coord::default();
      velocities.push(vel);
  }

  let mut xiter = 0;
  let mut yiter = 0;
  let mut ziter = 0;

  let mut output1 = 0;

  for i in 0..1000000
  {
    for j in 0..moons.len()
    {
      for k in j..moons.len()
      {
        if moons[j].x<moons[k].x
        {
          velocities[j].x+=1;
          velocities[k].x-=1;
        } 
        else if moons[j].x>moons[k].x
        {
          velocities[j].x-=1;
          velocities[k].x+=1;
        } 

        if moons[j].y<moons[k].y
        {
          velocities[j].y+=1;
          velocities[k].y-=1;
        }
        else if moons[j].y>moons[k].y
        {
          velocities[j].y-=1;
          velocities[k].y+=1;
        }

        if moons[j].z<moons[k].z
        {
          velocities[j].z+=1;
          velocities[k].z-=1;
        }
        else if moons[j].z>moons[k].z
        {
          velocities[j].z-=1;
          velocities[k].z+=1;
        }
      }
    }
    
    let mut xcount=0;
    let mut ycount=0;
    let mut zcount=0;
    for j in 0..moons.len()
    {
      moons[j].x+=velocities[j].x;
      moons[j].y+=velocities[j].y;
      moons[j].z+=velocities[j].z;

      if velocities[j].x==0 && moons[j].x==omoons[j].x
      {
        xcount+=1;
      }

      if velocities[j].y==0 && moons[j].y==omoons[j].y
      {
        ycount+=1;
      }

      if velocities[j].z==0 && moons[j].z==omoons[j].z
      {
        zcount+=1;
      }
    }
  
    if xcount==moons.len() && xiter == 0 { xiter = i; }
    if ycount==moons.len() && yiter == 0 { yiter = i; }
    if zcount==moons.len() && ziter == 0 { ziter = i; }

    if xiter != 0 && yiter != 0 && ziter !=0 { break; }

    if i==999
    {
      for i in 0..moons.len()
      {
        output1 += (moons[i].x.abs() + moons[i].y.abs() + moons[i].z.abs()) * (velocities[i].x.abs() + velocities[i].y.abs() + velocities[i].z.abs());
      }
    }
  }

  let xi = fi64(xiter+1);
  let yi = fi64(yiter+1);
  let zi = fi64(ziter+1);

  let mut x = xi.clone();
  let mut y = yi.clone();
  let mut z = zi.clone();
  
  let _zero = fi64(0);

  let mut output2 = fi64(0);
  loop
  {
    if x<y { x=x+xi.clone(); }
    if x<z { x=x+xi.clone(); }
    if y<x { y=y+yi.clone(); }
    if y<z { y=y+yi.clone(); }
    if z<x { z=z+zi.clone(); }
    if z<y { z=z+zi.clone(); }
    if x==y && x==z
    {
      output2 = x;
      break;
    }
  }

  return (output1.to_string(),output2.to_str_radix(10));
}
