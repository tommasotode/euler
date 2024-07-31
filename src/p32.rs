use itertools::Itertools;
use std::collections::HashSet;

fn to_int(digits : &[&u8]) -> usize {
  let mut res : usize = 0; 

  for &d in digits {
    res = res * 10 + *d as usize;
  }

  res
}

pub fn main() {
  let a : [u8; 9] = [1,2,3,4,5,6,7,8,9];
  let mut products = HashSet::new();


  let perms = a.iter().permutations(9);
  for p in perms {
    for len_f1 in 1..=4 {
      for len_f2 in 1..=3 {
        let f1 : (usize, usize) = (0, len_f1);
        let f2 : (usize, usize) = (f1.1, f1.1 + 1 + len_f2);
        let prod: (usize, usize) = (f2.1, p.len());
      
        if to_int(&p[f1.0..f1.1]) * to_int(&p[f2.0..f2.1]) == to_int(&p[prod.0..prod.1]) {
          products.insert(to_int(&p[prod.0..prod.1]));
        }
      } 
    }
  }

  let mut sum = 0;
  for p in products {
    sum = sum + p;
  }

  println!("{}", sum);
}