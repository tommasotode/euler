use num::BigUint;

fn fib(size : usize) -> Vec<BigUint> {
  let mut v = vec![BigUint::from(0 as u32); size];
  v[0] = BigUint::from(1 as u32);
  v[1] = BigUint::from(1 as u32);

  for i in 2..v.len() {
    v[i] = &v[i-1] + &v[i-2];
  }

  v
}

fn count_digits(n : BigUint) -> usize {
  n.to_string().len()
}

pub fn main() {
  let limit = BigUint::pow(&BigUint::from(10 as u32), 999);
  for (i, n) in fib(5000).iter().enumerate() {
    if n >= &limit {
      println!("{} ({})", i+1, count_digits(n.clone()));
      break;
    }
  }
}