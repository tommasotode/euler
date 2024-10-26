// fn search(i: usize) -> usize {
//     64
// }

fn subarrays(s: &str) -> Vec<Vec<usize>> {
    let v : Vec<&str> = s.split("\n").collect();

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

// subarray for each row
// in a bigger array (matrix)
// for each row (subarray)
// row++
// r+ o r=

pub fn main() {
    let a = subarrays(
"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
    );
}
