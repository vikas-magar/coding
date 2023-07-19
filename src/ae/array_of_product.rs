use std::i32;

fn array_of_product(nums: Vec<i32>) -> Vec<i32> {
    let mut left = vec![1; nums.len()];
    let mut right = vec![1; nums.len()];
    let mut output = Vec::new();
    let mut product = 1;
    for (key, _) in nums.iter().enumerate() {
        if key == 0 {
            left[key] = 1;
        } else {
            product *= nums[key - 1];
            left[key] = product;
        }
    }

    product = 1;
    for (key, _) in nums.iter().rev().enumerate() {
        if key == 0 {
            right[key] = 1;
        } else {
            product *= nums[nums.len() - key];
            right[key] = product;
        }
    }
    for (x, y) in left.iter().zip(right.iter().rev()) {
        output.push(x * y);
    }
    return output;
}

pub fn run() {
    let input = vec![5, 1, 4, 2];
    array_of_product(input);
}
