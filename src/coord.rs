pub struct Coord
{
  pub x:i64,
  pub y:i64,
  pub z:i64,
}

impl Default for Coord
{
  fn default() -> Coord
  {
    Coord
    {
      x:0,
      y:0,
      z:0
    }
  }
}
