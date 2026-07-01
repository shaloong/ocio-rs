param(
    [switch]$IncludeBundled,
    [switch]$IncludeTopLevelPackage,
    [switch]$Offline
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

$script:Failures = @()
$script:Warnings = @()

function Invoke-Check {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [string]$WorkingDirectory = $repoRoot,
        [hashtable]$Environment = @{},
        [switch]$AllowKnownTopLevelPackageBlocker
    )

    Write-Host ""
    Write-Host "==> $Name"
    Write-Host "    cargo $($Arguments -join ' ')"

    $stdoutPath = [System.IO.Path]::GetTempFileName()
    $stderrPath = [System.IO.Path]::GetTempFileName()
    try {
        $startProcessParams = @{
            FilePath = "cargo"
            ArgumentList = $Arguments
            WorkingDirectory = $WorkingDirectory
            NoNewWindow = $true
            Wait = $true
            PassThru = $true
            RedirectStandardOutput = $stdoutPath
            RedirectStandardError = $stderrPath
        }
        if ($Environment.Count -gt 0) {
            $startProcessParams["Environment"] = $Environment
        }

        $process = Start-Process @startProcessParams
        $exitCode = $process.ExitCode
        $stdout = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
        $stderr = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }
        $text = @($stdout, $stderr) -join ""
    }
    finally {
        Remove-Item -LiteralPath $stdoutPath -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $stderrPath -ErrorAction SilentlyContinue
    }
    $text = $text.TrimEnd()

    if ($text) {
        Write-Host $text
    }

    if ($exitCode -eq 0) {
        Write-Host "    PASS" -ForegroundColor Green
        return
    }

    if ($AllowKnownTopLevelPackageBlocker) {
        $knownBlocker =
            $text -match 'failed to select a version for the requirement `ocio-sys = "\^0\.2\.0"`' -and
            $text -match 'candidate versions found which didn''t match: 0\.1\.1'
        if ($knownBlocker) {
            $script:Warnings += "$Name blocked by registry state: publish ocio-sys 0.2.0 before packaging ocio-rs."
            Write-Host "    WARN: blocked by registry state, not by repository contents" -ForegroundColor Yellow
            return
        }
    }

    $script:Failures += "$Name failed."
    Write-Host "    FAIL" -ForegroundColor Red
}

