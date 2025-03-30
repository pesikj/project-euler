fn main() {
    let mut num_1: i64 = 999;
    let mut num_2: i64 = 999;
    let mut change_num_1 = true;
    loop {
        let mult: i64 = num_1 * num_2;
        let mult_str: String = mult.to_string();
        let mult_str_len: i32 = mult_str.chars().count();
        let loop_stop: i32 = mult_str_len / 2;
        let mut palindrome: bool = true;
        for n in 1..loop_stop {
            let l: char = mult_str.chars().nth(n-1).unwrap();
            let r: char = mult_str.chars().nth(mult_str_len - n).unwrap();
            if l != r {
                palindrome = false;
                break;
            }
        }
        if palindrome == true {
            print!("{}", mult);
            break;
        }
        if change_num_1 {
            num_1 -= 1;
        } else {
            num_2 -= 1;
        }
        change_num_1 = !change_num_1;
    }
}
