[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [Alias('d')]
    [int] 
    $day
)

$base_name = [string]::Format("day{0:d2}", $day)
$files = Get-ChildItem -Path "input" -Filter "*.txt"
$list_nums = New-Object System.Collections.Generic.List[int]
$found_base_file = $false
foreach ($file in $files) {
    $details = $file.Name.Replace(".txt", "").Split("_")
    if ($details[0] -ne $base_name) {
        continue
    }

    $found_base_file = $true

    if ($details.Length -ne 2) {
        continue
    }
    
    # Recall that our file will look something like
    # day01_test1.txt
    # details[1] will be test1.txt
    # Check if there is ANY integer between 'test'
    if ($details[1] -match "test(\d+)") {
        $test_num = [int]::Parse($Matches[1])
        $list_nums.Add($test_num)
    }
}

if (!$found_base_file) {
    Write-Warning "No base input file for day $day. Run ./create first."
    exit 1
}

# Get the max element in list_nums
$max = 0
foreach ($num in $list_nums) {
    if ($num -gt $max) {
        $max = $num
    }
}

# Create a new file with the name
# base_name_test(max + 1).txt
$new_file_name = [string]::Format("{0}_test{1:d}.txt", $base_name, $max + 1)
New-Item -Path "input/$new_file_name" -ItemType File | Out-Null
if (!$?) {
    Write-Warning ([string]::Format("Failed to create file '{0}'", $new_file_name))
    exit 1
}

Write-Host "Created new test file: $new_file_name"