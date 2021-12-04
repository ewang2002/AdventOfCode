use chrono::{NaiveDateTime, Timelike};
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (i32, usize) {
    let (events, guards) = get_guards_and_events(input);
    // Now determine how much time was spent sleeping
    let (guard_slept_time, guard_most_occurring_min) = get_guard_sleep_time(events, guards);

    return (part1(&guard_slept_time, &guard_most_occurring_min), part2(&guard_most_occurring_min));
}

// https://adventofcode.com/2018/day/4
//
// --- Day 4: Repose Record ---
//
// You've sneaked into another supply closet - this time, it's across from the prototype suit
// manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a
// guard stationed outside the lab, so this is as close as you can safely get.
//
// As you search the closet for anything that might help, you discover that you're not the first
// person to want to sneak in. Covering the walls, someone has spent an hour starting every
// midnight for the past few months secretly observing this guard post! They've been writing down
// the ID of the one guard on duty that night - the Elves seem to have decided that one guard was
// enough for the overnight shift - as well as when they fall asleep or wake up while at their post
// (your puzzle input).
//
// For example, consider the following records, which have already been organized into
// chronological order:
//
// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up
// Timestamps are written using year-month-day hour:minute format. The guard falling asleep or
// waking up is always the one whose shift most recently started. Because all asleep/awake times
// are during the midnight hour (00:00 - 00:59), only the minute portion (00 - 59) is relevant for
// those events.
//
// Visually, these records show that the guards are asleep at these times:
//
//  Date   ID   Minute
//              000000000011111111112222222222333333333344444444445555555555
//              012345678901234567890123456789012345678901234567890123456789
//  11-01  #10  .....####################.....#########################.....
//  11-02  #99  ........................................##########..........
//  11-03  #10  ........................#####...............................
//  11-04  #99  ....................................##########..............
//  11-05  #99  .............................................##########.....
//
// The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the
// guard on duty that day; and Minute, which shows the minutes during which the guard was asleep
// within the midnight hour. (The Minute column's header shows the minute's ten's digit in the
// first row and the one's digit in the second row.) Awake is shown as ., and asleep is shown as #.
//
// Note that guards count as asleep on the minute they fall asleep, and they count as awake on the
// minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25
// is marked as awake.
//
// If you can figure out the guard most likely to be asleep at a specific time, you might be able
// to trick that guard into working tonight so you can have the best chance of sneaking in. You
// have two strategies for choosing the best guard/minute combination.
//
// Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend
// asleep the most?
//
// In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5),
// while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most
// during minute 24 (on two days, whereas any other minute the guard was asleep was only seen on
// one day).
//
// While this example listed the entries in chronological order, your entries are in the order you
// found them. You'll need to organize them before they can be analyzed.
//
// What is the ID of the guard you chose multiplied by the minute you chose? (In the above example,
// the answer would be 10 * 24 = 240.)

pub fn part1(guard_slept_time: &HashMap<u32, i64>, guard_most_occurring_min: &HashMap<u32, [usize; 60]>) -> i32 {
    // Find the laziest guard
    let laziest_guard = guard_slept_time
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .expect("Something went wrong when trying to find max.");

    // Find index corresponding to minute that is most encountered in sleeping process
    let longest_time = guard_most_occurring_min.get(&laziest_guard).unwrap()
        .iter()
        .enumerate()
        .max_by(|(_, v), (_, w)| v.cmp(w))
        .map(|(idx, _)| idx)
        .expect("Something bad happened.");

    return (laziest_guard * longest_time as u32) as i32;
}

// --- Part Two ---
// Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
//
// In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute -
// three times in total. (In all other cases, any guard spent any minute asleep at most twice.)
//
// What is the ID of the guard you chose multiplied by the minute you chose? (In the above example,
// the answer would be 99 * 45 = 4455.)

