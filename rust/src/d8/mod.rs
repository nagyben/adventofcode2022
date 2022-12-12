use std::fmt;

#[derive(PartialEq, Clone)]
struct Matrix2D {
    data: Vec<Vec<usize>>,
}

impl Matrix2D {
    fn new(data: Vec<Vec<usize>>) -> Matrix2D {
        Matrix2D { data: data }
    }

    fn from_size(size: usize) -> Matrix2D {
        let mut data: Vec<Vec<usize>> = vec![];
        for _ in 0..size {
            let mut ddata: Vec<usize> = vec![];
            for _ in 0..size {
                ddata.push(0);
            }
            data.push(ddata);
        }
        Matrix2D { data: data }
    }

    fn get_row(&self, i: usize) -> &Vec<usize> {
        &self.data[i]
    }

    fn get_col(&self, i: usize) -> Vec<usize> {
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

    fn get(&self, row: usize, col: usize) -> usize {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, val: usize) {
        self.data[row][col] = val;
    }
}

impl fmt::Debug for Matrix2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "").unwrap();
        for row in &self.data {
            write!(f, "[").unwrap();
            for val in row {
                write!(f, "{:?}", val).unwrap();
                write!(f, ",").unwrap();
            }
            writeln!(f, "]").unwrap();
        }
        Ok(())
    }
}

