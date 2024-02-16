pub fn run(input: &str) -> i32 {
    let mut stack: Vec<String> = Vec::new();
    let l = "(";
    let r = ")";
    for c in input.chars() {
        println!("{}", c)
    }
    2
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let s = "(())";
        println!("{:?}", run(s));

        assert_eq!(1, 2);
    }
}
