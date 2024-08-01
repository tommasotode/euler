use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn roman_to_decimal(s: String) -> String {    
  let mut total = 0;
  let mut prev = 0;

  for digit in s.chars().rev() {
    let curr; 
    match digit {
      'I' => curr = 1,
      'V' => curr = 5,
      'X' => curr = 10,
      'L' => curr = 50,
      'C' => curr = 100,
      'D' => curr = 500,
      'M' => curr = 1000,
      _ => curr = 0 
    }
    if curr < prev {
      total -= curr;
    } else {
      total += curr;
    }
    prev = curr;
  }

  total.to_string()
}


fn decimal_to_roman(n: String) -> String {
  let mut result = String::new();
  let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
  let digits = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
  
  let mut num : usize = n.parse().unwrap();
  for (i, &value) in values.iter().enumerate() {
      while num >= value {
          num -= value;
          result.push_str(digits[i]);
      }
  }
  
  result
}

pub fn main() -> io::Result<()> {
    let path = Path::new("inputs/p89.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut sum : usize = 0;
    for line in reader.lines() {
        let line = line?;
        let diff = line.len() - decimal_to_roman(roman_to_decimal(line)).len();
        sum += diff;
    }
    println!("{}", sum);

    Ok(())
}