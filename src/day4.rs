pub fn go()
{
  let start:i32 = 125730;
  let end:i32 = 579381;

  let mut countpart1 = 0;
  let mut countpart2 = 0;

  for _i in start..end
  {
    let mut haspair = false;
    let mut isincreasing = true;
    let mut atleastonepair = false;

    let mut haspaircount = 0;
    
    let mut lastdigit:i32 = -1;
    let mut temp = _i;

    for _j in 0..6
    {
      let digit = temp%10;
      temp = temp/10;

      if lastdigit==-1 { lastdigit = digit; }
      else if digit==lastdigit
      {
        haspair = true;
        haspaircount+=1;
      }
      else
      {
        if haspaircount==1 { atleastonepair = true }
        haspaircount=0;
      }

      if digit>lastdigit { isincreasing = false; }

      lastdigit = digit;    
    }

    if isincreasing && haspair { countpart1+=1; }
    if isincreasing && (atleastonepair || haspaircount==1) { countpart2+=1; }
  }

  println!("{} {}",countpart1, countpart2);
}
