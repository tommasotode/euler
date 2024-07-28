use num::integer::lcm;

pub fn main() {

  let mut l = lcm(2, 3);
  for i in 4..=20 {
    l = lcm(l, i);
  }

  println!("{}", l);
}