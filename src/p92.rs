fn get_square_sum(n : usize) -> usize {
  let mut res = 0;

  let mut num = n;
  while num > 0 {
    res = res + (num % 10)*(num % 10);
    num = num / 10;
  }

  res
}

pub fn main() {
  let mut c = 0;
  
  for mut n in 1..10000000 {
    while n != 89 && n != 1 {
      n = get_square_sum(n);
      if n == 89 {
        c += 1;
        break;
      }
    }
  }

  println!("{}", c+1);
}