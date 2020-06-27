fn main() {
    let m = 10;
    let k = vec![1, 3, 5];
    let result = lottery(m, k);
    println!("{}", if result { "Yes" } else { "No" });
}

fn lottery(m: u32, k: Vec<u32>) -> bool {
    for ka in &k {
        for kb in &k {
            for kc in &k {
                for kd in &k {
                    if (ka + kb + kc + kd) == m {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        let actual = super::lottery(10, vec![1, 3, 5]);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = super::lottery(9, vec![1, 3, 5]);
        let expected = false;
        assert_eq!(actual, expected);
    }
}
