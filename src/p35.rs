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

fn get_circular_perms(digits : &Vec<usize>) -> Vec<usize> {
  let mut res = vec![0; digits.len()];
  let mut tmp = digits.clone();

  for i in 0..digits.len() {
    let last = tmp.pop().unwrap();
    tmp.insert(0, last);
    
    let num = tmp.iter().fold(0, |acc, &d| acc * 10 + d);
    res[i] = num;
  }

  res
}

fn is_circular(n: usize, primes : &Vec<usize>) -> bool {
  let digits: Vec<usize> = n.to_string().chars().map(|d| d.to_digit(10).unwrap() as usize).collect();

  for d in digits.iter() {
    if *d % 2 == 0 || *d == 5 {
      return false;
    }
  }
  for perm in get_circular_perms(&digits) {
    if !primes.contains(&perm) {
      return false;
    }
  }

  true
}

pub fn main() {
  let primes = sieve(1000000);
  let mut c = 0;
  for i in primes.iter() {
    if is_circular(*i, &primes) {
      c += 1;
    }
  }

  println!("{}", c+2);
}