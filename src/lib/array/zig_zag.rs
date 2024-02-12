pub fn run(input: &Vec<Vec<i32>>) {
    for ele in input.iter() {
        println!("{:?}", ele);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let input = vec![
            vec![1, 3, 4, 10],
            vec![2, 5, 9, 11],
            vec![6, 8, 12, 15],
            vec![7, 13, 14, 16],
        ];
        let output = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        run(&input);
    }
}
