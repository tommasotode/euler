fn get_digits_sum(n: u128) -> u128 {
    let mut res = 0;

    let mut num = n;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }

    res
}

// for each number with N digits -> 1 < digits_sum < N*9

// i can check the possible digits sums up to 9*X
// and raise each of them to an exponent up to Y
// if the digits sum of the power obtained is equal to the base
// i found a solution

pub fn main() {
    let mut solutions: Vec<u128> = Vec::new();

    let max_digits = 10;
    let max_exp = 10;

    for digitsum in 2..max_digits * 9 {
        for exp in 2..max_exp {
            let p = u128::pow(digitsum, exp);
            if get_digits_sum(p) == digitsum {
                solutions.push(p);
            }
        }
        if solutions.len() > 30 {
            break;
        }
    }

    solutions.sort();
    println!("{}", solutions[29]);
}
