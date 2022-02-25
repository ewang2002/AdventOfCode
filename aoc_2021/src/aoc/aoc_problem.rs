pub trait AoCProblem<Part1, Part2> {
    /// Sets up an `AoCProblem` structure. This will parse the `input` vector so that it can be
    /// used for both parts of the problem. The parsed inputs should be made available as data
    /// members in the structure for easy access.
    ///
    /// # Parameters
    /// - `input`: The input.
    ///
    /// # Returns
    /// - The `AoCProblem`.
    fn prepare(input: Vec<String>) -> Self;

    /// Solves part 1 of the day's Advent of Code problem.
    ///
    /// # Returns
    /// - The solution to part 1.
    fn part1(&self) -> Part1;

    /// Solves part 2 of the day's Advent of Code problem.
    ///
    /// # Returns
    /// - The solution to part 2.
    fn part2(&self) -> Part2;
}
