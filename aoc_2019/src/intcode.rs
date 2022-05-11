use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

const ADD: isize = 1;
const MULTIPLY: isize = 2;
const INPUT: isize = 3;
const OUTPUT: isize = 4;
const JMP_IF_TRUE: isize = 5;
const JMP_IF_FALSE: isize = 6;
const LESS_THAN: isize = 7;
const EQUALS: isize = 8;
const HALT: isize = 99;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum ModeType {
    /// Causes a parameter to be interpreted as a value. For example, if the parameter is 50, its
    /// value is simply 50.
    Immediate,
    /// Causes the parameter to be interpreted as a position. For example, if the parameter is 50,
    /// its value is the value stored at address 50 in memory.
    Position,
}

impl From<isize> for ModeType {
    fn from(val: isize) -> Self {
        match val {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("invalid value {} given", val),
        }
    }
}

pub struct IntCodeComputer {
    /// The original program code.
    original: Vec<isize>,
    /// The current program code.
    curr_prgm: Vec<isize>,
    /// The instruction pointer.
    ins_pointer: usize,
    /// The length of the opcodes.
    len: usize,
    /// The output from opcode 4.
    stdout: Vec<isize>,
    /// The input for opcode 3.
    stdin: VecDeque<isize>,
}

impl IntCodeComputer {
    /// Creates a new IntCodeComputer with the specified program.
    ///
    /// # Parameters
    /// - `prgm`: The program.
    /// - `input`: The input, to be processed in a queue.
    ///
    /// # Returns
    /// THe new computer.
    pub fn new(prgm: &[isize], input: Option<Vec<isize>>) -> Self {
        IntCodeComputer {
            original: Vec::from(prgm),
            curr_prgm: Vec::from(prgm),
            ins_pointer: 0,
            len: prgm.len(),
            stdout: vec![],
            stdin: match input {
                Some(v) => VecDeque::from(v),
                None => VecDeque::new()
            }
        }
    }

    /// Inputs the `isize` to the computer's input queue.
    ///
    /// # Parameters
    /// - `input`: The input.
    #[allow(dead_code)]
    pub fn input_to_stdin(&mut self, input: isize) {
        self.stdin.push_back(input);
    }

    /// Resets the program to the default.
    pub fn reset(&mut self) {
        self.ins_pointer = 0;
        for i in 0..self.len {
            self.curr_prgm[i] = self.original[i];
        }
    }

    /// Gets an immutable view into the current program.
    ///
    /// # Returns
    /// An immutable view into the program.
    #[allow(dead_code)]
    pub fn view_program(&self) -> &[isize] {
        &self.curr_prgm
    }

    /// Gets an immutable view into the standard output.
    ///
    /// # Returns
    /// An immutable view into the standard output of the program.
    #[allow(dead_code)]
    pub fn view_stdout(&self) -> &[isize] {
        &self.stdout
    }

    /// Runs the program.
    pub fn run(&mut self) {
        while self.ins_pointer < self.curr_prgm.len() {
            let (opcode, p1, p2, _) = interpret_opcode(self.curr_prgm[self.ins_pointer]);
            if opcode == HALT {
                break;
            }

            let num_args = get_args_needed(opcode);

            // 1 argument needed
            let v1 = self.get_value(1, p1);
            if num_args == 1 {
                match opcode {
                    INPUT => {
                        let input = self.stdin.pop_front().unwrap();
                        self.set_value(1, input);
                    },
                    OUTPUT => self.stdout.push(v1),
                    _ => panic!("Invalid or unknown opcode {}", opcode),
                };

                self.ins_pointer += 2;
                continue;
            }

            // 2 arguments needed
            let v2 = self.get_value(2, p2);
            if num_args == 2 {
                match opcode {
                    JMP_IF_TRUE => {
                        if v1 != 0 {
                            self.ins_pointer = v2 as usize;
                            continue;
                        }
                    },
                    JMP_IF_FALSE => {
                        if v1 == 0 {
                            self.ins_pointer = v2 as usize;
                            continue;
                        }
                    },
                    _ => panic!("Invalid or unknown opcode {}", opcode),
                };

                self.ins_pointer += 3;
                continue;
            }

            // 3 arguments needed, note that the third argument will implicitly be
            // used by `set_value`, since the third argument tells us where to put
            // the result of the operation.
            if num_args == 3 {
                match opcode {
                    ADD => self.set_value(3, v1 + v2),
                    MULTIPLY => self.set_value(3, v1 * v2),
                    LESS_THAN => {
                        self.set_value(3, if v1 < v2 { 1 } else { 0 })
                    },
                    EQUALS => {
                        self.set_value(3, if v1 == v2 { 1 } else { 0 })
                    },
                    _ => panic!("Invalid or unknown opcode {}", opcode),
                };

                self.ins_pointer += 4;
                continue;
            }

            // Unsupported opcode
            panic!("Unsupported opcode {}", opcode);
        }
    }

