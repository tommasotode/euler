pub fn main() {

  let mut s = 0;
  let mut sp = 0;
  for x in 1..=100 {
    s += x;
    sp += x*x;
  }
  
  println!("{}", (s*s - sp));
}