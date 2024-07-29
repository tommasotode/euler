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

fn checknum(n: usize, primes : &Vec<usize>, max : usize) -> bool {

  for p in primes {
    for s in 1..max {
      if p + 2 * (s*s) == n {
        return true;
      }
    }
  }
  false
}

pub fn main() {

  let max = 10000;
  let primes = sieve(max);

  let mut i = 9;
  while i < max {
    if primes.contains(&i) {
      i += 2;
      continue;
      
    }

    if !checknum(i, &primes, max) {
      
      println!("found -> {}", i);
      break;
    }

    i += 2;
  }
}