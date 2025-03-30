fn main() {
    let mut sum: i64 = 2;
    let mut n_2: i64 = 1;
    let mut n_1: i64 = 2;
    while n_1 < 4000000 {
        let n = n_2 + n_1;
        n_2 = n_1;
        n_1 = n;
        if n % 2 == 0 {
            sum += n;
        }
    }
    print!("{}", sum);
}
