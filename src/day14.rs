use std::collections::HashMap;
use crate::util::*;

struct Material
{
  amount:i64,
  name:String,
}

impl Default for Material
{
  fn default() -> Material
  {
    Material
    {
      amount:0,
      name:String::new()
    }
  }
}

struct Reaction
{
  inputs:Vec<Material>,
  output:Material
}

impl Default for Reaction
{
  fn default() -> Reaction
  {
    Reaction
    {
      inputs:Vec::new(),
      output:Material::default()
    }
  }
}

fn doreaction(index:i64,reactions:&HashMap<String,Reaction>, inventory:&mut HashMap<String,i64>, reactionname:&str, totalore:&mut i64, oreremaining:&mut i64) -> bool
{

  let reaction = &reactions[reactionname];

  // use inventory
  for i in 0..reaction.inputs.len()
  {
    *inventory.get_mut(&reaction.inputs[i].name).unwrap() -= reaction.inputs[i].amount;
  }

  let mut exiting = false;

  // replenish inventory
  for i in 0..reaction.inputs.len()
  {
    loop
    {
      if inventory[&reaction.inputs[i].name] < 0
      {
        if reaction.inputs[i].name=="ORE"
        {
          *inventory.get_mut("ORE").unwrap() += reaction.inputs[i].amount.clone();
          *totalore+=reaction.inputs[i].amount.clone();

          if *oreremaining >= reaction.inputs[i].amount
          {
            *oreremaining-=reaction.inputs[i].amount.clone();
          }
          else
          {
            exiting = true;
            break;
          }        
        }
        else
        {
          exiting = doreaction(index+1, &reactions, inventory, &reaction.inputs[i].name, totalore, oreremaining);
        }
      }
      else
      {
        break;
      }
    }

    if exiting { break; }
  }

  if !exiting
  {
    // add back to inventory
    *inventory.get_mut(&reaction.output.name).unwrap() += reaction.output.amount.clone();
  }
/*
  for i in 0..index
  {
    print!(" ");
  }

  // use inventory
  print!("{}: ", reactionname);
  for (k,v) in inventory.iter()
  {
    print!("{} {}, ", k,v);
  }
  println!("");*/

  return exiting;
}

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut inventory:HashMap<String,i64> = HashMap::default();
  let mut reactions:HashMap<String,Reaction> = HashMap::default();

  for _i in 0..lines.len()
  {
    let parts:Vec<&str> = lines[_i].split('=').collect();

    let mut reaction = Reaction::default();

    if parts.len()>1
    {
      let inputparts:Vec<&str> = parts[0].split(',').collect();
      let output = &parts[1][1..];

      for _j in 0..inputparts.len()
      {
        let parts2:Vec<&str> = inputparts[_j].trim().split(' ').collect();

        let mut material = Material::default();
        material.amount = parts2[0].parse::<i64>().unwrap();
        material.name = parts2[1].to_string();

        if !inventory.contains_key(parts2[1])
        {
          inventory.insert(material.name.clone(),0);
        }

        reaction.inputs.push(material);        
      }

      let parts3:Vec<&str> = output.trim().split(' ').collect();

      let mut material = Material::default();
      material.amount = parts3[0].parse::<i64>().unwrap();
      material.name = parts3[1].to_string();

      reaction.output = material;
    }

    reactions.insert(reaction.output.name.clone(), reaction);    
  }

  inventory.insert("FUEL".to_string(),0);

  let index = 0;
  let mut totalore = 0;
  let mut oreremaining = 1000000000000;

  doreaction(index,&reactions, &mut inventory,"FUEL", &mut totalore, &mut oreremaining);
  let output1 = totalore;

  let mut firstinv = HashMap::new();
  firstinv = inventory.clone();

  loop
  {
    if doreaction(index,&reactions, &mut inventory,"FUEL", &mut totalore, &mut oreremaining) { break; }

    let mut m = true;
    for (k,v) in inventory.iter()
    {
      if k != "FUEL" && k != "ORE"
      {
        if firstinv[k] != *v
        {
          m = false;
          break;
        }
      }
    } 

    if m
    {
      println!("{} {}",totalore, inventory["FUEL"]);
    }

    /*
    let mut allzeroes = true;
    for (k,v) in inventory.iter()
    {
      if k != "FUEL" && k != "ORE"
      {
        if *v != 0
        {
          allzeroes = false;
          break;
        }
      }
    }

    if allzeroes
    {
      println!("{} {} {}", inventory["FUEL"], oreremaining, totalore);
    }*/
  }

  let output2 = inventory["FUEL"];

  return ( output1.to_string(),output2.to_string() );
}
