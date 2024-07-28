fn is_prime(n: u64) -> bool {
  if n <= 1 {
      return false;
  }
  if n <= 3 {
      return true;
  }
  if n % 2 == 0 || n % 3 == 0 {
      return false;
  }
  let mut i = 5;
  while i * i <= n {
      if n % i == 0 || n % (i + 2) == 0 {
          return false;
      }
      i += 6;
  }
  true
}

pub fn main() {
  let num = 600851475143;
  let mut i : u64 = num/29 + 1;

  while i > 2 {
    if num % i == 0 && is_prime(i) {
        println!("found -> {}", i);
        break;
    }
    if (num - i) % 100000000 == 0 {
        println!("{}", i)
    }
    i -= 2;
  }
}