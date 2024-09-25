use std::ops::Add;

pub fn add<T: Copy + Add<Output = T>>(lhs: T, rhs: T) -> T {
    lhs + rhs
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
}