    /// Gets the value at the specified offset, for the given mode type.
    ///
    /// # Parameters
    /// - `offset`: The offset of the parameter, from the instruction pointer.
    /// - `mode_type`: The mode type for this parameter.
    ///
    /// # Returns
    /// The value.
    fn get_value(&self, offset: usize, mode_type: ModeType) -> isize {
        match mode_type {
            ModeType::Immediate => self.curr_prgm[self.ins_pointer + offset],
            ModeType::Position => {
                self.curr_prgm[self.curr_prgm[self.ins_pointer + offset] as usize]
            }
        }
    }

    /// Sets the value `new_val` to the current program at the index specified by the value at
    /// the current program's index at index `ins_pointer + offset`. In other words, this will
    /// perform the operation
    /// ```
    /// curr_prgm[curr_prgm[ins_pointer + offset]] = new_val
    /// ```
    ///
    /// For example, if you specified `offset = 2`, then `ins_pointer = 25`, then this will
    /// perform `curr_prgm[curr_prgm[25 + 2]] = new_val`.
    ///
    /// Note that this will implicitly require an additional argument. For example, with the
    /// case of the `ADD` operation, while two arguments are needed to get the first (`v1`) and
    /// second (`v2`) values, a "third" argument (usually indicated by `offset`, in our case
    /// here this would be `offset = 3`) is needed to determine where the resulting value
    /// `new_val = v1 + v2` should go.
    ///
    /// # Parameters
    /// - `offset`: The offset of the parameter, from the instruction pointer.
    /// - `new_val`: The new value.
    fn set_value(&mut self, offset: usize, new_val: isize) {
        // Parameters that an instruction writes to will never be in immediate mode.
        let idx = self.curr_prgm[self.ins_pointer + offset] as usize;
        self.curr_prgm[idx] = new_val;
    }
}

impl Index<usize> for IntCodeComputer {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.curr_prgm[index]
    }
}

impl IndexMut<usize> for IntCodeComputer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.curr_prgm[index]
    }
}

/// Gets the number of arguments needed for this opcode.
///
/// # Parameters
/// - `opcode`: The opcode.
///
/// # Returns
/// The number of arguments needed.
fn get_args_needed(opcode: isize) -> usize {
    match opcode {
        INPUT | OUTPUT => 1,
        JMP_IF_TRUE | JMP_IF_FALSE => 2,
        ADD | MULTIPLY | LESS_THAN | EQUALS => 3,
        _ => panic!("invalid opcode {}", opcode)
    }
}

/// Interprets the opcode & parameter from the given value.
///
/// # Parameters
/// - `raw_opcode`: The current opcode.
///
/// # Returns
/// A tuple where
/// - The first element is the opcode itself, and
/// - The second element is the mode of the first parameter, and
/// - The third element is the mode of the second parameter, and
/// - The fourth element is the mode of the third parameter.
fn interpret_opcode(raw_opcode: isize) -> (isize, ModeType, ModeType, ModeType) {
    let mut digits = get_digits(raw_opcode);
    digits.reverse();
    while digits.len() < 5 {
        digits.push(0);
    }
    digits.reverse();

    (
        digits[3] * 10 + digits[4],
        digits[2].into(),
        digits[1].into(),
        digits[0].into(),
    )
}

