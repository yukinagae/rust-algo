use std::cmp::max;

fn main() {
    println!("Hello, world!");
}

fn get_max_triangle_len(a: Vec<u32>) -> u32 {
    let mut ans = 0;
    let n = a.len();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let len = a[i] + a[j] + a[k];
                let ma = max(a[i], max(a[j], a[k]));
                if len > 2 * ma {
                    ans = max(ans, len);
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        let actual = super::get_max_triangle_len(vec![2, 3, 4, 5, 10]);
        let expected = 12;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = super::get_max_triangle_len(vec![4, 5, 10, 20]);
        let expected = 0;
        assert_eq!(actual, expected);
    }
}
