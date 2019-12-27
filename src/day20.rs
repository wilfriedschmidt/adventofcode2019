use crate::util::*;
use crate::grid::*;

#[derive(Clone)]
struct PortalLink
{
  portalindex:usize,
  steps:i32
}

impl Default for PortalLink
{
  fn default() -> PortalLink
  {
    PortalLink
    {
      portalindex:0,
      steps:0
    }
  }
}


#[derive(Clone)]
struct Portal
{
  name:String,
  x:i32,
  y:i32,
  links:Vec<PortalLink>,
  endindex:usize,
  outside:bool,
}

impl Default for Portal
{
  fn default() -> Portal
  {
    Portal
    {
      name:String::new(),
      x:0,
      y:0,
      links:Vec::new(),
      endindex:0,
      outside:false,
    }
  }
}


fn isportal(tile:char) -> bool
{
  if tile != '#' && tile != '.' && tile != ' ' && tile != '*' && tile != '@' { return true; }
  else { return false; }
}

fn isopen(tile:char) -> bool
{
  if tile=='.' { return true; }
  else { return false; }
}

fn addifnotpresent(portal:&Portal, portals:&mut Vec<Portal>)
{
  let mut found = false;
  for i in 0..portals.len()
  {
    let p = &portals[i];
    if p.name==portal.name && p.x==portal.x && p.y==portal.y { found = true; break; }
  }

  if !found { portals.push(portal.clone()); }
}

fn findbycoord(x:i32,y:i32,portals:&Vec<Portal>) -> usize
{
  for i in 0..portals.len()
  {
    let portal = &portals[i];
    if x==portal.x && y==portal.y { return i; }
  }

  println!("find failed {} {}", x,y);
  return 0;
}

fn linkportals(startindex:usize, x:i32, y:i32, steps:i32,portals:&mut Vec<Portal>)
{
  let endindex = findbycoord(x,y,portals);

  if endindex != startindex
  {
    let mut found = false;
    for i in 0..portals[startindex].links.len()
    { 
      if portals[startindex].links[i].portalindex == endindex { found = true; break; }
    }

    if !found
    {
      let mut link = PortalLink::default();
      link.portalindex = endindex;
      link.steps = steps;
      portals[startindex].links.push(link);
    }
  }
}

fn floodfill(grid:&mut Grid<char>, startportalindex:usize, portals:&mut Vec<Portal>)
{
  let x = portals[startportalindex].x;
  let y = portals[startportalindex].y;
  grid.put(x,y,'@');

  let mut steps = 0;
  loop
  {
    // mark
    let mut putval = false;
    for y in 0..grid.height
    {
      for x in 0..grid.width
      {
        if *grid.get(x,y)=='@'
        {
          let up = *grid.get(x,y-1);
          let down = *grid.get(x,y+1);
          let left = *grid.get(x-1,y);
          let right = *grid.get(x+1,y);

          if isopen(up) { grid.put(x,y-1,'*'); putval = true; }
          else if isportal(up) { linkportals(startportalindex, x,y, steps, portals); }

          if isopen(down) { grid.put(x,y+1,'*'); putval = true; }
          else if isportal(down) { linkportals(startportalindex, x,y, steps, portals); }

          if isopen(left) { grid.put(x-1,y,'*'); putval = true; }
          else if isportal(left) { linkportals(startportalindex, x,y, steps, portals); }

          if isopen(right) { grid.put(x+1,y,'*'); putval = true; }
          else if isportal(right) { linkportals(startportalindex, x,y, steps, portals); }
        }
      }
    }
    if !putval { break; }

    steps+=1;

    // sweep
    for y in 0..grid.height
    {
      for x in 0..grid.width
      {
        if *grid.get(x,y)=='*' { grid.put(x,y,'@'); }
      }
    }
  }
}

