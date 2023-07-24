fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    use std::cmp::max;
    let mut out = Vec::new();

    for (k, i) in asteroids.iter().zip(asteroids[1..].iter()) {
        // collusion
        if *i == (-1 * *k) {
            continue;
        } else {
            if *i > 0 && *k < 0 {
                out.push(max(*i, -1 * k));
            } else {
                out.push(*k)
            }
        }
    }
    return out;
}

pub fn run() {
    //let input = vec![5, 10, -5];
    let input = vec![8, -8];
    asteroid_collision(input);
}
#[test]
fn validate() {
    let input = vec![5, 10, -5];
    //let input = vec![8, -8];
    assert_eq!(vec![5, 10], asteroid_collision(input));
}

#[test]
fn validate2() {
    let input = vec![1, 5, -1];
    assert_eq!(vec![1, 2, 3], asteroid_collision(input))
}
