use std::cmp;

fn is_palindrome(n: u64) -> bool {
  let binding = n.to_string();
  let s = binding.as_bytes();

  for i in 0..s.len()/2 {
    if s[i] != s[s.len()-i-1] {
      return false;
    }
  }
  
  true
}

pub fn main() {
  let mut max : u64 = 0;

  for i in 100..1000 {
    for j in 100..1000 {
      let n : u64 = i*j;

      if is_palindrome(n) {
        max = cmp::max(max, n);
      }
    }
  }

  println!("{}", max);
}