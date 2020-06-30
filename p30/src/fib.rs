pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub fn fib_memo(n: i32, memo: &mut Vec<i32>) -> i32 {
    if n <= 1 {
        return n;
    }
    if memo[n as usize] != 0 {
        return memo[n as usize];
    }
    memo[n as usize] = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let actual = fib(3);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = fib(6);
        let expected = 8;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example3() {
        let actual = fib_memo(3, &mut vec![0; 3 + 1]);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example4() {
        let actual = fib_memo(6, &mut vec![0; 6 + 1]);
        let expected = 8;
        assert_eq!(actual, expected);
    }
}
