#!/usr/bin/env pwsh

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [Alias('y')]
    [string] 
    $year,

    [Parameter(Mandatory = $true)]
    [ValidateRange(0, 25)]
    [Alias('d')]
    [int] 
    $day
)

$aoc_folder = "aoc" + $year
if (!(Test-Path -Path $aoc_folder)) {
    Write-Warning ([string]::Format("AOC folder '{0}' does not exist.", $aoc_folder))
    exit 1
}

if (!(Test-Path -Path ($aoc_folder + "/Cargo.toml"))) {
    Write-Warning ([string]::Format("AOC folder '{0}' is not a Cargo project.", $aoc_folder))
    exit 1
}

Set-Location $aoc_folder

# If the file already exists, don't write to it -- especially since we might have
# made changes to the already existing file.
$new_prob_file_name = [string]::Format("src/aoc/day{0:d2}.rs", $day)
if (Test-Path -Path $new_prob_file_name) {
    Write-Warning ([string]::Format("File 'day{0:d2}.rs' already exists", $day))
    Set-Location ..
    exit 1
}

# =================================================================================== #
#                       Create the problem file                                       #
# =================================================================================== #
$base_problem_str = @'
use common::problem::day::{{AoCProblem, Solution}};

pub struct Day{0:d2} {{
    // fields here
}}

impl AoCProblem for Day{0:d2} {{
    fn prepare(input: String) -> Self {{
        Self {{}}
    }}

    fn part1(&mut self) -> Solution {{
        0.into()
    }}

    fn part2(&mut self) -> Solution {{
        0.into()
    }}

    fn day() -> u32 {{
        {0}
    }}

    fn year() -> u32 {{
        {1}
    }}
}}
'@

[string]::Format($base_problem_str, $day, $year) | Out-File -FilePath $new_prob_file_name -Encoding UTF8
if (!$?) {
    Write-Warning ([string]::Format("Failed to write to file 'day{0:d2}.rs'", $day))
    Set-Location ..
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
    Set-Location ..
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
        Set-Location ..
        exit 1
    }
}

# =================================================================================== #
#                       Finally, update main.rs                                       #
# =================================================================================== #
$run_enum_base = ""
# Go through each file in the aoc directory
$files = Get-ChildItem -Path "src/aoc" -Filter "*.rs"
foreach ($file in $files) {
    # Does it have a number before the .rs?
    if ($file.Name -match "day(\d{2})\.rs") {
        # Get the number
        $num = [int]::Parse($Matches[1])
        $run_enum_base += [string]::Format("        {0} => run::<crate::aoc::Day{0:d2}>(test_case),`n", $num)
    }
}

# Finally, apply $run_enum_base to the file base
$run_base = @'
use common::problem::run;
use std::env;
mod aoc;

fn main() {{
    let args = env::args().skip(1).take(2).collect::<Vec<_>>();
    if args.is_empty() {{
        println!("Usage: ./aoc{0} <day> [test]");
        println!("\twhere <day> is an integer in [0, 25].");
        println!("\tand [test] is optionally a positive integer.");
        return;
    }}

    let day_to_use = match args[0].parse::<u32>() {{
        Ok(o) if o <= 25 => o,
        _ => {{
            println!("Usage: ./aoc{0} <day> [test]");
            println!("\twhere <day> is an integer in [0, 25].");
            println!("\tand [test] is optionally a positive integer.");
            return;
        }}
    }};

    let test_case = if args.len() == 2 {{
        args[1].parse::<u32>().ok()
    }} else {{
        None
    }};

    match day_to_use {{
        {1}
        _ => {{
            eprintln!("[Error] Day {{day_to_use}} has not been implemented yet.");
        }}
    }}
}}
'@

[string]::Format($run_base, $year, $run_enum_base.Trim()) | Out-File -FilePath "src/main.rs" -Encoding UTF8
if (!$?) {
    Write-Warning "Unable to apply changes to 'main.rs'; please do so manually."
    Set-Location ..
    exit 1
}

Set-Location ..
exit 0