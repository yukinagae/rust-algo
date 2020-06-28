fn main() {
    let m = 10;
    let k = vec![1, 3, 5];
    let result = lottery(m, k);
    println!("{}", if result { "Yes" } else { "No" });
}

// O(n^2logn)
fn lottery(m: i32, k: Vec<i32>) -> bool {
    let n = k.len();
    let mut kk: Vec<i32> = vec![0; n * n];
    for c in 0..n {
        for d in 0..n {
            kk[c * n + d] = k[c] + k[d];
        }
    }

    kk.sort();

    for ka in &k {
        for kb in &k {
            if binary_search(&kk, m - ka - kb) {
                return true;
            }
        }
    }
    false
}

// `k` should be sorted for binary search
fn binary_search(k: &Vec<i32>, x: i32) -> bool {
    let mut left = 0;
    let mut right = k.len();
    while (right - left) >= 1 {
        let center = (left + right) / 2;
        if k[center] == x {
            return true;
        } else if k[center] < x {
            left = center + 1;
        } else {
            right = center;
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
