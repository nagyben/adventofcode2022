use std::fmt;

#[derive(PartialEq, Clone)]
struct Matrix2D<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix2D<T>
where
    T: Copy,
{
    fn new(data: Vec<Vec<T>>) -> Matrix2D<T> {
        Matrix2D { data: data }
    }

    fn get_row(&self, i: usize) -> &Vec<T> {
        &self.data[i]
    }

    fn get_col(&self, i: usize) -> Vec<T> {
        let mut v = vec![];
        for j in 0..self.data[0].len() {
            v.push(self.data[j][i])
        }
        v
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }
}

impl<T> fmt::Debug for Matrix2D<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "");
        for row in &self.data {
            write!(f, "[");
            for val in row {
                write!(f, "{:?}", val);
                write!(f, ",");
            }
            writeln!(f, "]");
        }
        Ok(())
    }
}

fn init_matrix(input: &str) -> Matrix2D<usize> {
    let mut v: Vec<Vec<usize>> = vec![];
    for line in input.split("\n").collect::<Vec<&str>>() {
        v.push(
            line.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }
    Matrix2D::new(v)
}

fn get_visibility_matrix(input: &str) -> Matrix2D<usize> {
    let m = init_matrix(input);
    let mut v = m.clone();
    for irow in 0..m.rows() {
        for jcol in 0..m.cols() {
            // edges are always visible
            if irow == 0
            || irow == m.rows() - 1
            || jcol == 0
            || jcol == m.cols() - 1
            {
                v.data[irow][jcol] = 1;
                continue
            }

            // the hard bit
            if is_visible_in_row(m.get_row(irow), jcol)
            || is_visible_in_row(&m.get_col(jcol), irow) {
                v.data[irow][jcol] = 1;
            }
        }
    }
    v
}

fn is_visible_in_row(row: &Vec<usize>, index: usize) -> bool {
    println!("{:?}", row);
    if index == 0
    || index == row.len() - 1 {
        return true;
    } else {
        // from left
        let mut max = 0;
        for i in 0..=index {
            if row[i] > max {
                max = row[i];
            }
            println!("LTR [{:?}] cur: {:?}\tmax: {:?}", i, row[i], max);
            if i == index
            && max <= row[i] {
                return true;
            }
        }
        // from right
        let mut max = 0;
        for j in (index..=row.len()-1).rev() {
            if row[j] > max {
                max = row[j];
            }
            println!("RTL [{:?}] cur: {:?}\tmax: {:?}", j, row[j], max);
            if j == index
            && max <= row[j] {
                return true;
            }
        }
        return false
    }
    panic!()
}

fn get_number_of_visible_trees(input: &str) -> usize {
    3
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_get_number_of_visible_trees() {
        let input = r#"30373
25512
65332
33549
35390"#;
        let expected = 21;
        let actual = get_number_of_visible_trees(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_init_ndarray() {
        let input = r#"30373
25512
65332
33549
35390"#;
        let expected: Matrix2D<usize> = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        let actual = init_matrix(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_Matrix2D_get_row() {
        let m: Matrix2D<usize> = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        let expected: Vec<usize> = vec![3, 0, 3, 7, 3];
        let actual = m.get_row(0).to_owned();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_Matrix2D_get_col() {
        let m: Matrix2D<usize> = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        let expected: Vec<usize> = vec![3, 2, 6, 3, 3];
        let actual = m.get_col(0).to_owned();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_visibility_matrix() {
        let input = r#"30373
25512
65332
33549
35390"#;
        let expected: Matrix2D<usize> = Matrix2D::new(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1],
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
        ]);
        let actual = get_visibility_matrix(input);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, true)]
    #[case(1, true)]
    #[case(2, false)]
    #[case(3, true)]
    #[case(4, true)]
    #[case(5, false)]
    #[case(6, true)]
    #[case(7, true)]
    fn test_is_visible_in_row(#[case] index: usize, #[case] expected: bool) {
        let row: Vec<usize> = vec![2, 3, 1, 4, 7, 4, 6, 2];
        assert_eq!(is_visible_in_row(&row, index), expected); // 2
    }
}
