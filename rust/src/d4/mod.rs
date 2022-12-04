#[derive(Debug, PartialEq)]
struct Job {
    min_id: u32,
    max_id: u32
}

impl Job {
    pub fn from_string(id_range_string: &str) -> Job{
        let job_id_range: Vec<u32> = id_range_string
        .split("-")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<u32>>();

        Job {
            min_id: job_id_range[0],
            max_id: job_id_range[1]
        }
    }

    pub fn is_fully_contained_in(&self, other: &Job) -> bool {
        self.min_id >= other.min_id
        && self.max_id <= other.max_id
    }
}

#[derive(Debug, PartialEq)]
struct PairAssignment {
    job1: Job,
    job2: Job
}

impl PairAssignment {
    pub fn from_string(input: &str) -> PairAssignment{
        let job_id_ranges: Vec<&str> = input.split(",").collect::<Vec<&str>>();
        PairAssignment {
            job1: Job::from_string(job_id_ranges[0]),
            job2: Job::from_string(job_id_ranges[1])
        }
    }

    pub fn is_one_of_pair_fully_contained_in_the_other(&self) -> bool {
        self.job1.is_fully_contained_in(&self.job2)
        || self.job2.is_fully_contained_in(&self.job1)
    }

    pub fn overlaps(&self) -> bool {
        (self.job1.min_id >= self.job2.min_id && self.job1.min_id <= self.job2.max_id)
        ||
        (self.job1.max_id <= self.job2.max_id && self.job1.max_id >= self.job2.min_id)
        ||
        self.is_one_of_pair_fully_contained_in_the_other()
    }
}

fn get_pair_assignments_from_input(input: &str) -> Vec<PairAssignment> {
    input.split("\n").collect::<Vec<&str>>().iter().map(|x| PairAssignment::from_string(x)).collect::<Vec<PairAssignment>>()
}

pub fn get_total_number_of_fully_contained_assignment_pairs(input: &str) -> u32 {
    let pair_assignments = get_pair_assignments_from_input(input);
    pair_assignments.into_iter().filter(|pair| pair.is_one_of_pair_fully_contained_in_the_other()).collect::<Vec<PairAssignment>>().len() as u32
}

pub fn get_total_number_of_overlapping_assignment_pairs(input: &str) -> u32 {
    let pair_assignments = get_pair_assignments_from_input(input);
    pair_assignments.into_iter().filter(|pair| pair.overlaps()).collect::<Vec<PairAssignment>>().len() as u32
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("1-2", Job{min_id: 1, max_id: 2})]
    #[case("1324-2563", Job{min_id: 1324, max_id: 2563})]
    fn test_job_from_string(#[case] input: &str, #[case] expected: Job) {
        assert_eq!(Job::from_string(input), expected)
    }

    #[rstest]
    #[case(Job{min_id: 1, max_id: 2}, Job{min_id: 1, max_id: 2}, true)]
    #[case(Job{min_id: 2, max_id: 5}, Job{min_id: 1, max_id: 2}, false)]
    fn test_job_is_fully_contained_in(#[case] j1: Job, #[case] j2: Job, #[case] expected: bool) {
        assert_eq!(j1.is_fully_contained_in(&j2), expected)
    }

    #[test]
    fn test_pairassignment_from_string() {
        let input = "1-2,3-4";
        let expected = PairAssignment{
            job1: Job{min_id:1, max_id:2},
            job2: Job{min_id:3, max_id:4},
        };
        let actual = PairAssignment::from_string(input);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Job{min_id: 1, max_id: 2}, Job{min_id: 1, max_id: 5}, true)]
    #[case(Job{min_id: 1, max_id: 5}, Job{min_id: 1, max_id: 2}, true)]

    #[case(Job{min_id: 1, max_id: 5}, Job{min_id: 5, max_id: 6}, false)]
    #[case(Job{min_id: 5, max_id: 6}, Job{min_id: 1, max_id: 5}, false)]
    fn test_is_one_of_pair_fully_contained_in_the_other(#[case] j1: Job, #[case] j2: Job, #[case] expected: bool) {
        let pair_assignment = PairAssignment{job1: j1, job2: j2};
        assert_eq!(pair_assignment.is_one_of_pair_fully_contained_in_the_other(), expected)
    }

    #[rstest]
    #[case("2-4,6-8", false)]
    #[case("2-3,4-5", false)]
    #[case("5-7,7-9", true)]
    #[case("2-8,3-7", true)]
    #[case("3-7,2-8", true)]
    #[case("6-6,4-6", true)]
    #[case("2-6,4-8", true)]
    fn test_jobs_overlap(#[case] id_string: &str, #[case] expected: bool) {
        let pa = PairAssignment::from_string(id_string);
        assert_eq!(pa.overlaps(), expected, "expected {} for {}", expected, id_string);
    }

    #[test]
    fn test_get_pair_assignments_from_input() {
        let input = r#"2-4,6-8
2-3,4-5"#;
        let expected: Vec<PairAssignment> = vec![
            PairAssignment{
                job1: Job {min_id: 2, max_id: 4},
                job2: Job {min_id: 6, max_id: 8},
            },
            PairAssignment{
                job1: Job {min_id: 2, max_id: 3},
                job2: Job {min_id: 4, max_id: 5},
            }
        ];

        let actual = get_pair_assignments_from_input(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_total_number_of_fully_contained_assignment_pairs() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let expected = 2;
        let actual = get_total_number_of_fully_contained_assignment_pairs(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_total_number_of_overlapping_assignment_pairs() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let expected = 4;
        let actual = get_total_number_of_overlapping_assignment_pairs(input);
        assert_eq!(actual, expected);
    }
}

