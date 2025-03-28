use std::collections::HashSet;

fn calc_mult_sum(mult: i32, numbers: &mut HashSet<i32>) {
    let max: i32 = 1000;
    let mut i: i32 = 1;
    loop {
        let num = mult * i;
        if num < max {
            numbers.insert(num);
        } else {
            break;
        }
        i += 1;
    }
}

fn main() {
    let mut numbers = HashSet::<i32>::new();
    calc_mult_sum(3, &mut numbers);
    calc_mult_sum(5, &mut numbers);
    println!("{}", numbers.iter().sum::<i32>());
}