/// Gets the digits of an `isize`.
///
/// # Parameters
/// - `num`: The number.
///
/// # Returns
/// The digits.
fn get_digits(mut num: isize) -> Vec<isize> {
    let mut digits = Vec::new();
    if num == 0 {
        digits.push(0);
        return digits;
    }

    num = num.abs();

    loop {
        if num == 0 {
            break;
        }

        digits.push(num % 10);
        num /= 10;
    }

    digits.reverse();
    digits
}

/// Parses a string containing an Intcode program.
///
/// # Parameters
/// - `code`: The Intcode program.
///
/// # Returns
/// The parsed `Intcode` program.
pub fn parse_intcode(code: &str) -> Vec<isize> {
    code.split(',')
        .into_iter()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::intcode::{
        get_digits, interpret_opcode, parse_intcode, IntCodeComputer, ModeType, MULTIPLY,
    };

    #[test]
    pub fn test_get_digits() {
        assert_eq!([1, 0, 1, 0].as_slice(), get_digits(1010));
        assert_eq!([0].as_slice(), get_digits(0));
        assert_eq!([9, 9, 2, 9, 9].as_slice(), get_digits(99299));
        assert_eq!([5, 2].as_slice(), get_digits(0052));
    }

    #[test]
    pub fn test_interpret_opcode_param() {
        let (opcode, p1, p2, p3) = interpret_opcode(1002);
        assert_eq!(MULTIPLY, opcode);
        assert_eq!(ModeType::Position, p1);
        assert_eq!(ModeType::Immediate, p2);
        assert_eq!(ModeType::Position, p3);
    }

    #[test]
    pub fn intcode2_test_1() {
        let program = parse_intcode("1,0,0,0,99");
        let mut c = IntCodeComputer::new(&program, None);
        c.run();
        assert_eq!([2, 0, 0, 0, 99].as_slice(), &c.curr_prgm);
    }

    #[test]
    pub fn intcode2_test_2() {
        let program = parse_intcode("2,3,0,3,99");
        let mut c = IntCodeComputer::new(&program, None);
        c.run();
        assert_eq!([2, 3, 0, 6, 99].as_slice(), &c.curr_prgm);
    }

    #[test]
    pub fn intcode2_test_3() {
        let program = parse_intcode("2,4,4,5,99,0");
        let mut c = IntCodeComputer::new(&program, None);
        c.run();
        assert_eq!([2, 4, 4, 5, 99, 9801].as_slice(), &c.curr_prgm);
    }

    #[test]
    pub fn intcode2_test_4() {
        let program = parse_intcode("1,1,1,4,99,5,6,0,99");
        let mut c = IntCodeComputer::new(&program, None);
        c.run();
        assert_eq!([30, 1, 1, 4, 2, 5, 6, 0, 99].as_slice(), c.curr_prgm);
    }

    #[test]
    pub fn intcode5_test_multiply_mixed() {
        let program = parse_intcode("1002,4,3,4,33");
        let mut c = IntCodeComputer::new(&program, None);
        c.run();
        assert_eq!([1002, 4, 3, 4, 99].as_slice(), c.curr_prgm);
    }

    #[test]
    pub fn intcode5_test_negative() {
        let program = parse_intcode("1101,100,-1,4,0");
        let mut c = IntCodeComputer::new(&program, None);
        c.run();
        assert_eq!([1101, 100, -1, 4, 99].as_slice(), c.curr_prgm);
    }

    #[test]
    pub fn intcode5_test_output_input() {
        let program = parse_intcode("3,0,4,0,99");
        let mut c = IntCodeComputer::new(&program, None);

        c.input_to_stdin(15);
        c.run();
        assert!(!c.view_stdout().is_empty());
        assert_eq!(15, c.view_stdout()[0]);
    }

    #[test]
    pub fn intcode5_test_equal_position() {
        let program = parse_intcode("3,9,8,9,10,9,4,9,99,-1,8");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, 125, 0);
        test_stdin_stdout_intcode_helper(&mut c, 8, 1);
        test_stdin_stdout_intcode_helper(&mut c, 3, 0);
        test_stdin_stdout_intcode_helper(&mut c, -8, 0);
    }

    #[test]
    pub fn intcode5_test_less_position() {
        let program = parse_intcode("3,9,7,9,10,9,4,9,99,-1,8");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, 15, 0);
        test_stdin_stdout_intcode_helper(&mut c, 8, 0);
        test_stdin_stdout_intcode_helper(&mut c, 1, 1);
        test_stdin_stdout_intcode_helper(&mut c, -8, 1);
    }

    #[test]
    pub fn intcode5_test_equal_immediate() {
        let program = parse_intcode("3,3,1108,-1,8,3,4,3,99");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, 125, 0);
        test_stdin_stdout_intcode_helper(&mut c, 8, 1);
        test_stdin_stdout_intcode_helper(&mut c, 3, 0);
        test_stdin_stdout_intcode_helper(&mut c, -8, 0);
    }

    #[test]
    pub fn intcode5_test_less_immediate() {
        let program = parse_intcode("3,3,1107,-1,8,3,4,3,99");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, 15, 0);
        test_stdin_stdout_intcode_helper(&mut c, 8, 0);
        test_stdin_stdout_intcode_helper(&mut c, 1, 1);
        test_stdin_stdout_intcode_helper(&mut c, -8, 1);
    }

    #[test]
    pub fn intcode5_test_jump_position() {
        let program = parse_intcode("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, 0, 0);
        test_stdin_stdout_intcode_helper(&mut c, 1, 1);
        test_stdin_stdout_intcode_helper(&mut c, 2, 1);
        test_stdin_stdout_intcode_helper(&mut c, -1, 1);
        test_stdin_stdout_intcode_helper(&mut c, -2, 1);
    }

    #[test]
    pub fn intcode5_test_jump_immediate() {
        let program = parse_intcode("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, 0, 0);
        test_stdin_stdout_intcode_helper(&mut c, 1, 1);
        test_stdin_stdout_intcode_helper(&mut c, 2, 1);
        test_stdin_stdout_intcode_helper(&mut c, -1, 1);
        test_stdin_stdout_intcode_helper(&mut c, -2, 1);
    }

    #[test]
    pub fn intcode5_test_complex() {
        let program = parse_intcode("3,21,1008,21,8,20,1005,20,22,107,8,21,20,\
        1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,\
        1,20,4,20,1105,1,46,98,99");
        let mut c = IntCodeComputer::new(&program, None);

        test_stdin_stdout_intcode_helper(&mut c, -5, 999);
        test_stdin_stdout_intcode_helper(&mut c, 6, 999);
        test_stdin_stdout_intcode_helper(&mut c, 0, 999);
        test_stdin_stdout_intcode_helper(&mut c, 8, 1000);
        test_stdin_stdout_intcode_helper(&mut c, 9, 1001);
        test_stdin_stdout_intcode_helper(&mut c, 11, 1001);
        test_stdin_stdout_intcode_helper(&mut c, 99, 1001);
    }

    /// Helps test standard input/output of the Intcode computer.
    ///
    /// # Parameters
    /// - `c`: The computer.
    /// - `input`: The input.
    /// - `expected`: The last expected output from standard output.
    fn test_stdin_stdout_intcode_helper(c: &mut IntCodeComputer, input: isize, expected: isize) {
        c.reset();
        c.input_to_stdin(input);
        c.run();
        assert!(!c.view_stdout().is_empty());
        assert_eq!(expected, *c.view_stdout().last().unwrap());
    }
}
