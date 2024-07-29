fn sieve(n: usize) -> Vec<usize> {
  let mut prime = vec![true; n + 1];
  let mut res = Vec::new();

  prime[0] = false;
  prime[1] = false;

  for p in 2..=n {
      if prime[p] {
          res.push(p);
          let mut mult = p * p;
          while mult <= n {
              prime[mult] = false;
              mult += p;
          }
      }
  }
  res
}

pub fn main() {
  let p = sieve(1000000);
  println!("{}", p[10001 - 1])
}