fn search(currentindex:usize, endindex:usize, steps:i32,  portals:&Vec<Portal>, visited:&mut Vec<usize>, mindist:&mut i32)
{
  for i in 0..visited.len()
  {
    if visited[i]==currentindex { return; }
  }
  visited.push(currentindex);

  for i in 0..portals[currentindex].links.len()
  {
    let link = &portals[currentindex].links[i];
    if link.portalindex==endindex 
    {
      if steps + link.steps < *mindist { *mindist = steps + link.steps; }
    }
    else
    {
      // teleport and keep searching
      search( portals[link.portalindex].endindex, endindex, steps + link.steps + 1, portals, visited, mindist);
    }  
  }
  visited.pop();
}

fn searchlevel(indent:String, level:i32, currentindex:usize, endindex:usize, steps:i32,  insideportals:&Vec<Portal>, outsideportals:&Vec<Portal>, visited:&mut Vec<(usize,i32)>, mindist:&mut i32)
{
  //println!("{} level: {} index: {} {}", indent, level, insideportals[currentindex].name, currentindex);
  
/*
  for i in 0..visited.len()
  {
    if visited[i].0==currentindex && (visited[i].1+10)<=level { return; }
  }*/

  if steps > 8000 { return; }

  /*
  if visited.len()>3
  {
    for i in 0..visited.len()-3
    {
      for j in i+1..visited.len()-4
      {
        let mut matched = true;
        for k in 0..3
        {
          if visited[i+k] != visited[j+k] { matched = false; }
        }

        if matched { println!("found repeat"); return; }
      }
    }
  }*/

  visited.push((currentindex,level));

  // look for cycle larger than

  let mut portals = outsideportals;
  if level>0
  {
    portals = insideportals;
  }

  for i in 0..portals[currentindex].links.len()
  {
    //println!("scanning: {}", portals[portals[currentindex].links[i].portalindex].name);
    let link = &portals[currentindex].links[i];
    if link.portalindex==endindex 
    {
      println!("made it {} ",steps);
      if (steps + link.steps) < *mindist { *mindist = steps + link.steps; }
    }
    else
    {
      let mut nextlevel = level;
      if portals[link.portalindex].outside { nextlevel-=1; } else { nextlevel+=1; }
      
      searchlevel( indent.clone(), nextlevel, portals[link.portalindex].endindex, endindex, steps + link.steps + 1, insideportals, outsideportals, visited, mindist);
    }  
  }

  visited.pop();
}


