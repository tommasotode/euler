use std::cmp::max;
use std::fs;

fn search(x: usize, y: usize, p: &Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>) -> usize {
    if x >= p.len() {
        return 0;
    }

    if dp[x][y] != 0 {
        return dp[x][y];
    }

    let left = p[x][y] + search(x + 1, y, p, dp);
    let right = p[x][y] + search(x + 1, y + 1, p, dp);

    dp[x][y] = max(left, right);
    dp[x][y]
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
    let s = fs::read_to_string("inputs/i67.txt").expect("ok");

    let rows = 101;
    let mut dp: Vec<Vec<usize>> = Vec::with_capacity(rows);
    for r in 1..rows {
        let row = vec![0; r];
        dp.push(row);
    }

    println!("{}", search(0, 0, &subarrays(&s), &mut dp));
}