fn init_matrix(input: &str) -> Matrix2D {
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

fn get_visibility_matrix(input: &str) -> Matrix2D {
    let m = init_matrix(input);
    let mut v = m.clone();
    for row in 0..m.rows() {
        for col in 0..m.cols() {
            // edges are always visible
            if row == 0 || row == m.rows() - 1 || col == 0 || col == m.cols() - 1 {
                v.data[row][col] = 1;
                continue;
            }

            // the hard bit
            if is_visible_in_row(m.get_row(row), col) || is_visible_in_row(&m.get_col(col), row) {
                v.data[row][col] = 1;
            } else {
                v.data[row][col] = 0;
            }
        }
    }
    v
}

fn get_viewing_distance_matrix(input: &Matrix2D, direction: ViewingDistanceDirection) -> Matrix2D {
    let mut m: Matrix2D = Matrix2D::from_size(input.rows());

    if direction == ViewingDistanceDirection::UP || direction == ViewingDistanceDirection::DOWN {
        for i in 0..input.cols() {
            let col = input.get_col(i);
            let jindexer = match direction {
                ViewingDistanceDirection::UP => (1..col.len()).collect::<Vec<usize>>(),
                ViewingDistanceDirection::DOWN => (0..col.len()).collect::<Vec<usize>>(),
                _ => panic!(),
            };
            for j in jindexer {
                let mut viewing_distance = 0;
                let zindexer = match direction {
                    ViewingDistanceDirection::UP => (0..j).rev().collect::<Vec<usize>>(),
                    ViewingDistanceDirection::DOWN => (j + 1..col.len()).collect::<Vec<usize>>(),
                    _ => panic!(),
                };
                for z in zindexer {
                    viewing_distance += 1;
                    if col[z] >= col[j] {
                        break;
                    }
                }
                m.set(j, i, viewing_distance);
            }
        }
    } else {
        for i in 0..input.rows() {
            let row = input.get_row(i);
            let jindexer = match direction {
                ViewingDistanceDirection::LEFT => (0..row.len()).collect::<Vec<usize>>(),
                ViewingDistanceDirection::RIGHT => (0..row.len()).collect::<Vec<usize>>(),
                _ => panic!(),
            };
            for j in jindexer {
                let mut viewing_distance = 0;
                let zindexer = match direction {
                    ViewingDistanceDirection::LEFT => (0..j).rev().collect::<Vec<usize>>(),
                    ViewingDistanceDirection::RIGHT => (j + 1..row.len()).collect::<Vec<usize>>(),
                    _ => panic!(),
                };
                for z in zindexer {
                    viewing_distance += 1;
                    if row[z] >= row[j] {
                        break;
                    }
                }
                m.set(i, j, viewing_distance);
            }
        }
    }
    m
}

fn get_scenic_score(input: &Matrix2D) -> Matrix2D {
    let md = get_viewing_distance_matrix(input, ViewingDistanceDirection::DOWN);
    let mu = get_viewing_distance_matrix(input, ViewingDistanceDirection::UP);
    let ml = get_viewing_distance_matrix(input, ViewingDistanceDirection::LEFT);
    let mr = get_viewing_distance_matrix(input, ViewingDistanceDirection::RIGHT);
    let mut m = Matrix2D::from_size(md.rows());

    for i in 0..md.rows() {
        for j in 0..md.cols() {
            let scenic_score = md.get(i, j) * mu.get(i, j) * ml.get(i, j) * mr.get(i, j);

            m.set(i, j, scenic_score)
        }
    }
    m
}

fn get_max_scenic_score(input: &Matrix2D) -> usize {
    let scenic_scores = get_scenic_score(input);
    scenic_scores.data.into_iter().flatten().max().unwrap()
}

pub fn get_scenic_score_from_string(input: &str) -> usize {
    let m = init_matrix(input);
    get_max_scenic_score(&m)
}

fn is_visible_in_row(row: &Vec<usize>, index: usize) -> bool {
    if index == 0 || index == row.len() - 1 {
        return true;
    } else {
        // from left
        let mut max = 0;
        let mut max_index = 0;
        for i in 0..=index {
            if row[i] > max {
                max = row[i];
                max_index = i;
            }
            if i == index && (max < row[i] || (max == row[i] && max_index == i)) {
                return true;
            }
        }
        // from right
        let mut max = 0;
        for j in (index..=row.len() - 1).rev() {
            if row[j] > max {
                max = row[j];
                max_index = j;
            }
            if j == index && (max < row[j] || (max == row[j] && max_index == j)) {
                return true;
            }
        }
        return false;
    }
}

pub fn get_number_of_visible_trees(input: &str) -> usize {
    let v = get_visibility_matrix(input);
    v.data
        .into_iter()
        .flatten()
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}

#[derive(PartialEq)]
enum ViewingDistanceDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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
        let expected: Matrix2D = Matrix2D::new(vec![
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
    fn test_matrix2d_get_row() {
        let m: Matrix2D = Matrix2D::new(vec![
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
    fn test_matrix2d_get_col() {
        let m: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        let expected: Vec<usize> = vec![3, 2, 6, 3, 3];
        let actual = m.get_col(0).to_owned();
        assert_eq!(actual, expected);

        let expected: Vec<usize> = vec![0, 5, 5, 3, 5];
        let actual = m.get_col(1).to_owned();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_visibility_matrix() {
        let input = r#"30373
25512
65332
33549
35390"#;
        let expected: Matrix2D = Matrix2D::new(vec![
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
    #[case(
        vec![2, 3, 1, 4, 7, 4, 6, 2],
        vec![true, true, false, true, true, false, true, true]
    )]
    #[case(
        vec![3, 3, 5, 4, 9],
        vec![true, false, true, false, true]
    )]
    fn test_is_visible_in_row(#[case] row: Vec<usize>, #[case] viz: Vec<bool>) {
        for i in 0..row.len() {
            let actual = is_visible_in_row(&row, i);
            assert_eq!(
                actual, viz[i],
                "expected index {} to be {}, got {}",
                i, viz[i], actual
            );
        }
    }

    #[test]
    fn test_get_viewing_distance_matrix_up() {
        let input: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);

        let expected: Matrix2D = Matrix2D::new(vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![2, 1, 1, 2, 1],
            vec![1, 1, 2, 3, 3],
            vec![1, 2, 1, 4, 1],
        ]);

        let actual = get_viewing_distance_matrix(&input, ViewingDistanceDirection::UP);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_viewing_distance_matrix_dn() {
        let input: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);

        let expected: Matrix2D = Matrix2D::new(vec![
            vec![2, 1, 1, 4, 3],
            vec![1, 1, 2, 1, 1],
            vec![2, 2, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
        ]);

        let actual = get_viewing_distance_matrix(&input, ViewingDistanceDirection::DOWN);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_viewing_distance_matrix_left() {
        let input: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);

        let expected: Matrix2D = Matrix2D::new(vec![
            vec![0, 1, 2, 3, 1],
            vec![0, 1, 1, 1, 2],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 2, 1, 4],
            vec![0, 1, 1, 3, 1],
        ]);

        let actual = get_viewing_distance_matrix(&input, ViewingDistanceDirection::LEFT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_viewing_distance_matrix_right() {
        let input: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);

        let expected: Matrix2D = Matrix2D::new(vec![
            vec![2, 1, 1, 1, 0],
            vec![1, 1, 2, 1, 0],
            vec![4, 3, 1, 1, 0],
            vec![1, 1, 2, 1, 0],
            vec![1, 2, 1, 1, 0],
        ]);

        let actual = get_viewing_distance_matrix(&input, ViewingDistanceDirection::RIGHT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_scenic_score() {
        let input: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);

        let expected: Matrix2D = Matrix2D::new(vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 4, 1, 0],
            vec![0, 6, 1, 2, 0],
            vec![0, 1, 8, 3, 0],
            vec![0, 0, 0, 0, 0],
        ]);

        let actual = get_scenic_score(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_max_scenic_score() {
        let input: Matrix2D = Matrix2D::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);

        let _: Matrix2D = Matrix2D::new(vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 4, 1, 0],
            vec![0, 6, 1, 2, 0],
            vec![0, 1, 8, 3, 0],
            vec![0, 0, 0, 0, 0],
        ]);

        let expected = 8;

        let actual = get_max_scenic_score(&input);

        assert_eq!(actual, expected);
    }
}
