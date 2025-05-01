fn main() {
    let mut num_1: i64 = 999;
    let mut num_2: i64 = 999;
    let mut num_1_max: i64 = num_1;
    let mut num_2_max: i64 = num_2;
    let mut change_num_1 = true;
    let mut highest_palindrome = 0;
    loop {
        let i_mult: i64 = num_1 * num_2;
        let s_mult: String = i_mult.to_string();
        let mult_len: i64 = s_mult.chars().count() as i64;
        let mut is_palindrome = true;
        let loop_stop: i64 = mult_len / 2 + 1;
        for i in 0..loop_stop {
            let front_pos: usize = i as usize;
            let back_pos: usize = (mult_len - i - 1) as usize;
            if s_mult.chars().nth(front_pos) != s_mult.chars().nth(back_pos) {
                is_palindrome = false;
            }
        }
        if is_palindrome && i_mult > highest_palindrome {
            highest_palindrome = i_mult;
        }
        if change_num_1 {
            num_1 -= 1;
        } else {
            num_2 -= 1;
        }
        if num_1 == 100 && num_2 > 100 {
            num_1_max -= 1;
            num_1 = num_1_max;
            change_num_1 = false;
        } else if num_1 > 100 && num_2 == 100 {
            num_2_max -= 1;
            num_2 = num_2_max;
            change_num_1 = true;
        } else if num_1 == 100 && num_2 == 100 {
            break;
        }
    }
    print!("{}", highest_palindrome);
}


