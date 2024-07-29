pub fn main() {

  for a in 0..500 {
    for b in 0..500 {
      for c in 0..500 {
        if a*a + b*b == c*c && a + b + c == 1000 {
          println!("a{} b{} c{}", a, b, c);
          println!("{}", a*b*c);
          break;
        }
      }
    }
  }
}