use std::cmp::Ordering;
use std::collections::{HashSet};
use std::fmt::{Display, Formatter, Write};

const STRAIGHT_VERT: char = '|';
const STRAIGHT_HORIZ: char = '-';
const CURVE_SLASH: char = '/';
const CURVE_BACKSLASH: char = '\\';
const INTERSECTION: char = '+';

const CART_UP: char = '^';
const CART_DOWN: char = 'v';
const CART_LEFT: char = '<';
const CART_RIGHT: char = '>';

type Cart = (char, u8, usize, usize);
type Coord = (usize, usize);

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (String, String) {
    let char_input = input.iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut track_system: Vec<Vec<Track>> = vec![];
    let mut active_carts: Vec<Cart> = vec![];
    for y in 0..char_input.len() {
        let mut row_track: Vec<Track> = vec![];
        for x in 0..char_input[y].len() {
            let track = Track::new(char_input[y][x], x, y);
            match char_input[y][x] {
                CART_UP | CART_DOWN | CART_RIGHT | CART_LEFT => {
                    active_carts.push((char_input[y][x], 0, x, y));
                }
                _ => {}
            }

            row_track.push(track);
        }

        track_system.push(row_track);
    }

    return (part1(&track_system, &active_carts), part2(&track_system, &active_carts));
}

