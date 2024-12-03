use common::problem::day::{AoCProblem, Solution};

pub struct Day02 {
    reports: Vec<Vec<isize>>,
}

impl AoCProblem for Day02 {
    fn prepare(input: String) -> Self {
        Self {
            reports: input
                .lines()
                .map(|l| l.split(' ').map(|lvl| lvl.parse().unwrap()).collect())
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.reports
            .iter()
            .filter(|report| {
                report
                    .windows(2)
                    .all(|x| x[0] < x[1] && (1..=3).contains(&(x[1] - x[0])))
                    || report
                        .windows(2)
                        .all(|x| x[0] > x[1] && (1..=3).contains(&(x[0] - x[1])))
            })
            .count()
            .into()
    }

    fn part2(&mut self) -> Solution {
        let mut good_reports = 0;
        for report in self.reports.iter() {
            if is_increasing_report_ok(report) {
                good_reports += 1;
            } else {
                let local_rev_report = report.iter().cloned().rev().collect::<Vec<_>>();
                if is_increasing_report_ok(&local_rev_report) {
                    good_reports += 1;
                }
            }
        }

        good_reports.into()
    }

    fn day() -> u32 {
        2
    }

    fn year() -> u32 {
        2024
    }
}

/// Checks if a report is increasing, or can be fixed at most once to make it increasing.
///
/// # Parameters
/// - `report`: The report.
///
/// # Returns
/// Whether the report is increasing or can be fixed one time to make it increasing.
fn is_increasing_report_ok(report: &[isize]) -> bool {
    let validate_report = |r: &[isize]| -> bool {
        r.windows(2)
            .all(|w| w[0] < w[1] && (1..=3).contains(&(w[1] - w[0])))
    };

    if validate_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let temp_report = report
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, elem)| elem)
            .cloned()
            .collect::<Vec<_>>();

        if validate_report(&temp_report) {
            return true;
        }
    }

    false
}
