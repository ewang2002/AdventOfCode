use std::cmp::Ordering;
use std::collections::HashSet;
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

// https://adventofcode.com/2018/day/13
#[allow(dead_code)]
pub fn execute(input: &[String]) -> (String, String) {
    let char_input = input
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut track_system: Vec<Vec<Track>> = vec![];
    let mut active_carts: Vec<Cart> = vec![];
    for (y, v) in char_input.iter().enumerate() {
        let mut row_track: Vec<Track> = vec![];
        for (x, c) in v.iter().enumerate() {
            let track = Track::new(*c, x, y);
            match *c {
                CART_UP | CART_DOWN | CART_RIGHT | CART_LEFT => {
                    active_carts.push((*c, 0, x, y));
                }
                _ => {}
            }

            row_track.push(track);
        }

        track_system.push(row_track);
    }

    (
        part1(&track_system, &active_carts),
        part2(&track_system, &active_carts),
    )
}

pub fn part1(track_system: &[Vec<Track>], active_carts: &[Cart]) -> String {
    let mut all_active_coords: HashSet<Coord> =
        active_carts.iter().map(|x| ((*x).2, (*x).3)).collect();

    let mut all_active_carts = active_carts.to_vec();

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
                if next_track.raw_track_type == INTERSECTION {
                    (dir + 1) % 3
                } else {
                    dir
                },
                next_track.x_pos,
                next_track.y_pos,
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

    format!(
        "{},{}",
        conflicting_coords.unwrap().0,
        conflicting_coords.unwrap().1
    )
}

pub fn part2(track_system: &[Vec<Track>], active_carts: &[Cart]) -> String {
    let mut all_active_coords: HashSet<Coord> =
        active_carts.iter().map(|x| ((*x).2, (*x).3)).collect();

    let mut all_active_carts = active_carts.to_owned();

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
                if next_track.raw_track_type == INTERSECTION {
                    (dir + 1) % 3
                } else {
                    dir
                },
                next_track.x_pos,
                next_track.y_pos,
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
            rest => rest,
        };

        Track {
            x_pos,
            y_pos,
            raw_track_type: track_to_use,
        }
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.raw_track_type)
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
fn sort_carts_by_row_col(all_active_carts: &mut [Cart]) {
    all_active_carts.sort_by(|cart_a, cart_b| {
        let (_, _, ax, ay) = *cart_a;
        let (_, _, bx, by) = *cart_b;
        match ax.cmp(&bx) {
            Ordering::Equal => ay.cmp(&by),
            o => o,
        }
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
fn get_next_cart_track(cart_tuple: Cart, track_system: &[Vec<Track>]) -> (char, &Track) {
    let (cart, dir, x, y) = cart_tuple;
    let next_track = match cart {
        CART_DOWN => &track_system[y + 1][x],
        CART_UP => &track_system[y - 1][x],
        CART_RIGHT => &track_system[y][x + 1],
        CART_LEFT => &track_system[y][x - 1],
        _ => panic!("invalid cart given: {}", cart),
    };

    // determine the next cart
    let next_cart = match next_track.raw_track_type {
        STRAIGHT_VERT | STRAIGHT_HORIZ => cart,
        CURVE_SLASH => match cart {
            '^' => '>',
            '<' => 'v',
            '>' => '^',
            'v' => '<',
            _ => panic!("invalid cart {} given with dir {}", cart, CURVE_SLASH),
        },
        CURVE_BACKSLASH => match cart {
            '>' => 'v',
            '^' => '<',
            '<' => '^',
            'v' => '>',
            _ => panic!("invalid cart {} given with dir {}", cart, CURVE_SLASH),
        },
        INTERSECTION => match dir {
            // Left
            0 => match cart {
                'v' => '>',
                '>' => '^',
                '^' => '<',
                '<' => 'v',
                _ => panic!("invalid cart {}", cart),
            },
            // Straight
            1 => cart,
            // Right
            2 => match cart {
                'v' => '<',
                '>' => 'v',
                '^' => '>',
                '<' => '^',
                _ => panic!("invalid cart {}", cart),
            },
            _ => panic!("invalid cart {} given with dir", cart),
        },
        _ => panic!("invalid track {}", next_track.raw_track_type),
    };

    (next_cart, next_track)
}
