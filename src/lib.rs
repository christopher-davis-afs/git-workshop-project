use std::ops::Add;

pub fn add<T: Copy + Add<Output = T>>(lhs: T, rhs: T) -> T {
    lhs + rhs
}

pub fn sum<T: Copy + Add<Output = T>>(summands: &Vec<T>) -> T {
    let sum = summands
        .iter()
        .skip(1)
        .fold(summands[0], |lhs, rhs| add(lhs, *rhs));

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

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
