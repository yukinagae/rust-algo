pub fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let actual = fact(3);
        let expected = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = fact(4);
        let expected = 24;
        assert_eq!(actual, expected);
    }
}
