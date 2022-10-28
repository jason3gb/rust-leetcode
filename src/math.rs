pub fn reverse_integer(x: i32) -> i32 {
    let is_negative = x < 0;

    let mut x_abs = x;
    if is_negative {
         x_abs = -x;
    }

    let x_abs_rev_str: String = x_abs.to_string().chars().rev().collect();
    let x_rev_abs = x_abs_rev_str.parse::<i32>().unwrap_or_else(|_| {
        return 0;
    });

    if is_negative {
        -x_rev_abs
    } else {
        x_rev_abs
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_integer() {
        assert_eq!(reverse_integer(0), 0);
        assert_eq!(reverse_integer(123), 321);
        assert_eq!(reverse_integer(-123), -321);
        assert_eq!(reverse_integer(1534236469), 0);
    }
}