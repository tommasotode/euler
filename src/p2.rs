fn fibottomup() -> u64 {
  let mut arr : [u64; 100] = [0; 100];
  arr[0] = 1;
  arr[1] = 1;

  let mut sum : u64 = 0;

  for i in 2..arr.len() {
    arr[i] = arr[i-1] + arr[i-2];
    if arr[i] > 4000000 {
      break;
    }
    if arr[i] % 2 == 0 {
      sum += arr[i];
    }
  }
  sum
}

pub fn main() {
  println!("{}", fibottomup());
}