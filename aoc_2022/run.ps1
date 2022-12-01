# Usage
# - ./run -d <day> -r <0 or 1> -t <int>     e.g., ./run -d 1 -r 0
# - ./run <day> -r <0 or 1> -t <int>        e.g., ./run 1 -r 0
#
# Here,
# - <day> is the day of the problem to run
# - <0 or 1> indicates either debug (0) or release (1) mode. Defaults to debug.
# - <int> is the test case to run. If no test case is provided, this defaults to the actual
#   solution file.
[CmdletBinding(DefaultParameterSetName = "Day")]
param(
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