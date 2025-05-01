fn is_evenly_divisible(num: i32, max_div: i32) -> bool {
    for i in 2..max_div {
        if num % i > 0 {
            return false;
        }
    }
    return true;
}

pub fn find_smallest_evenly_divisible(max_div: i32) -> i32 {
    let mut curr_num: i32 = max_div;
    loop {
        let res: bool = is_evenly_divisible(curr_num, max_div);
        if res == true {
            return curr_num;
        } else {
            curr_num += max_div;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_is_evenly_divisible() {
        assert!(is_evenly_divisible(2520, 10));
        assert!(!is_evenly_divisible(2519, 10));
    }

    #[test]
    fn test_find_smallest_evenlz_divisible() {
        assert!(find_smallest_evenly_divisible(10) == 2520);
    }
}

