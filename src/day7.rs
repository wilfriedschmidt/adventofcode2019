use crate::util::*;

fn generate_phases(offset:i64) -> Vec<Vec<i64>>
{
  // generate outputs
  let mut allphases = Vec::new();
 
  for i in 0..5
  {
    for j in 0..5
    {
      if j==i { continue; }
      for k in 0..5
      {
        if k==j { continue; }
        if k==i { continue; }
        for l in 0..5
        {
          if l==k { continue; }
          if l==j { continue; }
          if l==i { continue; }
          for m in 0..5
          {
            if m==l { continue; }
            if m==k { continue; }
            if m==j { continue; }
            if m==i { continue; }

            let mut phases = Vec::new();
            phases.resize(5,0);
            phases[0] = i + offset; 
            phases[1] = j + offset;
            phases[2] = k + offset;
            phases[3] = l + offset;
            phases[4] = m + offset;
                       
            allphases.push(phases);
          }
        }
      }
    }
  }

  return allphases;
}

fn run_phases(allphases:&Vec<Vec<i64>>, ints:&Vec<i64>) -> i64
{
  let mut largest = 0;

  // for each combo, run programs
  for p in 0..allphases.len()
  {
    // initialize inputs
    let mut inputs:[Vec<i64>;5] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
    let mut inputindexes:[usize;5] = [0,0,0,0,0];

    inputs[0].push(allphases[p][0]);
    inputs[0].push(0);
    
    inputs[1].push(allphases[p][1]);
    inputs[2].push(allphases[p][2]);
    inputs[3].push(allphases[p][3]);
    inputs[4].push(allphases[p][4]);

    // initialize programs
    let mut programs:[Vec<i64>;5] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
    let mut instpointers:[usize;5] = [0,0,0,0,0];

    for i in 0..5
    {
      programs[i].resize(ints.len(),0);
      for j in 0..ints.len()
      {
        programs[i][j] = ints[j];
      }
    }

    // run a program until an output is created
    let mut currentprog = 0;    
    loop
    {
      let mut output = 0;
      let retval = runprogram(&mut programs[currentprog], &mut instpointers[currentprog], &inputs[currentprog], &mut inputindexes[currentprog], &mut output);

      currentprog+=1;
      currentprog = currentprog % 5;

      inputs[currentprog].push(output);

      // if the last program halted, and the next one is the first one, then stop
      if retval==1 && currentprog==0
      {
        if output > largest { largest = output; }
        break;
      }
    }
  }

  return largest;
}

pub fn go(filename:&str)
{
  // load program
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

  let allphases = generate_phases(0);
  println!("part 1 largest output: {}",run_phases(&allphases,&ints));
  
  let allfeedbackphases = generate_phases(5);
  println!("part 2 largest output: {}",run_phases(&allfeedbackphases,&ints));
}
