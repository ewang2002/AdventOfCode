use std::ops::{Index, IndexMut};

pub struct IntCodeComputer {
    /// The original program code.
    original: Vec<usize>,
    /// The current program code.
    curr_prgm: Vec<usize>,
    /// The instruction pointer.
    ins_pointer: usize,
    /// The length of the opcodes.
    len: usize,
}

impl IntCodeComputer {
    /// Creates a new IntCodeComputer with the specified program.
    ///
    /// # Parameters
    /// - `prgm`: The program.
    ///
    /// # Returns
    /// THe new computer..
    pub fn new(prgm: &[usize]) -> Self {
        IntCodeComputer {
            original: Vec::from(prgm),
            curr_prgm: Vec::from(prgm),
            ins_pointer: 0,
            len: prgm.len(),
        }
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
    pub fn view(&self) -> &[usize] {
        &self.curr_prgm
    }

    /// Runs the program.
    pub fn run(&mut self) {
        while self.ins_pointer < self.curr_prgm.len() {
            match self.curr_prgm[self.ins_pointer] {
                1 => self.add(),
                2 => self.multiply(),
                99 => break,
                _ => panic!("Unknown opcode {}", self.curr_prgm[self.ins_pointer]),
            }
        }
    }

    fn add(&mut self) {
        let (param1, param2, apply_to) = self.get_indices();
        self.curr_prgm[apply_to] = self.curr_prgm[param1] + self.curr_prgm[param2];
        self.ins_pointer += 4;
    }

    fn multiply(&mut self) {
        let (param1, param2, apply_to) = self.get_indices();
        self.curr_prgm[apply_to] = self.curr_prgm[param1] * self.curr_prgm[param2];
        self.ins_pointer += 4;
    }

    fn get_indices(&self) -> (usize, usize, usize) {
        (
            self.curr_prgm[self.ins_pointer + 1],
            self.curr_prgm[self.ins_pointer + 2],
            self.curr_prgm[self.ins_pointer + 3],
        )
    }
}

impl Index<usize> for IntCodeComputer {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.curr_prgm[index]
    }
}

impl IndexMut<usize> for IntCodeComputer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.curr_prgm[index]
    }
}
