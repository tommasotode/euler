fn is_valid(n : usize) -> bool {
  if n < 100000000 || n > 999999999 {
    return false;
  }

  let mut arr = [false; 10];

  let mut num = n;
  while num > 0 {
    if num%10 == 0 {
      return false;
    }
    if arr[num % 10] {
      return false;
    } else {
      arr[num % 10] = true;
    }

    num = num / 10;
  }

  true
}

fn get_results(n : usize) -> [usize; 6] {
  let mut results = [0; 6];
  
  let mut s = String::from("");
  for mul in 1..7 {
    let prod = n * mul;
    s.push_str(&prod.to_string());
    if s.len() > 9 {
      break;
    }
    results[mul-1] = s.parse().unwrap();
  }

  results
}

pub fn main() {
  let mut max = 0;

  for n in 1..10000 {
    for res in get_results(n) {
      if is_valid(res) && res > max {
        max = res;
      }
    } 
  }

  println!("{}", max);
}