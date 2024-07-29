fn collatz_count (n : u128) -> u32 {
  if n == 1 {
    return 1;
  }

  if n % 2 == 0 {
    return 1 + collatz_count(n/2);
  }
  else {
    return 1 + collatz_count(3*n + 1)
  }
}

pub fn main () {
  let mut max = 0;
  let mut max_i = 0;
  
  for n in 1..1000000 {
    let c = collatz_count(n);

    if c > max {
      max = c;
      max_i = n;
    }
  }

  println!("{}", max_i);
}