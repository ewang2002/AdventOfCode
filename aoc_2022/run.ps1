# Usage
# - ./run -d <day> -r <0 or 1>      e.g., ./run -d 1 -r 0
# - ./run <day> -r <0 or 1>         e.g., ./run 1 -r 0
#
# Here,
# - <day> is the day of the problem to run
# - <0 or 1> indicates either debug (0) or release (1) mode. Defaults to debug.
[CmdletBinding(DefaultParameterSetName = "Day")]
param(
    [Parameter(Mandatory, ParameterSetName = "Day", Position = 0)]
    [ValidateRange(0, 25)]
    [Alias('d')]
    [int] 
    $day,

    [Parameter()]
    [Alias('r')]
    [bool] 
    $release = $false
)

if ($release) {
    cargo -q run -r -- $day
}
else {
    cargo -q run -- $day
}