[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateRange(0, 25)]
    [Alias('d')]
    [int] 
    $day
)

# If the file already exists, don't write to it -- especially since we might have
# made changes to the already existing file.
$new_prob_file_name = [string]::Format("src/aoc/day{0:d2}.rs", $day)
if (Test-Path -Path $new_prob_file_name) {
    Write-Warning ([string]::Format("File 'day{0:d2}.rs' already exists", $day))
    exit 1
}

# =================================================================================== #
#                       Create the problem file                                       #
# =================================================================================== #
$base_problem_str = @'
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day{0:d2} {{
    // fields here
}}

impl AoCProblem<usize, usize> for Day{0:d2} {{
    fn prepare(input: Vec<&str>) -> Self {{
        Self
    }}

    fn part1(&mut self) -> usize {{
        0
    }}

    fn part2(&mut self) -> usize {{
        0
    }}
}}
'@

[string]::Format($base_problem_str, $day) | Out-File -FilePath $new_prob_file_name -Encoding UTF8
if (!$?) {
    Write-Warning ([string]::Format("Failed to write to file 'day{0:d2}.rs'", $day))
    exit 1
}

# =================================================================================== #
#                       Update the mod.rs file                                        #
# =================================================================================== #
$mod_append_str = @'

mod day{0:d2};
pub use day{0:d2}::Day{0:d2};
'@

Add-Content -Path "src/aoc/mod.rs" -Value ([string]::Format($mod_append_str, $day))
if (!$?) {
    Write-Warning ([string]::Format("Failed to append to file 'mod.rs'"))
    exit 1
}

# =================================================================================== #
#                       Create the input files                                        #
# =================================================================================== #
$input_name = [string]::Format("input/day{0:d2}.txt", $day)
if (!(Test-Path -Path $input_name)) {
    New-Item -Path $input_name -ItemType File | Out-Null
    if (!$?) {
        Write-Warning ([string]::Format("Failed to create file 'day{0:d2}.txt'", $day))
        exit 1
    }
}

# =================================================================================== #
#                       Finally, update run.rs                                        #
# =================================================================================== #
$run_enum_base = ""
# Go through each file in the aoc directory
$files = Get-ChildItem -Path "src/aoc" -Filter "*.rs"
foreach ($file in $files) {
    # Does it have a number before the .rs?
    if ($file.Name -match "day(\d{2})\.rs") {
        # Get the number
        $num = [int]::Parse($Matches[1])
        $run_enum_base += [string]::Format("        {0} => Box::new(aoc::Day{0:d2}::prepare(content)),`n", $num)
    }
}

# Finally, apply $run_enum_base to the file base
$run_base = @'
use std::{{fs, path::Path, time::Instant}};

use crate::*;

use crate::aoc::{{self, AoCProblem}};

/// Runs the specified day.
///
/// # Parameters
/// - `day`: The day to run. This should be in the range [0, 25].
///
/// # Returns
/// A result representing whether the execution was successful or not.
pub fn run(day: u32) -> RunResult {{
    // Look for input file.
    let input_file = Path::new("input").join(format!("day{{:02}}.txt", day));
    if !input_file.exists() {{
        return RunResult::InputFileNotFound(input_file);
    }}

    let mut start = Instant::now();
    let input_str = match fs::read_to_string(&input_file) {{
        Ok(o) => o,
        Err(_) => return RunResult::InputFileNotValid(input_file),
    }};

    let content = input_str.lines().collect::<Vec<_>>();

    let mut solver: Box<dyn AoCProblem<_, _>> = match day {{
        {0}
        _ => return RunResult::ProblemNotFound(day),
    }};

    let input_time = start.elapsed();

    // Part 1
    start = Instant::now();
    println!("Part 1 Solution: {}", solver.part1());
    let p1_t = start.elapsed();

    // Part 2
    start = Instant::now();
    println!("Part 2 Solution: {}", solver.part2());
    let p2_t = start.elapsed();

    // Execution ends, display time statistics.
    println!();
    println!("Input Parse : \t{{}} ms.", input_time.as_millis());
    println!("Part 1 Time : \t{{}} ms.", p1_t.as_millis());
    println!("Part 2 Time : \t{{}} ms.", p2_t.as_millis());
    println!();
    println!("P1 + P2     : \t{{}} ms.", (p1_t + p2_t).as_millis(),);
    println!(
        "P + P1 + P2 : \t{{}} ms.",
        (input_time + p1_t + p2_t).as_millis(),
    );

    RunResult::Success
}}
'@

[string]::Format($run_base, $run_enum_base.Trim()) | Out-File -FilePath "src/run.rs" -Encoding UTF8
if (!$?) {
    Write-Warning "Unable to apply changes to 'run.rs'; please do so manually."
    exit 1
}

exit 0