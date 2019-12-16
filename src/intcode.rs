use std::collections::HashMap;
use crate::util::*;
extern crate num_bigint;
extern crate num_traits;

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

pub struct Program
{
  program:HashMap::<num_bigint::BigInt,num_bigint::BigInt>, 
  instpointer:num_bigint::BigInt, 
  relativebase:num_bigint::BigInt
}

impl Default for Program
{
  fn default() -> Program
  {
    Program
    {
      program:HashMap::new(),
      instpointer:fi64(0),
      relativebase:fi64(0)
    }
  }
}

impl Program
{
  fn readi64(&mut self) -> i64
  {
    if self.program.contains_key(&self.instpointer)
    {
      return num_traits::ToPrimitive::to_i64( self.program.get(&self.instpointer).unwrap() ).unwrap();
    }
    else
    {
      return 0;
    }
  }

  fn readbigint(&mut self, pointer:&num_bigint::BigInt) -> num_bigint::BigInt
  {
    if self.program.contains_key(pointer)
    {
      return self.program[pointer].clone();
    }
    else
    {
      return num_traits::FromPrimitive::from_i64(0).unwrap();
    }
  }

  fn writebigint(&mut self, pointer:&num_bigint::BigInt, value:&num_bigint::BigInt)
  {
    self.program.insert( pointer.clone(), value.clone() );
  }

  fn writei64(&mut self, pointer:&num_bigint::BigInt, value:i64)
  {
    self.program.insert( pointer.clone(), fi64(value) );
  }

  fn readwithmode(&mut self, mode:i64, param:&num_bigint::BigInt) -> num_bigint::BigInt
  {

    let mut retval = param.clone();
    if mode==0 
    { 
      retval = self.readbigint( &param ); 
    }
    else if mode==2
    { 
      retval = self.readbigint( &(self.relativebase.clone() + param) ); 
    }

    return retval;
  }        

  pub fn load(&mut self, filename:&str)
  {
    // load program
    let payload:Vec<u8> = readfile(filename);
    let payloadstr:String = String::from_utf8(payload).unwrap();
    let parts:Vec<&str> = payloadstr.split(',').collect();

    for i in 0..parts.len()
    {
      if parts[i].trim().len()>0
      {
        let opcode = &parts[i].trim().parse::<i64>().unwrap();
        self.program.insert( fi64(i as i64), fi64(*opcode) );
      }
    }
  }

  pub fn step(&mut self, inputs:&Vec::<num_bigint::BigInt>, output:&mut num_bigint::BigInt) -> i32
  {
    let mut inputindex = 0;
    let mut retval = -1;
    loop
    {
      let wholeinstruction = self.readi64();
      let instruction = wholeinstruction % 100;
      let mode1 = wholeinstruction / 100 % 10;
      let mode2 = wholeinstruction / 1000 % 10;
      let mode3 = wholeinstruction / 10000 % 10;

      let address1 = self.instpointer.clone() + 1;
      let address2 = self.instpointer.clone() + 2;
      let address3 = self.instpointer.clone() + 3;

      if instruction==1 || instruction==2 || instruction==7 || instruction==8
      {
        let param1 = self.readbigint( &address1 );
        let param2 = self.readbigint( &address2 );
        let dest = self.readbigint( &address3 );

        let mut ddest = dest.clone();

        let dparam1 = self.readwithmode( mode1, &param1 );
        let dparam2 = self.readwithmode( mode2, &param2 );
        if mode3==2 { ddest = self.relativebase.clone() + dest.clone(); }

        if instruction==1 { self.writebigint( &ddest, &(dparam1 + dparam2) ); }
        else if instruction==2 { self.writebigint( &ddest, &(dparam1 * dparam2) ); }
        else if instruction==7 { if dparam1 < dparam2 { self.writei64( &ddest, 1 ); } else { self.writei64( &ddest, 0 ); } }
        else if instruction==8 { if dparam1 == dparam2 { self.writei64( &ddest, 1 ); } else { self.writei64( &ddest, 0 ); } }
      
        self.instpointer+=4;
      }
      else if instruction==3 || instruction==4 || instruction==9
      {
        let param1 = self.readbigint( &address1 );

        if instruction==3
        {
          if inputindex>=inputs.len()
          {
            // not done
            //println!("ran out of inputs");
            retval = 0;
          }

          let mut rparam1 = self.relativebase.clone();
          rparam1+=param1.clone();

          if mode1==0 { self.writebigint( &param1, &inputs[inputindex].clone() ); } 
          else if mode1==2 { self.writebigint( &rparam1, &inputs[inputindex].clone()); }

          inputindex+=1;
        }
        else if instruction==4 
        { 
          let dparam1 = self.readwithmode( mode1, &param1 );
          *output = dparam1;
          retval = 0;
        }
        else if instruction==9 
        { 
          let dparam1 = self.readwithmode( mode1, &param1 );
          self.relativebase += dparam1; 
        }

        self.instpointer+=2;
      }

      else if instruction==5 || instruction==6
      {
        let param1 = self.readbigint(&address1);
        let param2 = self.readbigint(&address2);

        let dparam1 = self.readwithmode( mode1, &param1 );
        let dparam2 = self.readwithmode( mode2, &param2 );

        if instruction==5 { if dparam1 != fi64(0) { self.instpointer = dparam2.clone(); } else { self.instpointer+=3; } }
        if instruction==6 { if dparam1 == fi64(0) { self.instpointer = dparam2.clone(); } else { self.instpointer+=3; } }
      }

      else if instruction==99
      {
        retval = 1;
      }

      else
      {
        println!("bad opcode ip: {}, code: {} ", self.instpointer.to_str_radix(10), self.readbigint(&self.instpointer.clone()).to_str_radix(10) );
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
}
