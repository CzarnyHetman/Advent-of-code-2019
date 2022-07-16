fn main() {
    let mut count = 0;

    for i in 236491..713787 {
        if check_increasing(&i) && check_double_digit(&i) {
            count += 1;
        }
    }

    println!("count: {}", count);
}

fn check_increasing(num: &i32) -> bool {
    let mut prev = 0u8;
    let num_str = num.to_string();
    for b in num_str.bytes() {
        if b < prev {
            return false;
        }
        prev = b;
    }

    true
}

fn check_double_digit(num: &i32) -> bool {
    let mut prev = 0u8;
    let mut count_in_prev = 1;
    let num_str = num.to_string();
    for b in num_str.bytes() {
        if b == prev {
            count_in_prev += 1;
        } else if count_in_prev == 2 {
            return true;
        } else {
            count_in_prev = 1;
        }

        prev = b;
    }

    if count_in_prev == 2 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_double_digit_should_pass() {
        let num = 112345;

        assert!(check_double_digit(&num));
    }

    
    #[test]
    fn check_double_digit_should_fail_for_tripple() {
        let num = 111345;

        assert!(!check_double_digit(&num));
    }

    #[test]
    fn check_double_digit_should_pass_for_double_and_tripple() {
        let num = 111335;

        assert!(check_double_digit(&num));
    }

    #[test]
    fn check_double_digit_should_pass_for_111122() {
        let num = 111122;

        assert!(check_double_digit(&num));
    }
    
    #[test]
    fn check_double_digit_should_fail() {
        let num = 123456;

        assert!(!check_double_digit(&num));
    }

    #[test]
    fn check_increasing_should_pass_increasing() {
        let num = 123456;

        assert!(check_increasing(&num));
    }

    #[test]
    fn check_increasing_should_pass_repeating() {
        let num = 111111;

        assert!(check_increasing(&num));
    }

    #[test]
    fn check_increasing_shoul_fail_decreasing() {
        let num = 654321;

        assert!(!check_increasing(&num));
    }
}