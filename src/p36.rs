fn is_palindrome(n : String) -> bool {
  let s = n.as_bytes();

  for i in 0..s.len()/2 {
    if s[i] != s[s.len()-i-1] {
      return false;
    }
  }
  
  true
}

pub fn main() {
  let mut sum = 0;
  for i in 0..1000000 {
    if is_palindrome(i.to_string()) && is_palindrome(format!("{:b}", i)) {
      sum += i;
    }
  }

  println!("{}", sum);

}