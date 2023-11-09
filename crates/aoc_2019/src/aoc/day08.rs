use common::day::{AoCProblem, Solution};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const AREA_OF_IMG: usize = WIDTH * HEIGHT;

pub struct Day08 {
    input: Vec<u32>,
}

// https://adventofcode.com/2019/day/8
impl AoCProblem for Day08 {
    fn prepare(input: String) -> Self {
        Self {
            input: input
                .lines()
                .nth(0)
                .unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut mult_12 = 0;
        let mut num_zeros = isize::MAX;

        let mut i = 0;
        while i < self.input.len() {
            let mut temp_zeros = 0;
            let mut temp_1 = 0;
            let mut temp_2 = 0;

            let mut num_pixels = 0;
            while i < self.input.len() && num_pixels < AREA_OF_IMG {
                match self.input[i] {
                    0 => temp_zeros += 1,
                    1 => temp_1 += 1,
                    2 => temp_2 += 1,
                    _ => {}
                }

                i += 1;
                num_pixels += 1;
            }

            if temp_zeros < num_zeros {
                num_zeros = temp_zeros;
                mult_12 = temp_1 * temp_2;
            }
        }

        mult_12.into()
    }

    fn part2(&mut self) -> Solution {
        let mut layers: Vec<[[u32; WIDTH]; HEIGHT]> = vec![];
        let mut i = 0;

        while i < self.input.len() {
            let mut layer = [[u32::MAX; WIDTH]; HEIGHT];
            let mut num_pixels = 0;
            let mut x = 0;
            let mut y = 0;

            while num_pixels < AREA_OF_IMG {
                layer[x][y] = self.input[i];
                i += 1;
                num_pixels += 1;

                y += 1;
                if y >= WIDTH {
                    y = 0;
                    x += 1;
                }
            }

            layers.push(layer);
        }

        let mut s = String::new();
        for h in 0..HEIGHT {
            s.push('\n');
            for w in 0..WIDTH {
                let mut this_pixel = 2;
                for layer in &layers {
                    this_pixel = match layer[h][w] {
                        0 | 1 => layer[h][w],
                        _ => continue,
                    };

                    if this_pixel != 2 {
                        break;
                    }
                }

                s.push(if this_pixel == 1 { '#' } else { '.' });
            }
        }

        s.into()
    }
}
