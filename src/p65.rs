use std::mem::swap;
use num::integer::lcm;
use num::integer::gcd;

fn fraction_sum(f1 : (u128, u128), f2 : (u128, u128)) -> (u128, u128) {
  let mut res : (u128, u128) = (0,0);

  res.1 = lcm(f1.1, f2.1);
  res.0 = (res.1/f1.1)*f1.0 + (res.1/f2.1)*f2.0;

  let gcd = gcd(res.0, res.1);
  res.1 = res.1/gcd;
  res.0 = res.0/gcd;

  res
}

fn inverted_sum(f1 : (u128, u128), f2 : (u128, u128)) -> (u128, u128) {
  let mut res = fraction_sum(f1, f2);
  swap(&mut res.0, &mut res.1);
  res
}

fn get_k_convergent(k : u128, count : u128) -> (u128, u128) {
  let mut val = (1,1);
  if count % 3 == 0 {
    val.0 = 2*(count/3);
  }
  
  if k == 0 {
    return (1,1);
  }

  let conv = get_k_convergent(k-1, count+1);
  // since it is 1 / by ..., it's like doing ^-1, so i can just swap operators
  let a = inverted_sum(val, conv);

  a
}

pub fn main() {

  for i in 0..10 {
    let sum = fraction_sum((2,1), get_k_convergent(i, 2));
    println!("{:?}", sum);
  }
}