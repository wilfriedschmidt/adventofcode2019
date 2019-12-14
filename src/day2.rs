use crate::util::*;

pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let parts:Vec<&str> = payloadstr.split(',').collect();

  let mut ints:Vec<i64> = Vec::new();
  ints.resize(parts.len()-1, 0);

  for i in 0..parts.len()-1
  {
    if parts[i].len()>0
    {
      let opcode = &parts[i].parse::<i64>().unwrap();
      ints[i] = *opcode;
    }
  }
  
  let mut tempints:Vec<i64> = Vec::new();
  tempints.resize(ints.len(), 0);

  let mut part1output = String::new();
  let mut part2output = String::new();

  for noun in 0..99
  {
    for verb in 0..99
    {
      for i in 0..ints.len()
      {
        tempints[i]=ints[i];
      }

      tempints[1] = noun;
      tempints[2] = verb;

      let mut index=0;
      loop
      {
        let instruction = tempints[index];
        let param1 = tempints[index+1] as usize;
        let param2 = tempints[index+2] as usize;
        let dest = tempints[index+3] as usize;

        if param1 >= tempints.len() || param2 >= tempints.len() || dest >= tempints.len()
        {
          // bad instructions, abort this run
          break;
        }

        if instruction==1
        {
          tempints[dest] = tempints[param1] + tempints[param2]; 
        }
        else if instruction==2
        {
          tempints[dest] = tempints[param1] * tempints[param2];
        }
        else if instruction==99
        {
          if noun==12 && verb==2
          {
            part1output = tempints[0].to_string();
          }

          if tempints[0]==19690720
          {
            part2output = (100*noun+verb).to_string();
          }

          break;    
        }
        else
        {
          println!("bad opcode {} {} ", index, ints[index]);
          break;
        }

        index+=4;
        if index>=tempints.len()
        {
          break;
        }
      }
    }
  }

  return ( part1output, part2output );
}
