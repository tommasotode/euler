use std::cmp::max;
use std::fs;

fn search(x: usize, y: usize, p: &Vec<Vec<usize>>) -> usize {
    if x >= p.len() {
        return 0;
    }

    let left = p[x][y] + search(x + 1, y, p);
    let right = p[x][y] + search(x + 1, y + 1, p);

    max(left, right)
}

fn subarrays(s: &str) -> Vec<Vec<usize>> {
    let v: Vec<&str> = s.split("\n").collect();

    let mut res: Vec<Vec<usize>> = Vec::with_capacity(v.len());
    for _ in 0..v.len() {
        res.push(Vec::new());
    }
    for (i, line) in v.iter().enumerate() {
        for n in line.split(" ") {
            let num = n.parse::<usize>().unwrap();
            res[i].push(num);
        }
    }

    res
}

pub fn main() {
    let s = fs::read_to_string("inputs/i18.txt")
        .expect("ok");

    println!("{}", search(0, 0, &subarrays(&s)));
}
