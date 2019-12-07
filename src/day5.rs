use crate::util::*;

fn runprogram(program:&mut Vec::<i64>, instpointer:&mut usize, inputs:&Vec::<i64>, inputindex:&mut usize, output:&mut i64) -> i32
{
  let mut retval = 0;
  loop
  {
    let instruction = program[*instpointer] % 100;
    let mode1 = program[*instpointer]/100 % 10;
    let mode2 = program[*instpointer]/1000 % 10;

    if instruction==1 || instruction==2 || instruction==7 || instruction==8
    {
      let param1 = program[*instpointer+1];
      let param2 = program[*instpointer+2];
      let dest = program[*instpointer+3] as usize;
  
      let mut dparam1 = param1;
      let mut dparam2 = param2;

      if mode1==0 { dparam1 = program[param1 as usize]; }          
      if mode2==0 { dparam2 = program[param2 as usize]; }

      if instruction==1 { program[dest] = dparam1 + dparam2; }
      else if instruction==2 { program[dest] = dparam1 * dparam2; }
      else if instruction==7 { if dparam1 < dparam2 { program[dest] = 1; } else { program[dest] = 0; } }
      else if instruction==8 { if dparam1 == dparam2 { program[dest] = 1; } else { program[dest] = 0; } }
    
      *instpointer+=4;
    }
    else if instruction==3 || instruction==4
    {
      let param1 = program[*instpointer+1];

      let mut dparam1 = param1;      
      if mode1==0 { dparam1 = program[param1 as usize]; }          

      if instruction==3
      {
        if *inputindex>=inputs.len()
        {
          // not done
          retval = 0;
          break;
        }

        program[param1 as usize] = inputs[*inputindex];
        *inputindex+=1;
      }
      else if instruction==4 { *output = dparam1; }

      *instpointer+=2;
    }

    else if instruction==5 || instruction==6
    {
      let param1 = program[*instpointer+1];
      let param2 = program[*instpointer+2];

      let mut dparam1 = param1;
      let mut dparam2 = param2;

      if mode1==0 { dparam1 = program[param1 as usize]; }          
      if mode2==0 { dparam2 = program[param2 as usize]; }          

      if instruction==5 { if dparam1 != 0 { *instpointer = dparam2 as usize; } else { *instpointer+=3; } }
      if instruction==6 { if dparam1 == 0 { *instpointer = dparam2 as usize; } else { *instpointer+=3; } }
    }

    else if instruction==99
    {
      retval = 1;
      break;
    }

    else
    {
      println!("bad opcode ip: {}, code: {} ", *instpointer, program[*instpointer]);
      break;
    }

    if *instpointer>=program.len()
    {
      println!("ran past end of program");
      break;
    }
  }

  return retval;
}

pub fn go(filename:&str)
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

  let mut program:Vec<i64> = Vec::new();
  program.resize(ints.len(), 0);

  // part 1
  for i in 0..ints.len()
  {
    program[i]=ints[i];
  }

  let mut instpointer = 0;
  let mut inputs = Vec::new();
  inputs.resize(1,1);

  let mut inputindex = 0;
  let mut output = 0;

  runprogram(&mut program, &mut instpointer, &inputs, &mut inputindex, &mut output); 

  println!("{}",output);


  // part 2
  for i in 0..ints.len()
  {
    program[i]=ints[i];
  }

  instpointer = 0;
  inputs[0] = 5;
  inputindex = 0;
  runprogram(&mut program, &mut instpointer, &inputs, &mut inputindex, &mut output); 

  println!("{}",output);
}
