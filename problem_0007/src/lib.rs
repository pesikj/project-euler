fn is_prime(num: i64) -> bool {
    let num_half: i64 = num/2 + 1;
    for i in 2..num_half {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn get_nth_prime(nth: i32) -> i64 {
    let mut prime_count: i32 = 0;
    let mut current: i64 = 2;
    loop {
        let curr_prime: bool = is_prime(current);
        if curr_prime {
            prime_count += 1;
        }
        if prime_count == nth {
            return current;
        }
        current += 1;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(100));
        assert!(is_prime(13));
    }

    #[test]
    fn test_nth_prime() {
        assert!(get_nth_prime(1) == 2);
        assert!(get_nth_prime(2) == 3);
        assert!(get_nth_prime(3) == 5);
        assert!(get_nth_prime(4) == 7);
        assert!(get_nth_prime(5) == 11);
        assert!(get_nth_prime(6) == 13);
    }
}

