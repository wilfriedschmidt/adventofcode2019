use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
extern crate num_bigint;
extern crate num_traits;

pub fn readfile(filename:&str) -> Vec<u8>
{
  let mut file = match File::open(&filename)
  {
    Err(why) => panic!("couldn't open {} because: {}", filename, why.description()),
    Ok(file) => file,
  };

  let mut payload = Vec::new();
  match file.read_to_end(&mut payload)  
  {
    Err(why) => panic!("couldn't read {} because: {}", filename, why.description()),
    Ok(payload) => payload,
  };

  return payload;
}

pub fn runprogram(program:&mut Vec::<i64>, instpointer:&mut usize, inputs:&Vec::<i64>, inputindex:&mut usize, output:&mut i64) -> i32
{
  let mut retval = 0;
  let mut relativebase = 0;
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

      if mode1==0 { dparam1 = program[param1 as usize]; } else if mode1==2 { dparam1 = program[(relativebase + param1) as usize]; }       
      if mode2==0 { dparam2 = program[param2 as usize]; } else if mode2==2 { dparam2 = program[(relativebase + param2) as usize]; }

      if instruction==1 { program[dest] = dparam1 + dparam2; }
      else if instruction==2 { program[dest] = dparam1 * dparam2; }
      else if instruction==7 { if dparam1 < dparam2 { program[dest] = 1; } else { program[dest] = 0; } }
      else if instruction==8 { if dparam1 == dparam2 { program[dest] = 1; } else { program[dest] = 0; } }
    
      *instpointer+=4;
    }
    else if instruction==3 || instruction==4 || instruction==9
    {
      let param1 = program[*instpointer+1];

      let mut dparam1 = param1;      
      if mode1==0 { dparam1 = program[param1 as usize]; } else if mode1==2 { dparam1 = program[(relativebase + param1) as usize]; }          

      if instruction==3
      {
        if *inputindex>=inputs.len()
        {
          // not done
          //println!("ran out of inputs");
          retval = 0;
          break;
        }

        if mode1==0 { program[param1 as usize] = inputs[*inputindex]; } else if mode1==2 { program[(relativebase + param1) as usize] = inputs[*inputindex]; }

        *inputindex+=1;
      }
      else if instruction==4 { *output = dparam1; }
      else if instruction==9 { relativebase += dparam1; }

      *instpointer+=2;
    }

    else if instruction==5 || instruction==6
    {
      let param1 = program[*instpointer+1];
      let param2 = program[*instpointer+2];

      let mut dparam1 = param1;
      let mut dparam2 = param2;

      if mode1==0 { dparam1 = program[param1 as usize]; } else if mode1==2 { dparam1 = program[(relativebase + param1) as usize]; }          
      if mode2==0 { dparam2 = program[param2 as usize]; } else if mode2==2 { dparam2 = program[(relativebase + param2) as usize]; }         

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

pub fn fi64(value:i64) -> num_bigint::BigInt
{
  return num_traits::FromPrimitive::from_i64(value).unwrap();
}

pub fn readi64(memory:&HashMap::<num_bigint::BigInt,num_bigint::BigInt>, pointer:&num_bigint::BigInt) -> i64
{
  if memory.contains_key(pointer)
  {
    return num_traits::ToPrimitive::to_i64( memory.get(pointer).unwrap() ).unwrap();
  }
  else
  {
    return 0;
  }
}

pub fn readbigint(memory:&HashMap::<num_bigint::BigInt,num_bigint::BigInt>, pointer:&num_bigint::BigInt) -> num_bigint::BigInt
{
  if memory.contains_key(pointer)
  {
    return memory[pointer].clone();
  }
  else
  {
    return num_traits::FromPrimitive::from_i64(0).unwrap();
  }
}

pub fn writebigint(memory:&mut HashMap::<num_bigint::BigInt,num_bigint::BigInt>, pointer:&num_bigint::BigInt, value:&num_bigint::BigInt)
{
  memory.insert( pointer.clone(), value.clone() );
}

pub fn writei64(memory:&mut HashMap::<num_bigint::BigInt,num_bigint::BigInt>, pointer:&num_bigint::BigInt, value:i64)
{
  memory.insert( pointer.clone(), fi64(value) );
}

pub fn readwithmode(program:&HashMap::<num_bigint::BigInt,num_bigint::BigInt>,
                    mode:i64,
                    param:&num_bigint::BigInt,
                    relativebase:&num_bigint::BigInt) -> num_bigint::BigInt
{

  let mut retval = param.clone();
  if mode==0 
  { 
    retval = readbigint( &program, &param ); 
  }
  else if mode==2
  { 
    retval = readbigint( &program, &(relativebase.clone() + param) ); 
  }

  return retval;
}        

pub fn runprogram_bigint(program:&mut HashMap::<num_bigint::BigInt,num_bigint::BigInt>, 
                         instpointer:&mut num_bigint::BigInt, 
                         inputs:&Vec::<num_bigint::BigInt>, 
                         inputindex:&mut usize, 
                         output:&mut num_bigint::BigInt) -> i32
{
  let mut retval = -1;
  let mut relativebase = fi64(0);
  loop
  {

    let wholeinstruction = readi64(&program,instpointer);
    let instruction = wholeinstruction % 100;
    let mode1 = wholeinstruction / 100 % 10;
    let mode2 = wholeinstruction / 1000 % 10;
    let mode3 = wholeinstruction / 10000 % 10;

    let address1 = instpointer.clone() + 1;
    let address2 = instpointer.clone() + 2;
    let address3 = instpointer.clone() + 3;

    if instruction==1 || instruction==2 || instruction==7 || instruction==8
    {
      let param1 = readbigint( &program, &address1 );
      let param2 = readbigint( &program, &address2 );
      let dest = readbigint( &program, &address3 );
  
      let mut ddest = dest.clone();

      let dparam1 = readwithmode( &program, mode1, &param1, &relativebase );
      let dparam2 = readwithmode( &program, mode2, &param2, &relativebase );
      if mode3==2 { ddest = relativebase.clone() + dest; }

      if instruction==1 { writebigint( program, &ddest, &(dparam1 + dparam2) ); }
      else if instruction==2 { writebigint( program, &ddest, &(dparam1 * dparam2) ); }
      else if instruction==7 { if dparam1 < dparam2 { writei64( program, &ddest, 1 ); } else { writei64( program, &ddest, 0 ); } }
      else if instruction==8 { if dparam1 == dparam2 { writei64( program, &ddest, 1 ); } else { writei64( program,& ddest, 0 ); } }
    
      *instpointer+=4;
    }
    else if instruction==3 || instruction==4 || instruction==9
    {
      let param1 = readbigint( &program, &address1 );

      let mut rparam1 = relativebase.clone();
      rparam1+=param1.clone();

      let dparam1 = readwithmode( &program, mode1, &param1, &relativebase );

      if instruction==3
      {
        if *inputindex>=inputs.len()
        {
          // not done
          println!("ran out of inputs");
          retval = 0;
        }

        if mode1==0 { writebigint( program, &param1, &inputs[*inputindex].clone() ); } else if mode1==2 { writebigint( program, &rparam1, &inputs[*inputindex].clone()); }

        *inputindex+=1;
      }
      else if instruction==4 
      { 
        *output = dparam1; 
        //print!("{} ",*output);
        retval = 0;
      }
      else if instruction==9 { relativebase += dparam1; }

      *instpointer+=2;
    }

    else if instruction==5 || instruction==6
    {
      let param1 = readbigint(&program,&address1);
      let param2 = readbigint(&program,&address2);

      let dparam1 = readwithmode( &program, mode1, &param1, &relativebase );
      let dparam2 = readwithmode( &program, mode2, &param2, &relativebase );

      if instruction==5 { if dparam1 != fi64(0) { *instpointer = dparam2.clone(); } else { *instpointer+=3; } }
      if instruction==6 { if dparam1 == fi64(0) { *instpointer = dparam2.clone(); } else { *instpointer+=3; } }
    }

    else if instruction==99
    {
      retval = 1;
    }

    else
    {
      println!("bad opcode ip: {}, code: {} ", instpointer.to_str_radix(10), readbigint(&program, &*instpointer).to_str_radix(10) );
      retval = 1;
      break;
    }

    if retval != -1
    { 
      break;
    }
  }

  return retval;
}
