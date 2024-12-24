use crate::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut left, mut right) = (1, piles.iter().max().unwrap().clone());
        while left < right {
            let mid = left + (right - left) / 2;
            if how_many(&piles, mid, h) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}

fn how_many(piles: &[i32], mid: i32, h: i32) -> bool {
    let mut total_hours = 0;
    for &pile in piles {
        total_hours += (pile + mid - 1) / mid;
    }
    total_hours <= h
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_one() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn test_two() {
        let piles2 = vec![30, 11, 23, 4, 20];
        let h2 = 5;
        assert_eq!(Solution::min_eating_speed(piles2, h2), 30);
    }
}
