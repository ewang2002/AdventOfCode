use std::collections::HashSet;

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (String, usize) {
    let steps: Vec<_> = input.iter()
        .map(|x| {
            // Step C must be finished before step A can begin.
            // 0    1 2    3  4        5      6    7 8   9
            let res = x.split(" ").collect::<Vec<_>>();
            return Step { requirement: res[1].to_string(), for_part: res[7].to_string() };
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

    return (part1(&steps, &base_letters), part2(&steps, &base_letters));
}

// https://adventofcode.com/2018/day/7
//
// --- Day 7: The Sum of Its Parts ---
// You find yourself standing on a snow-covered coastline; apparently, you landed a little off
// course. The region is too hilly to see the North Pole from here, but you do spot some Elves
// that seem to be trying to unpack something that washed ashore. It's quite cold out, so you
// decide to risk creating a paradox by asking them for directions.
//
// "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018
// speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a
// translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.
//
// "We do need to find our way back to the North Pole, but we have higher priorities at the moment.
// You see, believe it or not, this box contains something that will solve all of Santa's
// transportation problems - at least, that's what it looks like from the pictures in the
// instructions." It doesn't seem like they can read whatever language it's in, but you can:
// "Sleigh kit. Some assembly required."
//
// "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start
// excitedly pulling more parts out of the box.
//
// The instructions specify a series of steps and requirements about which steps must be finished
// before others can begin (your puzzle input). Each step is designated by a single letter. For
// example, suppose you have the following instructions:
//
// - Step C must be finished before step A can begin.
// - Step C must be finished before step F can begin.
// - Step A must be finished before step B can begin.
// - Step A must be finished before step D can begin.
// - Step B must be finished before step E can begin.
// - Step D must be finished before step E can begin.
// - Step F must be finished before step E can begin.
// Visually, these requirements look like this:
//
//   -->A--->B--
//  /    \      \
// C      -->D----->E
//  \           /
//   ---->F-----
// Your first goal is to determine the order in which the steps should be completed. If more than
// one step is ready, choose the step which is first alphabetically. In this example, the steps
// would be completed as follows:
//
// - Only C is available, and so it is done first.
// - Next, both A and F are available. A is first alphabetically, so it is done next.
// - Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
// - After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
// - F is the only choice, so it is done next.
// - Finally, E is completed.
// So, in this example, the correct order is CABDFE.
//
// In what order should the steps in your instructions be completed?

pub fn part1(ins: &Vec<Step>, base_letters: &HashSet<&String>) -> String {
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

            return true;
        });

        poss_avail.retain(|&item| item != target_elem);

        // Step 4: Add this element to the finished vector.
        finished += target_elem.as_str();
    }

    return finished;
}

// --- Part Two ---
// As you're about to begin construction, four of the Elves offer to help. "The sun will set soon;
// it'll go faster if we work together." Now, you need to account for multiple people working on
// steps simultaneously. If multiple steps are available, workers should still begin them in
// alphabetical order.
//
// Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3, and so on.
// So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds. No time is required
// between steps.
//
// To simplify things for the example, however, suppose you only have help from one Elf (a total of
// two workers) and that each step takes 60 fewer seconds (so that step A takes 1 second and step Z
// takes 26 seconds). Then, using the same instructions as above, this is how each second would be
// spent:
//
// Second   Worker 1   Worker 2   Done
//    0        C          .
//    1        C          .
//    2        C          .
//    3        A          F       C
//    4        B          F       CA
//    5        B          F       CA
//    6        D          F       CAB
//    7        D          F       CAB
//    8        D          F       CAB
//    9        D          .       CABF
//   10        E          .       CABFD
//   11        E          .       CABFD
//   12        E          .       CABFD
//   13        E          .       CABFD
//   14        E          .       CABFD
//   15        .          .       CABFDE
//
// Each row represents one second of time. The Second column identifies how many seconds have
// passed as of the beginning of that second. Each worker column shows the step that worker is
// currently doing (or . if they are idle). The Done column shows completed steps.
//
// Note that the order of the steps has changed; this is because steps now take time to finish and
// multiple workers can begin multiple steps simultaneously.
//
// In this example, it would take 15 seconds for two workers to complete these steps.
//
// With 5 workers and the 60+ second step durations described above, how long will it take to
// complete all of the steps?

pub fn part2(ins: &Vec<Step>, base_letters: &HashSet<&String>) -> usize {
    let mut instructions: Vec<&Step> = ins.iter().collect();
    let mut finished = String::new();
    let mut poss_avail: HashSet<&String> = base_letters.iter().map(|x| *x).collect();

    let mut workers: [Worker; 5] = [
        Worker { id: 1, curr_job: EMPTY_STR, time_left: 0 },
        Worker { id: 2, curr_job: EMPTY_STR, time_left: 0 },
        Worker { id: 3, curr_job: EMPTY_STR, time_left: 0 },
        Worker { id: 4, curr_job: EMPTY_STR, time_left: 0 },
        Worker { id: 5, curr_job: EMPTY_STR, time_left: 0 }
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
        for i in 0..workers.len() {
            if workers[i].has_assigned_job() {
                continue;
            }

            if all_avail.len() == 0 {
                break;
            }

            workers[i].set_job(all_avail[0]);
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

                    return true;
                });

                finished += completed_job.as_str();
            }
        }

        time_taken += 1;
    }

    return time_taken;
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
    fn set_job(&mut self, job_id: &String) -> bool {
        if self.time_left > 0 {
            return false;
        }

        self.curr_job = job_id.to_string();
        self.time_left = 60 + (job_id.as_bytes()[0] as usize) - 64;
        return true;
    }

    /// Checks if this worker has an assigned job. This does not check if the time remaining is 0,
    /// only that this worker has a job assigned to it.
    ///
    /// # Returns
    /// `true` if this worker has a job, `false` otherwise.
    fn has_assigned_job(&self) -> bool {
        return self.curr_job != EMPTY_STR;
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
        return self.time_left == 0;
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
        return Some(str_to_return);
    }

    /// Checks if the worker still needs to work on the job.
    ///
    /// # Returns
    /// `true` if the worker is still working, `false` otherwise.
    fn is_working(&self) -> bool {
        return self.time_left > 0;
    }
}