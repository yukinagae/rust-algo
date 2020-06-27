use std::cmp::{max, min};

fn main() {
    let result = ant(10, vec![2, 6, 7]);
    println!("min = {}", result.0);
    println!("max = {}", result.1);
}

fn ant(l: u32, x: Vec<u32>) -> (u32, u32) {
    let mut min_t = 0;
    for xi in &x {
        min_t = max(min_t, min(*xi, l - xi));
    }

    let mut max_t = 0;
    for xi in &x {
        max_t = max(max_t, max(*xi, l - xi));
    }
    (min_t, max_t)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        let actual = super::ant(10, vec![2, 6, 7]);
        let expected = (4, 8);
        assert_eq!(actual, expected);
    }
}
