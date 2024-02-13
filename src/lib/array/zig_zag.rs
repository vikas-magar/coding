// Time: O(n) | Space : O(n)
pub fn run(input: &Vec<Vec<i32>>) -> Vec<&i32> {
    let height = input.len() - 1;
    let width = input[0].len() - 1;
    let mut result: Vec<&i32> = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let mut going_down: bool = true;

    while !is_out_of_bound(&row, &col, &height, &width) {
        result.push(&input[row][col]);
        if going_down {
            if col == 0 || row == height {
                going_down = false;
                if row == height {
                    col += 1;
                } else {
                    row += 1
                }
            } else {
                row += 1;
                col -= 1;
            }
        } else {
            if row == 0 || col == width {
                going_down = true;
                if col == width {
                    row += 1;
                } else {
                    col += 1;
                }
            } else {
                row -= 1;
                col += 1;
            }
        }
    }
    result
}
fn is_out_of_bound(row: &usize, col: &usize, height: &usize, width: &usize) -> bool {
    *row > *height || *col > *width
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
        let output: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let opt: Vec<i32> = run(&input).iter().map(|x| **x).collect();
        assert_eq!(output, opt);
    }
}
