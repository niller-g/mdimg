<#
Builds the fuzz Docker image and runs it from a clean corpus every time,
capturing any crash to fuzz/artifacts.

Usage:
  .\run.ps1                # 60 second default run
  .\run.ps1 -Seconds 300   # longer run
#>
param(
    [int]$Seconds = 60
)

$fuzzDir = $PSScriptRoot
$repoRoot = Resolve-Path (Join-Path $fuzzDir "..\..\..")
$dockerfile = Join-Path $fuzzDir "Dockerfile"
$artifactsDir = Join-Path $fuzzDir "artifacts"
$image = "mdimg-fuzz"

Write-Host "Building $image ..."
docker build -f $dockerfile -t $image $repoRoot
if ($LASTEXITCODE -ne 0) {
    Write-Error "docker build failed (exit $LASTEXITCODE)"
    exit $LASTEXITCODE
}

Write-Host "Running fuzzer for $Seconds second(s) ..."
docker run --rm -v "${artifactsDir}:/app/crates/mdimg/fuzz/artifacts" $image replace_images -- "-max_total_time=$Seconds"
$exitCode = $LASTEXITCODE

$crashFiles = Get-ChildItem (Join-Path $artifactsDir "replace_images") -Filter "crash-*" -ErrorAction SilentlyContinue

if ($crashFiles) {
    Write-Host ""
    Write-Host "Crash captured:"
    foreach ($f in $crashFiles) {
        Write-Host "  $($f.FullName)"
    }
    Write-Host ""
    Write-Host "To reproduce and see the panic message, run:"
    Write-Host "  docker run --rm -v `"${artifactsDir}:/app/crates/mdimg/fuzz/artifacts`" --entrypoint cargo $image fuzz run replace_images `"/app/crates/mdimg/fuzz/artifacts/replace_images/$($crashFiles[0].Name)`""
} elseif ($exitCode -ne 0) {
    Write-Host "Fuzzer exited with code $exitCode but no crash file was found under $artifactsDir\replace_images"
} else {
    Write-Host "No crash found in $Seconds second(s)."
}
