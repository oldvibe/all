pub fn next_prime(nbr: u64) -> u64 {
    let mut i = nbr;
    if i < 2 {
        return 2;
    }
    loop {
        if is_prime(i) {
            return i;
        }
        i += 1;
    }
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n / 2.0 as u64) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