pub fn go(filename:&str) -> (String,String)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let dim = 150;
  let mut grid:Grid<char> = Grid::default();
  grid.width = dim;
  grid.height = dim;
  grid.data.resize((dim*dim) as usize,' ');

  let mut maxx = 0;
  let mut maxy = 0;

  for y in 0..lines.len()
  {
    let chars:Vec<char> = lines[y].chars().collect();
    for x in 0..chars.len()
    {
      grid.put(x as i32,y as i32, chars[x]);

      if x > maxx { maxx = x; }
    }
    if y > maxy { maxy = y; }
  }

  let mut portals = Vec::new();

  for y in 0..grid.height
  {
    for x in 0..grid.width
    {
      let tile = *grid.get(x,y);
      if isportal(tile)
      {
        // read in portal
        let up = *grid.get(x,y-1);
        let down = *grid.get(x,y+1);
        let left = *grid.get(x-1,y);
        let right = *grid.get(x+1,y);

        if isportal(up)
        {
          let mut portal = Portal::default();
          portal.name.push(up);
          portal.name.push(tile);
          portal.x = x;
          if y<=2 || y>=(maxy-2) as i32 { portal.outside = true; }
          if isopen(down) { portal.y = y+1; } else { portal.y = y-2; }
          addifnotpresent(&portal,&mut portals);
        }

        if isportal(down)
        {
          let mut portal = Portal::default();
          portal.name.push(tile);
          portal.name.push(down);
          portal.x = x;
          if y<=2 || y>=(maxy-2) as i32 { portal.outside = true; }
          if isopen(up) { portal.y = y-1; } else { portal.y = y+2; }
          addifnotpresent(&portal,&mut portals);
        }

        if isportal(left)
        {
          let mut portal = Portal::default();
          portal.name.push(left);
          portal.name.push(tile);
          if x<=2 || x>=(maxx-2) as i32 { portal.outside = true; }
          if isopen(right) { portal.x = x+1; } else { portal.x = x-2; }
          portal.y = y;
          addifnotpresent(&portal,&mut portals);
        }

        if isportal(right)
        {
          let mut portal = Portal::default();
          portal.name.push(tile);
          portal.name.push(right);
          if x<=2 || x>=(maxx-2) as i32 { portal.outside = true; }
          if isopen(left) { portal.x = x-1; } else { portal.x = x+2; }
          portal.y = y;
          addifnotpresent(&portal,&mut portals);
        }
      }
    }
  }

  grid.print();

  let mut insidegrid = grid.clone();
  let mut insideportals = portals.clone();
  for i in 0..insideportals.len()
  {
    if insideportals[i].name == "AA" || insideportals[i].name == "ZZ"
    {
      insidegrid.put( insideportals[i].x, insideportals[i].y, '#' );
    }
  }

  for i in 0..insideportals.len()
  {
    if insideportals[i].name != "AA" && insideportals[i].name != "ZZ"
    {
      floodfill(&mut insidegrid.clone(), i, &mut insideportals);
    }   
  }

  let mut outsidegrid = grid.clone();
  let mut outsideportals = portals.clone();
  for i in 0..outsideportals.len()
  {
    if outsideportals[i].name != "AA" && outsideportals[i].name != "ZZ" && outsideportals[i].outside
    {
      outsidegrid.put( insideportals[i].x, insideportals[i].y, '#' );
    }
  }

  for i in 0..outsideportals.len()
  {
    if !outsideportals[i].outside || outsideportals[i].name == "AA" || outsideportals[i].name == "ZZ"
    {
      floodfill(&mut outsidegrid.clone(), i, &mut outsideportals);   
    }  
  }

  let mut startindex = 0;
  let mut endindex=0;
  for i in 0..outsideportals.len()
  {
    if outsideportals[i].name=="AA" { startindex = i; }
    if outsideportals[i].name=="ZZ" { endindex = i; }

    for j in 0..outsideportals.len()
    {
      if i != j && outsideportals[i].name==outsideportals[j].name
      {
        outsideportals[i].endindex = j;
        outsideportals[j].endindex = i;
      }
    }
  }

  for i in 0..insideportals.len()
  {
    if insideportals[i].name=="AA" { startindex = i; }
    if insideportals[i].name=="ZZ" { endindex = i; }

    for j in 0..insideportals.len()
    {
      if i != j && insideportals[i].name==insideportals[j].name
      {
        insideportals[i].endindex = j;
        insideportals[j].endindex = i;
      }
    }
  }

  //insidegrid.print();
  
  println!("inside portals");
  for i in 0..insideportals.len()
  {
    let portal = &insideportals[i];
  
    println!("{}: {} {} {} {} {}", i, portal.name, portal.x, portal.y, portal.endindex, portal.outside);
    for j in 0.. insideportals[i].links.len()
    {
      println!("  {} {}", insideportals[insideportals[i].links[j].portalindex].name, insideportals[i].links[j].steps);
    }  
  }


  //outsidegrid.print();

  println!("\noutside portals");
  for i in 0..outsideportals.len()
  {
    let portal = &outsideportals[i];
  
    println!("{}: {} {} {} {} {}", i, portal.name, portal.x, portal.y, portal.endindex, portal.outside);
    for j in 0.. outsideportals[i].links.len()
    {
      println!("  {} {}", outsideportals[outsideportals[i].links[j].portalindex].name, outsideportals[i].links[j].steps);
    }  
  }

 

  println!("\n{} {}\n", startindex, endindex);

  let mut visited:Vec<(usize,i32)> = Vec::new();
  let mut mindist:i32 = 1000000;
  searchlevel(" ".to_string(), 0, startindex, endindex, 0, &insideportals, &outsideportals, &mut visited, &mut mindist);

  println!("{}", mindist);

  return ( String::new(), String::new() );
}
