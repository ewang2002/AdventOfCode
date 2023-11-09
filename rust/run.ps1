# Usage
# - ./run -y <year> -d <day> -r <0 or 1> -t <int>     e.g., ./run -y 2022 -d 1 -r 0
#
# Here,
# - <day> is the day of the problem to run
# - <0 or 1> indicates either debug (0) or release (1) mode. Defaults to debug.
# - <int> is the test case to run. If no test case is provided, this defaults to the actual
#   solution file.
[CmdletBinding(DefaultParameterSetName = "Day")]
param(
    [Parameter(Mandatory = $true)]
    [Alias('y')]
    [string] 
    $year,

    [Parameter(Mandatory, ParameterSetName = "Day", Position = 0)]
    [ValidateRange(0, 25)]
    [Alias('d')]
    [int] 
    $day,

    [Parameter()]
    [Alias('t')]
    [int] 
    $test = 0,

    [Parameter()]
    [Alias('r')]
    [bool] 
    $release = $false
)

$aoc_folder = "aoc_" + $year
if (!(Test-Path -Path $aoc_folder)) {
    Write-Warning ([string]::Format("AOC folder '{0}' does not exist.", $aoc_folder))
    exit 1
}

if (!(Test-Path -Path ($aoc_folder + "/Cargo.toml"))) {
    Write-Warning ([string]::Format("AOC folder '{0}' is not a Cargo project.", $aoc_folder))
    exit 1
}

Set-Location $aoc_folder

try {
    if ($release) {
        if ($test -eq 0) {
            cargo -q run -r -- $day
        }
        else {
            cargo -q run -r -- $day $test
        }
    }
    else {
        if ($test -eq 0) {
            cargo -q run -- $day
        }
        else {
            cargo -q run -- $day $test
        }
    }
}
finally {
    Set-Location ..
}