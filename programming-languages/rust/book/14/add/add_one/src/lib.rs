use rand;

pub fn add_one(num: usize) -> usize {
    num + 1
}

pub fn add_rand(num: usize) -> usize {
    num + rand::random::<u8>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_test() {
        let result: usize = add_one(2);
        assert_eq!(result, 3);
    }

    #[test]
    fn add_rand_test() {
        let num = 1;
        let result = add_rand(num);
        assert!((u8::MIN as usize) < result);
        assert!(result < (u8::MAX as usize + num + 1));
    }
}
