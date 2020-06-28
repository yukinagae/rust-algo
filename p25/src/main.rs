fn main() {
    let m = 10;
    let mut k = vec![1, 3, 5];
    let result = lottery(m, &mut k);
    println!("{}", if result { "Yes" } else { "No" });
}

// O(n^3logn)
//
// Caution: Use i32 because u32 can cause `attempt to subtract with overflow` error,
// if `m - ka - kb - kc;` will be minus.
fn lottery(m: i32, _k: &mut Vec<i32>) -> bool {
    _k.sort();
    let k: Vec<i32> = _k.to_vec();
    for ka in &k {
        for kb in &k {
            for kc in &k {
                let kd = m - ka - kb - kc;
                if binary_search(&k, kd) {
                    return true;
                }
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
        let actual = super::lottery(10, &mut vec![1, 3, 5]);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = super::lottery(9, &mut vec![1, 3, 5]);
        let expected = false;
        assert_eq!(actual, expected);
    }
}
