use std::collections::HashSet;

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

fn sort_digits(n : usize) -> usize {
  let mut chars: Vec<char> = n.to_string().chars().collect();
  chars.sort();
  let sorted: String = chars.into_iter().collect();
  
  sorted.parse::<usize>().unwrap()
}

pub fn main() {
  let mut primes = sieve(9999);
  for i in primes.clone() {
    if i < 1000 {
      primes.remove(0);
      continue;
    }
    break;
  }

  let mut sol : HashSet<usize> = HashSet::new();
  
  for i in 0..primes.len()-2 {
    let mut diffs = vec![0; primes.len()];
    for j in i+1..primes.len() {
      diffs[j] = primes[j] - primes[i];
    }
    
    for d in i+1..diffs.len() {
      if diffs.contains( &(diffs[d] * 2) ) && 
      sort_digits(primes[i]) == sort_digits(primes[i] + diffs[d]) && 
      sort_digits(primes[i] + diffs[d]) == sort_digits(primes[i] + diffs[d] * 2) {
        sol.insert(primes[i]);
        sol.insert(primes[i] + diffs[d]);
        sol.insert(primes[i] + diffs[d] * 2);
      }
    }
  }

  println!("{:?}", sol);
}