// --- Day 13: Mine Cart Madness ---
// A crop of this size requires significant logistics to transport produce, soil, fertilizer, and
// so on. The Elves are very busy pushing things around in carts on some kind of rudimentary system
// of tracks they've come up with.
//
// Seeing as how cart-and-track systems don't appear in recorded history for another 1000 years,
// the Elves seem to be making this up as they go along. They haven't even figured out how to avoid
// collisions yet.
//
// You map out the tracks (your puzzle input) and see where you can help.
//
// Tracks consist of straight paths (| and -), curves (/ and \), and intersections (+). Curves
// connect exactly two perpendicular pieces of track; for example, this is a closed loop:
//
// /----\
// |    |
// |    |
// \----/
// Intersections occur when two perpendicular paths cross. At an intersection, a cart is capable of
// turning left, turning right, or continuing straight. Here are two loops connected by two
// intersections:
//
// /-----\
// |     |
// |  /--+--\
// |  |  |  |
// \--+--/  |
//    |     |
//    \-----/
// Several carts are also on the tracks. Carts always face either up (^), down (v), left (<), or
// right (>). (On your initial map, the track under each cart is a straight path matching the
// direction the cart is facing.)
//
// Each time a cart has the option to turn (by arriving at any intersection), it turns left the
// first time, goes straight the second time, turns right the third time, and then repeats those
// directions starting again with left the fourth time, straight the fifth time, and so on. This
// process is independent of the particular intersection at which the cart has arrived - that is,
// the cart has no per-intersection memory.
//
// Carts all move at the same speed; they take turns moving a single step at a time. They do this
// based on their current location: carts on the top row move first (acting from left to right),
// then carts on the second row move (again from left to right), then carts on the third row, and
// so on. Once each cart has moved one step, the process repeats; each of these loops is called a
// tick.
//
// For example, suppose there are two carts on a straight track:
//
// |  |  |  |  |
// v  |  |  |  |
// |  v  v  |  |
// |  |  |  v  X
// |  |  ^  ^  |
// ^  ^  |  |  |
// |  |  |  |  |
// First, the top cart moves. It is facing down (v), so it moves down one square. Second, the
// bottom cart moves. It is facing up (^), so it moves up one square. Because all carts have moved,
// the first tick ends. Then, the process repeats, starting with the first cart. The first cart
// moves down, then the second cart moves up - right into the first cart, colliding with it! (The
// location of the crash is marked with an X.) This ends the second and last tick.
//
// Here is a longer example:
//
//  /->-\
//  |   |  /----\
//  | /-+--+-\  |
//  | | |  | v  |
//  \-+-/  \-+--/
//    \------/
//
//  /-->\
//  |   |  /----\
//  | /-+--+-\  |
//  | | |  | |  |
//  \-+-/  \->--/
//    \------/
//
//  /---v
//  |   |  /----\
//  | /-+--+-\  |
//  | | |  | |  |
//  \-+-/  \-+>-/
//    \------/
//
//  /---\
//  |   v  /----\
//  | /-+--+-\  |
//  | | |  | |  |
//  \-+-/  \-+->/
//    \------/
//
//  /---\
//  |   |  /----\
//  | /->--+-\  |
//  | | |  | |  |
//  \-+-/  \-+--^
//    \------/
//
//  /---\
//  |   |  /----\
//  | /-+>-+-\  |
//  | | |  | |  ^
//  \-+-/  \-+--/
//    \------/
//
//  /---\
//  |   |  /----\
//  | /-+->+-\  ^
//  | | |  | |  |
//  \-+-/  \-+--/
//    \------/
//
//  /---\
//  |   |  /----<
//  | /-+-->-\  |
//  | | |  | |  |
//  \-+-/  \-+--/
//    \------/
//
//  /---\
//  |   |  /---<\
//  | /-+--+>\  |
//  | | |  | |  |
//  \-+-/  \-+--/
//    \------/
//
//  /---\
//  |   |  /--<-\
//  | /-+--+-v  |
//  | | |  | |  |
//  \-+-/  \-+--/
//    \------/
//
//  /---\
//  |   |  /-<--\
//  | /-+--+-\  |
//  | | |  | v  |
//  \-+-/  \-+--/
//    \------/
//
//  /---\
//  |   |  /<---\
//  | /-+--+-\  |
//  | | |  | |  |
//  \-+-/  \-<--/
//    \------/
//
//  /---\
//  |   |  v----\
//  | /-+--+-\  |
//  | | |  | |  |
//  \-+-/  \<+--/
//    \------/
//
//  /---\
//  |   |  /----\
//  | /-+--v-\  |
//  | | |  | |  |
//  \-+-/  ^-+--/
//    \------/
//
//  /---\
//  |   |  /----\
//  | /-+--+-\  |
//  | | |  X |  |
//  \-+-/  \-+--/
//    \------/
//
// After following their respective paths for a while, the carts eventually crash. To help prevent
// crashes, you'd like to know the location of the first crash. Locations are given in X,Y
// coordinates, where the furthest left column is X=0 and the furthest top row is Y=0:
//
//             111
//   0123456789012
//  0/---\
//  1|   |  /----\
//  2| /-+--+-\  |
//  3| | |  X |  |
//  4\-+-/  \-+--/
//  5  \------/
// In this example, the location of the first crash is 7,3.

pub fn part1(track_system: &Vec<Vec<Track>>, active_carts: &Vec<Cart>) -> String {
    let mut all_active_coords: HashSet<Coord> = active_carts
        .iter()
        .map(|x| ((*x).2, (*x).3))
        .collect();

    let mut all_active_carts = active_carts.clone();


    let mut conflicting_coords: Option<Coord> = None;
    while conflicting_coords.is_none() {
        let mut next_active_carts: Vec<Cart> = vec![];
        let mut next_active_coords: HashSet<Coord> = HashSet::new();
        for cart in all_active_carts {
            let (_, dir, x, y) = cart;
            all_active_coords.remove(&(x, y));
            // determine the next track
            let (next_cart, next_track) = get_next_cart_track(cart, track_system);
            next_active_carts.push((
                next_cart,
                if next_track.raw_track_type == INTERSECTION { (dir + 1) % 3 } else { dir },
                next_track.x_pos,
                next_track.y_pos
            ));

            let coord: Coord = (next_track.x_pos, next_track.y_pos);
            // Crash occurred
            if next_active_coords.contains(&coord) || all_active_coords.contains(&coord) {
                conflicting_coords = Some(coord);
                break;
            }

            next_active_coords.insert(coord);
        }

        all_active_carts = next_active_carts;
        all_active_coords = next_active_coords;
        sort_carts_by_row_col(&mut all_active_carts);
    }

    return format!("{},{}", conflicting_coords.unwrap().0, conflicting_coords.unwrap().1);
}

