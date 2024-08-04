use num::{bigint::BigInt, pow};

pub fn main() {
  let base = BigInt::from(2);
  let result = pow(base, 1000);
  
  let mut sum = 0;
  for c in result.to_string().chars() {
    sum += c.to_digit(10).unwrap();
  }
  println!("{}", sum);
}