pub fn part2(guard_most_occurring_min: &HashMap<u32, [usize; 60]>) -> usize {
    let mut occurrences: usize = 0;
    let mut minute_most_occurring: usize = 0;
    let mut guard_id: u32 = 0;

    for (guard, time_table) in guard_most_occurring_min {
        for j in 0..time_table.len() {
            if time_table[j] > occurrences {
                occurrences = time_table[j];
                minute_most_occurring = j;
                guard_id = *guard;
            }
        }
    }

    assert_ne!(guard_id, 0);
    assert_ne!(minute_most_occurring, 0);
    return guard_id as usize * minute_most_occurring;
}

#[derive(Debug)]
struct Event {
    time: NaiveDateTime,
    guard_num: u32,
    event_type: EventType
}

#[derive(Debug)]
#[derive(PartialEq)]
enum EventType {
    BeginShift,
    FallAsleep,
    WakesUp
}

fn get_guard_sleep_time(events: Vec<Event>, guards: HashSet<u32>) -> (HashMap<u32, i64>, HashMap<u32, [usize; 60]>) {
    let mut guard_slept_time: HashMap<u32, i64> = HashMap::new();
    let mut guard_most_occurring_min: HashMap<u32, [usize; 60]> = HashMap::new();
    for guard in guards {
        let corr_events: Vec<&Event> = events
            .iter()
            .filter(|&x| x.guard_num == guard && x.event_type != EventType::BeginShift)
            .collect();

        // Index is minutes (for example, time_table[5] = 5 minutes)
        // Value is number of times encountered (for example, if time_table[5] is 3, then we saw 5
        // minutes 3 times).
        let mut time_table: [usize; 60] = [0; 60];
        let mut time_slept: i64 = 0;
        // Pair every two elements
        // [Falls asleep, wakes up] [Falls asleep, wakes up] ...
        for i in (1..corr_events.len()).step_by(2) {
            let start_sleep_time = corr_events[i - 1].time;
            let sleep_session = corr_events[i].time - start_sleep_time;
            time_slept += sleep_session.num_minutes();

            let start_sleep_time_i64 = start_sleep_time.minute() as i64;
            for j in start_sleep_time_i64..(start_sleep_time_i64 + sleep_session.num_minutes()) {
                time_table[(j as usize) % 60] += 1;
            }
        }

        guard_most_occurring_min.insert(guard, time_table);
        guard_slept_time.insert(guard, time_slept);
    }
    (guard_slept_time, guard_most_occurring_min)
}

fn get_guards_and_events(input: &Vec<String>) -> (Vec<Event>, HashSet<u32>) {
    let mut date_event: Vec<(NaiveDateTime, String)> = Vec::new();

    for line in input {
        let date_time = NaiveDateTime::parse_from_str(
            line.split(&['[', ']'][..]).collect::<Vec<_>>()[1],
            "%Y-%m-%d %H:%M"
        ).expect(format!("Error parsing \"{}\"", line).as_str());
        date_event.push((date_time, line.split("] ").collect::<Vec<_>>()[1].parse().unwrap()))
    }

    date_event.sort_by(|a, b| a.cmp(b));

    // Populate events vector
    let mut events: Vec<Event> = Vec::new();
    let mut guards: HashSet<u32> = HashSet::new();
    let mut current_guard = 0;
    for (d, e) in &date_event {
        if e.starts_with("Guard") {
            current_guard = get_guard_id(&e);
            guards.insert(current_guard);
            events.push(Event {
                time: *d,
                guard_num: current_guard,
                event_type: EventType::BeginShift
            });
            continue;
        }

        if e.starts_with("falls") {
            events.push(Event {
                time: *d,
                guard_num: current_guard,
                event_type: EventType::FallAsleep
            });
            continue;
        }

        if e.starts_with("wakes") {
            events.push(Event {
                time: *d,
                guard_num: current_guard,
                event_type: EventType::WakesUp
            });
            continue;
        }
    }

    assert_eq!(date_event.len(), events.len());
    return (events, guards);
}

fn get_guard_id(str: &String) -> u32 {
    return str.split("#")
        .flat_map(|x| x.split(" begins"))
        .collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
}