// --- Part Two ---
// There isn't much you can do to prevent crashes in this ridiculous system. However, by predicting
// the crashes, the Elves know where to be in advance and instantly remove the two crashing carts
// the moment any crash occurs.
//
// They can proceed like this for a while, but eventually, they're going to run out of carts. It
// could be useful to figure out where the last cart that hasn't crashed will end up.
//
// For example:
//
//  />-<\
//  |   |
//  | /<+-\
//  | | | v
//  \>+</ |
//    |   ^
//    \<->/
//
//  /---\
//  |   |
//   | v-+-\
//  | | | |
//  \-+-/ |
//    |   |
//    ^---^
//
//  /---\
//  |   |
//  | /-+-\
//  | v | |
//  \-+-/ |
//    ^   ^
//    \---/
//
//  /---\
//  |   |
//  | /-+-\
//  | | | |
//  \-+-/ ^
//    |   |
//    \---/
//
// After four very expensive crashes, a tick ends with only one cart remaining; its final location
// is 6,4.
//
// What is the location of the last cart at the end of the first tick where it is the only cart
// left?

pub fn part2(track_system: &Vec<Vec<Track>>, active_carts: &Vec<Cart>) -> String {
    let mut all_active_coords: HashSet<Coord> = active_carts
        .iter()
        .map(|x| ((*x).2, (*x).3))
        .collect();

    let mut all_active_carts = active_carts.clone();

    while all_active_carts.len() > 1 {
        let mut new_active_carts: Vec<Cart> = vec![];
        let mut next_active_coords: HashSet<Coord> = HashSet::new();

        for cart in all_active_carts {
            let (_, dir, x, y) = cart;
            // If the active coordinates do not contain this cart's coordinates, then a crash must
            // have happened so we can simply skip this cart.
            if !all_active_coords.contains(&(x, y)) {
                continue;
            }

            all_active_coords.remove(&(x, y));
            // determine the next track
            let (next_cart, next_track) = get_next_cart_track(cart, track_system);

            let new_coord = (next_track.x_pos, next_track.y_pos);
            let new_cart = (
                next_cart,
                if next_track.raw_track_type == INTERSECTION { (dir + 1) % 3 } else { dir },
                next_track.x_pos,
                next_track.y_pos
            );
            // Is this coordinate in the hashset of active coordinates? If so, then don't add the
            // next cart
            if all_active_coords.contains(&new_coord) {
                all_active_coords.remove(&new_coord);
                continue;
            }

            // Is this coordinate in the hashset of new coordinates? If so, then remove it from
            // said hashset and additionally remove the offending cart
            if next_active_coords.contains(&new_coord) {
                next_active_coords.remove(&new_coord);
                new_active_carts.retain(|x| (*x).2 != new_coord.0 || (*x).3 != new_coord.1);
                continue;
            }

            new_active_carts.push(new_cart);
            next_active_coords.insert(new_coord);
        }

        all_active_coords = next_active_coords;
        all_active_carts = new_active_carts;
        sort_carts_by_row_col(&mut all_active_carts);
    }

    return format!("{},{}", all_active_carts[0].2, all_active_carts[0].3);
}


/// Prints the track out. Useful for debugging.
///
/// # Parameters
/// - `track`: The track.
/// - `carts`: The carts.
#[allow(dead_code)]
fn print_track(track: &Vec<Vec<Track>>, carts: &Vec<Cart>) -> () {
    for y in 0..track.len() {
        for x in 0..track[y].len() {
            let res = carts.iter().find(|r| (**r).2 == x && (**r).3 == y);
            match res {
                Some(c) => print!("{}", (*c).0),
                None => print!("{}", track[y][x].raw_track_type)
            };
        }

        println!();
    }

    println!();
}

