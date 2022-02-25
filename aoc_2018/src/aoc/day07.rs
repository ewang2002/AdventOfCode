use std::collections::HashSet;

// https://adventofcode.com/2018/day/7
#[allow(dead_code)]
pub fn execute(input: &[String]) -> (String, usize) {
    let steps: Vec<_> = input
        .iter()
        .map(|x| {
            // Step C must be finished before step A can begin.
            // 0    1 2    3  4        5      6    7 8   9
            let res = x.split(' ').collect::<Vec<_>>();
            Step {
                requirement: res[1].to_string(),
                for_part: res[7].to_string(),
            }
        })
        .collect();

    // Potential candidates that we should look into.
    let mut base_letters: HashSet<&String> = HashSet::new();

    // Step 0: We're looking for a (req, for) pair such that we don't see req anywhere in the "for"
    // member of any Step struct in the vector. That is, we're essentially doing:
    //      all_for = {map instructions to its for_part member.}
    //      all_req = {map instructions to its requirements member.}
    //  =>  result = all_req \ all_for
    // Then, we can add both the requirement and for_part members to the possible available set.
    //
    // The important thing to keep in mind is that, unlike the example, there are *multiple*
    // pairs that can be the "first" character in the string.
    //
    // Don't remove any instructions yet as we haven't actually updated the final string with these
    // values; we're only finding potential candidates to start with.
    for instruction in steps.iter() {
        if steps.iter().any(|y| y.for_part == instruction.requirement) {
            continue;
        }

        base_letters.insert(&instruction.requirement);
        base_letters.insert(&instruction.for_part);
    }

    (part1(&steps, &base_letters), part2(&steps, &base_letters))
}

pub fn part1(ins: &[Step], base_letters: &HashSet<&String>) -> String {
    let mut instructions: Vec<&Step> = ins.iter().collect();
    let mut finished = String::new();
    // Clone this since we need the original hashset for part 2.
    let mut poss_avail = base_letters.clone();

    // Go through the instructions...
    while !poss_avail.is_empty() {
        // Step 2: Filter all possibly available elements so that we only have elements that are
        // available.
        let mut all_avail: Vec<&String> = vec![];
        for &elem in &poss_avail {
            if instructions.iter().any(|x| x.for_part == *elem) {
                continue;
            }

            all_avail.push(elem);
        }

        // 2.1: Sort the available elements.
        if all_avail.is_empty() {
            panic!("Something went terribly wrong.");
        }

        all_avail.sort();

        // Step 3: Now that we have all elements that are available, take the first element and
        // then find any instructions that has this element has a requirement.
        let target_elem = all_avail[0];
        instructions.retain(|&item| {
            if *target_elem == item.requirement {
                poss_avail.insert(&item.for_part);
                return false;
            }

            true
        });

        poss_avail.retain(|&item| item != target_elem);

        // Step 4: Add this element to the finished vector.
        finished += target_elem.as_str();
    }

    finished
}

pub fn part2(ins: &[Step], base_letters: &HashSet<&String>) -> usize {
    let mut instructions: Vec<&Step> = ins.iter().collect();
    let mut finished = String::new();
    let mut poss_avail: HashSet<&String> = base_letters.iter().copied().collect();

    let mut workers: [Worker; 5] = [
        Worker {
            id: 1,
            curr_job: EMPTY_STR,
            time_left: 0,
        },
        Worker {
            id: 2,
            curr_job: EMPTY_STR,
            time_left: 0,
        },
        Worker {
            id: 3,
            curr_job: EMPTY_STR,
            time_left: 0,
        },
        Worker {
            id: 4,
            curr_job: EMPTY_STR,
            time_left: 0,
        },
        Worker {
            id: 5,
            curr_job: EMPTY_STR,
            time_left: 0,
        },
    ];

    let mut time_taken: usize = 0;
    loop {
        if poss_avail.is_empty() && workers.iter().all(|x| !x.has_assigned_job()) {
            break;
        }

        // Get all instructions that we can work on.
        let mut all_avail: Vec<&String> = vec![];
        for &elem in &poss_avail {
            if instructions.iter().any(|x| x.for_part == *elem) {
                continue;
            }

            all_avail.push(elem);
        }

        // Sort them.
        all_avail.sort_unstable();

        // Then, allocate these jobs to the workers.
        // TODO how to not use an indexed-for loop to set values?
        // Seems like if you want to use a normal for each loop, an error will be brought up
        // regarding immutability.
        for worker in &mut workers {
            if worker.has_assigned_job() {
                continue;
            }

            if all_avail.is_empty() {
                break;
            }

            worker.set_job(all_avail[0]);
            poss_avail.retain(|&item| item != all_avail[0]);
            all_avail.remove(0);
        }

        // And then start working.
        for i in 0..workers.len() {
            if !workers[i].has_assigned_job() {
                continue;
            }

            let is_done = workers[i].work();
            if is_done {
                let completed_job = workers[i].return_job_and_reset().unwrap();
                assert_ne!(completed_job, EMPTY_STR);

                // Remove any instructions that we've essentially done, and adds more instructions
                // that we could possibly work on.
                instructions.retain(|&item| {
                    if completed_job == item.requirement {
                        poss_avail.insert(&item.for_part);
                        return false;
                    }

                    true
                });

                finished += completed_job.as_str();
            }
        }

        time_taken += 1;
    }

    time_taken
}

#[derive(Clone, Debug)]
pub struct Step {
    requirement: String,
    for_part: String,
}

const EMPTY_STR: String = String::new();

pub struct Worker {
    #[allow(dead_code)]
    id: i8,
    curr_job: String,
    time_left: usize,
}

impl Worker {
    /// Sets this worker's job. If the worker already has a job, then this does not do anything.
    ///
    /// # Parameters
    /// * `job_id` - The job ID.
    ///
    /// # Returns
    /// `true` if the job was set; `false` otherwise.
    fn set_job(&mut self, job_id: &str) -> bool {
        if self.time_left > 0 {
            return false;
        }

        self.curr_job = job_id.to_string();
        self.time_left = 60 + (job_id.as_bytes()[0] as usize) - 64;
        true
    }

    /// Checks if this worker has an assigned job. This does not check if the time remaining is 0,
    /// only that this worker has a job assigned to it.
    ///
    /// # Returns
    /// `true` if this worker has a job, `false` otherwise.
    fn has_assigned_job(&self) -> bool {
        self.curr_job != EMPTY_STR
    }

    /// Makes the worker work for 1 second. All this does is decrements the `time_left` member by 1
    /// (essentially simulating 1 second of work).
    ///
    /// # Returns
    /// `true` if this worker completed its job, `false` if the worker still needs to work.
    fn work(&mut self) -> bool {
        if self.curr_job == EMPTY_STR {
            return false;
        }

        self.time_left -= 1;
        self.time_left == 0
    }

    /// Returns the job ID and resets it.
    ///
    /// # Returns
    /// A string representing the job ID if the worker is not working. If the worker is working,
    /// then this returns nothing.
    fn return_job_and_reset(&mut self) -> Option<String> {
        if self.is_working() {
            return None;
        }

        let str_to_return = String::from(&self.curr_job);
        self.curr_job = EMPTY_STR;
        Some(str_to_return)
    }

    /// Checks if the worker still needs to work on the job.
    ///
    /// # Returns
    /// `true` if the worker is still working, `false` otherwise.
    fn is_working(&self) -> bool {
        self.time_left > 0
    }
}
