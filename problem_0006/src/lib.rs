pub fn calc_sum_square_diff(num: i32) -> i32 {
    let mut sum_of_squares: i32 = 0;
    let mut square_of_sum: i32 = 0;
    for i in 1..(num + 1) {
        sum_of_squares += i.pow(2);
        square_of_sum += i;
    }
    square_of_sum = square_of_sum.pow(2);
    return square_of_sum - sum_of_squares;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_sum_square_diff() {
        assert!(calc_sum_square_diff(10) == 2640);
    }
}