function Test-OcioSysBundledPayload {
    Write-Host ""
    Write-Host "==> Package payload (ocio-sys bundled source)"
    Write-Host "    cargo package -p ocio-sys --allow-dirty --list"

    $stdoutPath = [System.IO.Path]::GetTempFileName()
    $stderrPath = [System.IO.Path]::GetTempFileName()
    try {
        $process = Start-Process `
            -FilePath "cargo" `
            -ArgumentList @("package", "-p", "ocio-sys", "--allow-dirty", "--list") `
            -NoNewWindow `
            -Wait `
            -PassThru `
            -RedirectStandardOutput $stdoutPath `
            -RedirectStandardError $stderrPath
        $exitCode = $process.ExitCode
        $stdout = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
        $stderr = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }
        $text = @($stdout, $stderr) -join ""
    }
    finally {
        Remove-Item -LiteralPath $stdoutPath -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $stderrPath -ErrorAction SilentlyContinue
    }

    if ($text.Trim()) {
        Write-Host $text.TrimEnd()
    }

    if ($exitCode -ne 0) {
        $script:Failures += "Package payload (ocio-sys bundled source) failed."
        Write-Host "    FAIL" -ForegroundColor Red
        return
    }

    $hasBundledSource = $text -match 'OpenColorIO[/\\]CMakeLists\.txt' -or $text -match 'third_party[/\\]OpenColorIO'
    if (-not $hasBundledSource) {
        $script:Warnings += "Published ocio-sys package does not currently vendor the OpenColorIO source tree."
        Write-Host "    WARN: bundled source tree not present in packaged ocio-sys payload" -ForegroundColor Yellow
        return
    }

    Write-Host "    PASS" -ForegroundColor Green
}

function Get-OcioSysPackageDir {
    $packageRoots = @()

    if ($script:OcioSysPackageTargetDir) {
        $packageRoots += (Join-Path $script:OcioSysPackageTargetDir "package")
    }

    $packageRoots += (Join-Path $repoRoot "target/package")

    foreach ($packageRoot in $packageRoots) {
        if (-not (Test-Path -LiteralPath $packageRoot)) {
            continue
        }

        $match = Get-ChildItem -LiteralPath $packageRoot -Directory |
            Where-Object { $_.Name -like "ocio-sys-*" } |
            Sort-Object LastWriteTime -Descending |
            Select-Object -First 1 -ExpandProperty FullName

        if ($match) {
            return $match
        }
    }

    return $null
}

function Clear-PackageArtifacts {
    param(
        [Parameter(Mandatory = $true)]
        [string]$CrateName
    )

    $packageRoot = Join-Path $repoRoot "target/package"
    if (-not (Test-Path -LiteralPath $packageRoot)) {
        return
    }

    $resolvedRoot = (Resolve-Path -LiteralPath $packageRoot).Path
    $expectedPrefix = [System.IO.Path]::GetFullPath($resolvedRoot + [System.IO.Path]::DirectorySeparatorChar)

    Get-ChildItem -LiteralPath $packageRoot -Force |
        Where-Object {
            $_.Name -eq "$CrateName.crate" -or
            $_.Name -like "$CrateName-*"
        } |
        ForEach-Object {
            $targetPath = $_.FullName
            $resolvedTarget = [System.IO.Path]::GetFullPath($targetPath)
            if (-not $resolvedTarget.StartsWith($expectedPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
                throw "Refusing to remove package artifact outside target/package: $resolvedTarget"
            }

            Remove-Item -LiteralPath $resolvedTarget -Recurse -Force -ErrorAction SilentlyContinue
        }
}

function New-PackageTargetDir {
    param(
        [Parameter(Mandatory = $true)]
        [string]$CrateName
    )

    $auditRoot = Join-Path $repoRoot "t\ra"
    Join-Path $auditRoot $CrateName
}

function Ensure-StandalonePackageManifest {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PackageDir
    )

    $manifestPath = Join-Path $PackageDir "Cargo.toml"
    if (-not (Test-Path -LiteralPath $manifestPath)) {
        return
    }

    $manifest = Get-Content -LiteralPath $manifestPath -Raw
    if ($manifest -match '(?m)^\[workspace\]\s*$') {
        return
    }

    Add-Content -LiteralPath $manifestPath -Value "`r`n[workspace]`r`n"
}

Invoke-Check -Name "Format" -Arguments @("fmt", "--all", "--", "--check")
Invoke-Check -Name "Clippy" -Arguments @("clippy", "--workspace", "--all-targets", "--no-default-features", "--", "-D", "warnings")
Invoke-Check -Name "Tests (stub)" -Arguments @("test", "--workspace", "--no-default-features")
Invoke-Check -Name "Examples (stub)" -Arguments @("test", "--examples", "--no-default-features")
Invoke-Check `
    -Name "Docs (stub)" `
    -Arguments @("doc", "--workspace", "--no-deps", "--no-default-features") `
    -Environment @{ RUSTDOCFLAGS = "-D warnings" }
Invoke-Check -Name "Parity" -Arguments @("run", "--bin", "check_parity", "--quiet")

Clear-PackageArtifacts -CrateName "ocio-sys"
$ocioSysPackageArgs = @("package", "-p", "ocio-sys", "--allow-dirty")
if ($Offline) {
    $ocioSysPackageArgs += "--offline"
}
$ocioSysPackageTargetDir = New-PackageTargetDir -CrateName "ocio-sys"
$script:OcioSysPackageTargetDir = $ocioSysPackageTargetDir
if (Test-Path -LiteralPath $ocioSysPackageTargetDir) {
    Remove-Item -LiteralPath $ocioSysPackageTargetDir -Recurse -Force -ErrorAction SilentlyContinue
}
Invoke-Check `
    -Name "Package ocio-sys" `
    -Arguments $ocioSysPackageArgs `
    -Environment @{ CARGO_TARGET_DIR = $ocioSysPackageTargetDir }
Test-OcioSysBundledPayload

$ocioSysPackageDir = Get-OcioSysPackageDir
if ($ocioSysPackageDir) {
    Ensure-StandalonePackageManifest -PackageDir $ocioSysPackageDir
    $ocioSysBundledBuildArgs = @("build", "--features", "bundled")
    if ($Offline) {
        $ocioSysBundledBuildArgs += "--offline"
    }
    Invoke-Check `
        -Name "Packaged bundled build (ocio-sys)" `
        -Arguments $ocioSysBundledBuildArgs `
        -WorkingDirectory $ocioSysPackageDir
} else {
    $script:Failures += "Packaged bundled build (ocio-sys) could not locate extracted package directory."
    Write-Host ""
    Write-Host "==> Packaged bundled build (ocio-sys)"
    Write-Host "    FAIL" -ForegroundColor Red
}

if ($IncludeBundled) {
    Invoke-Check -Name "Tests (bundled)" -Arguments @("test", "--workspace", "--features", "bundled")
}

if ($IncludeTopLevelPackage) {
    Clear-PackageArtifacts -CrateName "ocio-rs"
    $topLevelPackageArgs = @("package", "--allow-dirty")
    if ($Offline) {
        $topLevelPackageArgs += "--offline"
    }
    $ocioRsPackageTargetDir = New-PackageTargetDir -CrateName "ocio-rs"
    if (Test-Path -LiteralPath $ocioRsPackageTargetDir) {
        Remove-Item -LiteralPath $ocioRsPackageTargetDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    Invoke-Check `
        -Name "Package ocio-rs" `
        -Arguments $topLevelPackageArgs `
        -Environment @{ CARGO_TARGET_DIR = $ocioRsPackageTargetDir } `
        -AllowKnownTopLevelPackageBlocker
}

Write-Host ""
Write-Host "==> Release audit summary"

if ($script:Warnings.Count -gt 0) {
    Write-Host "Warnings:" -ForegroundColor Yellow
    foreach ($warning in $script:Warnings) {
        Write-Host "  - $warning"
    }
}

if ($script:Failures.Count -gt 0) {
    Write-Host "Failures:" -ForegroundColor Red
    foreach ($failure in $script:Failures) {
        Write-Host "  - $failure"
    }
    exit 1
}

if ($script:Warnings.Count -gt 0) {
    Write-Host "Result: repository checks passed; external publish-order state still pending." -ForegroundColor Yellow
    exit 0
}

Write-Host "Result: all selected release checks passed." -ForegroundColor Green
