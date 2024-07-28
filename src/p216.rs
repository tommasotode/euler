fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn main() {
    let number = 500000;
    let mut c = 0;

    for i in 2..=number {
        if is_prime(2 * u64::pow(i, 2) - 1) {
            c += 1;
        }
    }

    println!("{}", c);
}
