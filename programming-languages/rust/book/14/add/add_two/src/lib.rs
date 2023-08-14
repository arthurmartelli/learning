pub fn add_two(num: usize) -> usize {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        let result: usize = add_two(2);
        assert_eq!(result, 4);
    }
}
