use std::ops::Add;

pub fn add<T: Copy + Add<Output = T>>(lhs: T, rhs: T) -> T {
    lhs + rhs
}

pub fn sum<T: Copy + Add<Output = T>>(summands: &Vec<T>) -> T {
    let mut sum = summands[0];
    for summand in summands.iter().skip(1) {
        sum = add(sum, summand.to_owned());
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_plus_two_equals_four() {
        let lhs = 2;
        let rhs = 2;

        assert_eq!(add(lhs, rhs), 4);
    }

    #[test]
    fn test_add() {
        let first_num = 2;
        let num_to_add = 1;

        assert_eq!(add(first_num, num_to_add), 3);
    }

    #[test]
    fn test_sum() {
        let nums = vec![1, 2, 3, 4];

        assert_eq!(sum(&nums), 10);
    }
}
