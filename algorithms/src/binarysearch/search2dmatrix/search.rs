pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    
    let n_cols = matrix[0].len();
    
    for row in matrix {
        let mut left = 0;
        let mut right = n_cols - 1;
        let mut first_true_index: isize = -1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if row[mid] >= target {
                first_true_index = mid as isize;
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        if first_true_index != -1 && row[first_true_index as usize] == target {
            return true;
        }
    }
    
    false
}

#[cfg(test)]
mod search_2d_matrix_tests {
    use parameterized::parameterized;
    use parameterized::ide;
    use super::search_matrix_ii;

    ide!();

    #[parameterized(
        matrix = {
            vec![
                vec![-7, -1, 5, 11],
                vec![-6, 0, 6, 14],
                vec![-4, 2, 9, 20],
            ],
            vec![
                vec![-2, 0, 3],
                vec![1, 4, 7],
            ],
            vec![
                vec![-10, -3],
                vec![-4, 2],
                vec![0, 5],
            ],
            vec![
                vec![-13],
            ],
        },
        target = {6, -5, 9, -13},
        expected = {true, false, false, true},
    )]
    fn test_search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32, expected: bool) {
        let actual = search_matrix_ii(matrix, target);
        assert_eq!(actual, expected);
    }
}