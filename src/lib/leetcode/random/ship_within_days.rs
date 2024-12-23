use crate::Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = weights.iter().max().unwrap().clone();
        let mut right = weights.iter().sum();
        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::feasibility(&weights, mid, days) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }

    pub fn feasibility(weights: &Vec<i32>, capacity: i32, days: i32) -> bool {
        let mut total_weight = 0;
        let mut d_days = 1;
        for weight in weights {
            total_weight += weight;
            if total_weight > capacity {
                total_weight = *weight;
                d_days += 1;
            }
            if d_days > days {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let d = 5;

        assert_eq!(Solution::ship_within_days(a, d), 15);
    }
}
