use common::problem::day::{AoCProblem, Solution};

pub struct Day06 {
    // (time, distance)
    paper: Vec<(usize, usize)>,
}

impl AoCProblem for Day06 {
    fn prepare(input: String) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let time = lines[0];
        let (_, raw_times) = time.split_once(':').unwrap();

        let distance = lines[1];
        let (_, raw_distances) = distance.split_once(':').unwrap();
        Self {
            paper: raw_times
                .split(' ')
                .filter(|d| !d.is_empty())
                .map(|d| d.parse::<usize>().unwrap())
                .zip(
                    raw_distances
                        .split(' ')
                        .filter(|d| !d.is_empty())
                        .map(|d| d.parse::<usize>().unwrap()),
                )
                .collect::<Vec<_>>(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut num_ways = 1;
        for (time, distance) in &self.paper {
            let mut num_wins = 0;
            for hold_button_time in 0..=*time {
                let distance_traveled = (*time - hold_button_time) * hold_button_time;
                if distance_traveled <= *distance {
                    continue;
                }

                num_wins += 1;
            }

            num_ways *= num_wins;
        }

        num_ways.into()
    }

    fn part2(&mut self) -> Solution {
        let mut raw_time = String::new();
        let mut raw_dist = String::new();
        for (t, d) in &self.paper {
            raw_time.push_str(&t.to_string());
            raw_dist.push_str(&d.to_string());
        }

        let target_time: usize = raw_time.parse::<usize>().unwrap();
        let target_distance = raw_dist.parse::<usize>().unwrap();
        // Let A = the target time,
        //     D = the target distance,
        // and H = the time to hold the button (unknown).
        //
        // Then, we want to solve the following inequality for H
        //
        //          (A - H) * H <= D
        //          -> AH - HH <= D
        //          -> AH - HH - D <= 0
        //          -> 0 <= HH - AH + D
        //          -> HH - AH + D >= 0
        //
        // We can use the quadratic formula to solve this inequality for H
        //
        //          H = (-(-A) ± sqrt((-A)(-A) - 4(1)(D))) / 2(1)
        //          -> H = (A ± sqrt(AA - 4D)) / 2
        //          -> H = (A + sqrt(AA - 4D)) / 2      -or     H = (A - sqrt(AA - 4D)) / 2

        let ans1 = (target_time as f64
            + ((target_time * target_time - 4 * target_distance) as f64).sqrt())
            / 2.0;
        let ans2 = (target_time as f64
            - ((target_time * target_time - 4 * target_distance) as f64).sqrt())
            / 2.0;

        let max_ans = f64::max(ans1, ans2).floor() as usize;
        let min_ans = f64::min(ans1, ans2).ceil() as usize;

        // + 1 because we want to include the lower time as a valid time to use
        (max_ans - min_ans + 1).into()
    }

    fn day() -> u32 {
        6
    }

    fn year() -> u32 {
        2023
    }
}