pub struct Track {
    raw_track_type: char,
    x_pos: usize,
    y_pos: usize,
}

impl Track {
    /// Creates a new `Track` with the specified character and position.
    ///
    /// # Parameters
    /// - `track_char`: The track character. Should be one of < > ^ v / \ | - +.
    /// - `x_pos`: The `x`-position of this track.
    /// - `y_pos`: The `y`-position of this track.
    pub fn new(track_char: char, x_pos: usize, y_pos: usize) -> Self {
        let track_to_use: char = match track_char {
            CART_UP | CART_DOWN => STRAIGHT_VERT,
            CART_RIGHT | CART_LEFT => STRAIGHT_HORIZ,
            rest => rest
        };

        return Track { x_pos, y_pos, raw_track_type: track_to_use };
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_char(self.raw_track_type);
    }
}

/// Sorts all carts by row/column order. This will first sort every cart by x-coordinate, and then
/// sort every cart by y-coordinate.
///
/// Remember: Carts on the top row move first (acting from left to right), then carts on the second
/// row move (again from left to right), then carts on the third row, and so on.
///
/// # Parameters
/// - `all_active_carts`: All active carts.
fn sort_carts_by_row_col(all_active_carts: &mut Vec<Cart>) -> () {
    all_active_carts.sort_by(|cart_a, cart_b| {
        let (_, _, ax, ay) = *cart_a;
        let (_, _, bx, by) = *cart_b;
        if ax < bx {
            return Ordering::Less;
        } else if ax > bx {
            return Ordering::Greater;
        }

        if ay < by {
            return Ordering::Less;
        } else if ay > by {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    })
}

/// Gets the next cart and track.
///
/// # Parameters
/// - `cart_tuple`: The cart itself.
/// - `track_system`: The track system.
///
/// # Returns
/// - A tuple containing the next cart character and the next track. You will need to manually
/// adjust any properties (like direction count, if applicable).
fn get_next_cart_track(cart_tuple: Cart, track_system: &Vec<Vec<Track>>) -> (char, &Track) {
    let (cart, dir, x, y) = cart_tuple;
    let next_track = match cart {
        CART_DOWN => &track_system[y + 1][x],
        CART_UP => &track_system[y - 1][x],
        CART_RIGHT => &track_system[y][x + 1],
        CART_LEFT => &track_system[y][x - 1],
        _ => panic!("invalid cart given: {}", cart)
    };

    // determine the next cart
    let next_cart = match next_track.raw_track_type {
        STRAIGHT_VERT | STRAIGHT_HORIZ => cart,
        CURVE_SLASH => match cart {
            '^' => '>',
            '<' => 'v',
            '>' => '^',
            'v' => '<',
            _ => panic!("invalid cart {} given with dir {}", cart, CURVE_SLASH)
        },
        CURVE_BACKSLASH => match cart {
            '>' => 'v',
            '^' => '<',
            '<' => '^',
            'v' => '>',
            _ => panic!("invalid cart {} given with dir {}", cart, CURVE_SLASH)
        },
        INTERSECTION => match dir {
            // Left
            0 => match cart {
                'v' => '>',
                '>' => '^',
                '^' => '<',
                '<' => 'v',
                _ => panic!("invalid cart {}", cart)
            },
            // Straight
            1 => cart,
            // Right
            2 => match cart {
                'v' => '<',
                '>' => 'v',
                '^' => '>',
                '<' => '^',
                _ => panic!("invalid cart {}", cart)
            },
            _ => panic!("invalid cart {} given with dir", cart)
        },
        _ => panic!("invalid track {}", next_track.raw_track_type)
    };

    return (next_cart, next_track);
}