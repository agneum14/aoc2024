use std::fs::read_to_string;

use itertools::Itertools;

struct UnusualData {
    reports: Vec<Report>,
}

impl UnusualData {
    fn new(input: &str) -> Self {
        let reports = input.lines().map(|x| Report::new(x)).collect_vec();
        Self { reports }
    }

    fn count_safe_reports(&self) -> usize {
        self.reports.iter().filter(|x| x.safe()).count()
    }

    fn count_safe_reports_with_problem_dampener(&self) -> usize {
        self.reports
            .iter()
            .filter(|x| x.problem_dampener_safe())
            .count()
    }
}

struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn new(input: &str) -> Self {
        let levels = input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        Self { levels }
    }

    fn safe(&self) -> bool {
        Report::inner_safe(&self.levels)
    }

    fn problem_dampener_safe(&self) -> bool {
        let mut permutations = Vec::from([self.levels.clone()]);
        for i in 0..self.levels.len() {
            let mut permutation = self.levels.clone();
            permutation.remove(i);
            permutations.push(permutation);
        }

        permutations
            .iter()
            .find(|x| Report::inner_safe(x))
            .is_some()
    }

    fn inner_safe(levels: &Vec<i64>) -> bool {
        let increasing = levels.windows(2).find(|x| x[0] >= x[1]).is_none();
        let decreasing = levels.windows(2).find(|x| x[0] <= x[1]).is_none();
        let good_diff = levels
            .windows(2)
            .find(|x| {
                let diff = x[0].abs_diff(x[1]);
                diff < 1 || diff > 3
            })
            .is_none();
        (increasing ^ decreasing) && good_diff
    }
}

pub fn run() {
    let input = read_to_string("inputs/day02.txt").unwrap();
    let ud = UnusualData::new(&input);
    println!("Safe report count: {}", ud.count_safe_reports());
    println!(
        "Safe report count with problem dampener: {}",
        ud.count_safe_reports_with_problem_dampener()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_safe_reports() {
        let ud = UnusualData::new(&read_to_string("inputs/day02_small.txt").unwrap());
        assert_eq!(2, ud.count_safe_reports())
    }

    #[test]
    fn problem_dampener_safe_reports() {
        let ud = UnusualData::new(&read_to_string("inputs/day02_small.txt").unwrap());
        assert_eq!(4, ud.count_safe_reports_with_problem_dampener())
    }
}
