
extern crate rand;

pub mod utilities {
    use super::rand::random;

    pub fn compute(num: i32) -> i32 {
        if num <= 0 {
            return 0;
        }
        return num + 2 + random::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::utilities::*;

    #[test]
    fn given_zero_expect_zero() {
        assert_eq!(0, compute(0))
    }

    #[test]
    fn given_negative_num_expect_zero() {
        assert_eq!(0, compute(-11))
    }

    #[test]
    fn given_two_expect_no_less_than_four() {
        assert!(compute(2) >= 4)
    }
}
