use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let path = Path::new("inputs/p79.txt");
    let file = File::open(&path)?;

    let reader = BufReader::new(file);


    let mut s :HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        s.insert(line);
    }

    for i in s {
      println!("{}", i);
    }

    Ok(())
}
