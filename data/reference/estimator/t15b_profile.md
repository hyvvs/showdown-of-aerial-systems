# T15b: test-profile evidence (2026-11-19)

## Scope

Measured on branch task/t15b, source revision 6121ae7. This follow-up changes
documentation only: no production/test source, profile, assertion, tolerance,
dependency, CI selection or mesh is changed. Scratch probes are outside the
build/CI in ignored target/t15b-profile. All wing and operating settings are
SYNTHETIC TEST VALUES, not aircraft data.

## Exact existing profile change

The owner approved the following change previously committed in 5ca8c74:

```diff
diff --git a/Cargo.toml b/Cargo.toml
index f603006..3ae69bf 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -1,3 +1,8 @@
 [workspace]
 members = ["crates/soas-core", "crates/soas-cli", "crates/soas-estimator"]
 resolver = "3"
+
+# Owner-approved T15b test-only optimization for the largest dense VLM cases.
+# No model, assertion, tolerance or dependency changes.
+[profile.test.package.soas-estimator]
+opt-level = 3
```

This is a per-package test-profile override, not a release-profile switch.
The [Cargo profile documentation](https://doc.rust-lang.org/cargo/reference/profiles.html)
specifies that test inherits dev; debug assertions and integer-overflow checks
remain enabled. Verbose compiler invocations corroborate the effective options:

```text
old: rustc.exe --crate-name soas_estimator --edition=2024 crates\soas-estimator\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2
new: rustc.exe --crate-name soas_estimator --edition=2024 crates\soas-estimator\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on
```

The old invocation uses default opt-level 0 and default checks; the new one
explicitly sets opt-level 3 and debug-assertions=on. Neither disables overflow
checks. Both probes print cfg(debug_assertions)=true, and both caught runtime
probes panic for an intentional debug_assert(false) and u64::MAX+1. Catching
these expected panics lets the informational diagnostic finish. The original
panic messages remain in scratch stderr; they are not gate failures.

## Identical 256 by 8 production-solver case, old/new profiles

Both scratch workspaces contain the current crates and Cargo.lock copied
unchanged. Only the root manifest differs: the old one is from 4c85025, the new
one is the current manifest. The identical integration probe calls production
vlm::solve with a rectangular wing: span20 m, chord1 m, AR20, alpha .05 rad,
Ny256/Nx8, full-span cosine spacing, no offsets/twist/zero-lift angle, reference
x=0. Section slope is the cited ideal 2*pi per rad helper. Dependency versions
are locked and no dependency installation was requested.

Numbers below are printed to 17 digits after the decimal in scientific notation;
hex columns are exact f64 bit patterns (old/new). Relative change divides the
absolute difference by the absolute old value. Tiny relative-to-zero measures
such as roll and residual are not physical accuracy claims.

| Output | Old | New | New minus old | Absolute relative change | Bits old / new |
| --- | --- | --- | --- | --- | --- |
| CL | 2.72009199786332090e-1 | 2.72009199786332145e-1 | +5.55111512312578270e-17 | 2.040782123e-16 | `3fd168994652c780` / `3fd168994652c781` |
| CDi | 1.29723858170230398e-3 | 1.29723858170230355e-3 | -4.33680868994201774e-19 | 3.343108007e-16 | `3f55410352253482` / `3f55410352253480` |
| e | 9.07752514413199485e-1 | 9.07752514413200151e-1 | +6.66133814775093924e-16 | 7.338275622e-16 | `3fed0c4f004886d8` / `3fed0c4f004886de` |
| Cm | -6.72480443791969168e-2 | -6.72480443791969168e-2 | +0.00000000000000000e+00 | 0.000000000e+00 | `bfb1372af7541f9d` / `bfb1372af7541f9d` |
| Cl_roll | 5.99667785266671219e-17 | 5.73112931793334504e-17 | -2.65548534733367147e-18 | 4.428260801e-02 | `3c9148c46b851eb8` / `3c9084d3b851eb85` |
| CL_alpha | 5.44018399572663913e0 | 5.44018399572663647e0 | -2.66453525910037570e-15 | 4.897877096e-16 | `4015c2bf97e7795d` / `4015c2bf97e7795a` |
| Cm_alpha | -1.34496088758393628e0 | -1.34496088758393606e0 | +2.22044604925031308e-16 | 1.650937265e-16 | `bff584f5b529277b` / `bff584f5b529277a` |
| neutral_x_m | -2.47227095377734823e-1 | -2.47227095377734879e-1 | -5.55111512312578270e-17 | 2.245350622e-16 | `bfcfa52330aa8d85` / `bfcfa52330aa8d87` |
| solve_residual | 2.86844064519942690e-17 | 2.95106675871089988e-17 | +8.26261135114729743e-19 | 2.880523732e-02 | `3c8089123ca41e84` / `3c81030180003f3d` |

The outputs are **not bit-identical**. The nonzero lift/drag/efficiency and
pitch quantities differ at floating-point roundoff scale; roll is close to
zero in both. This observation is not proof of equality for other cases, nor
does it change any acceptance tolerance.

## Resource measurement and stop rule

The owner confirmed a four-minute (240 s) wall-time limit for the unchanged
largest existing test function, which contains both 8192-panel cases plus its
coarser cases. The memory guard is 2.0 GB = 2,000,000,000 bytes, applied to the
larger of process peak resident working set and peak paged/commit counter.
No reserved virtual-address total is used as resident memory.

Measurement uses a monotonic Stopwatch and 100 ms polling of .NET Process
high-water counters. See Microsoft's definitions of
[PeakWorkingSet64](https://learn.microsoft.com/en-us/dotnet/api/system.diagnostics.process.peakworkingset64)
and [PeakPagedMemorySize64](https://learn.microsoft.com/en-us/dotnet/api/system.diagnostics.process.peakpagedmemorysize64).
The private-bytes column is a sampled maximum. OS peak counters cover earlier
peaks, but the final unsampled interval before process exit is a measurement
limitation. The monitor kills only its own child tree if a guard trips; it
never skips or changes tests.

| Run | Wall s | Peak resident bytes | Peak paged/commit bytes | Sampled private bytes | Exit | Stopped |
| --- | --- | --- | --- | --- | --- | --- |
| old-probe | 47.3175938 | 73965568 | 68608000 | 68608000 | 0 | None |
| new-probe | 0.9426695 | 73719808 | 68624384 | 68624384 | 0 | None |
| largest | 179.9444381 | 1081479168 | 1078722560 | 1078722560 | 0 | None |
| full-suite | 194.9422389 | 30416896 | 41099264 | 3813376 | 0 | None |

The two probe wall times exclude compilation and include process startup.
The largest test is selected by exact function name in the current test binary,
built by cargo test -p soas-estimator --test vlm_acceptance --no-run. No source
or test content is altered. Full cargo test --all is separately timed as a
whole, with the existing warm build cache, including Cargo startup/checks,
all integration binaries and doc tests. Its memory counters belong to the
Cargo parent only and are **not** a suite-wide or descendant peak claim.

### Largest-test output

```text

running 1 test
T15b ellipse AR=6 Ny=128 Nx=1 e=1.008869464747230 slope=4.394298273783361
T15b ellipse AR=6 Ny=256 Nx=1 e=1.003809879901171 slope=4.384661653400875
T15b ellipse AR=6 Ny=512 Nx=1 e=1.001286657966816 slope=4.379805673993876
T15b Munk AR=6 Nx=1 last Ny=512 estimate=0.998776508925988 p=1.003751905734
T15b ellipse AR=6 Ny=1024 Nx=1 e=1.000026651086844 slope=4.377368330247043
T15b Munk AR=6 Nx=1 last Ny=1024 estimate=0.998769844233513 p=1.001835495391
T15b ellipse AR=40 Ny=128 Nx=1 e=1.009674979568375 slope=5.965304147716014
T15b ellipse AR=40 Ny=256 Nx=1 e=1.004771212490004 slope=5.962810328124676
T15b ellipse AR=40 Ny=512 Nx=1 e=1.002334799376086 slope=5.961505436333992
T15b Munk AR=40 Nx=1 last Ny=512 estimate=0.999928939111532 p=1.009131679975
T15b ellipse AR=40 Ny=1024 Nx=1 e=1.001120923785017 slope=5.960838582540237
T15b Munk AR=40 Nx=1 last Ny=1024 estimate=0.999915648754146 p=1.005138206061
T15b ellipse AR=6 Ny=128 Nx=8 e=1.008096948615462 slope=4.410074080428464
T15b ellipse AR=6 Ny=256 Nx=8 e=1.003027326094736 slope=4.401591665620781
T15b ellipse AR=6 Ny=512 Nx=8 e=1.000499703818527 slope=4.397215969539353
T15b Munk AR=6 Nx=8 last Ny=512 estimate=0.997986378186488 p=1.004097443920
test elliptic_richardson_estimates_obey_munk_bound_without_tolerance has been running for over 60 seconds
T15b ellipse AR=6 Ny=1024 Nx=8 e=0.999237091555117 slope=4.394992310112553
T15b Munk AR=6 Nx=8 last Ny=1024 estimate=0.997976872496307 p=1.001369216818
T15b ellipse AR=40 Ny=128 Nx=8 e=1.009633066699138 slope=5.965631252880777
T15b ellipse AR=40 Ny=256 Nx=8 e=1.004735142808941 slope=5.963430841602480
T15b ellipse AR=40 Ny=512 Nx=8 e=1.002300858305939 slope=5.962310113315382
T15b Munk AR=40 Nx=8 last Ny=512 estimate=0.999895578916286 p=1.008672565236
T15b ellipse AR=40 Ny=1024 Nx=8 e=1.001088737755023 slope=5.961741674477645
T15b Munk AR=40 Nx=8 last Ny=1024 estimate=0.999886578071417 p=1.005964601971
test elliptic_richardson_estimates_obey_munk_bound_without_tolerance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 179.92s
```

### Full-suite summaries

```text
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.81s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.59s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.19s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.21s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 182.99s
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Sum of printed summaries (including zero-test/doc-test binaries): (177, 0, 0).
Columns in this tuple are passed, failed, ignored. Full stdout/stderr and JSON
measurements remain in ignored target/t15b-profile for local audit.

## Whole-suite quality gates and ignore audit

All three local gates passed: formatting exit 0, Clippy exit 0, and the full
test gate exit 0, with 177 passed, 0 failed and 0 ignored. Repository CI
.github/workflows/ci.yml runs the
same unfiltered gates: cargo fmt --all -- --check; cargo clippy --all-targets
--all-features -- -D warnings; cargo test --all. Formatting checks all workspace
source, Clippy checks all targets/features, and cargo test --all runs the whole
workspace test suite. These commands contain no test-name filter or skip flag.
The exact-function command above is an additional resource measurement only,
not a substitute for the full test gate. No #[ignore] or cfg_attr(ignore)
attribute was found in crate source/tests by rg; the full summaries must also
confirm zero ignored tests. No remote CI run is claimed.

## Numerical and settings ledger

| Setting/result | Origin and purpose |
| --- | --- |
| 4c85025, 5ca8c74, 6121ae7 | Existing Git revisions: old manifest, approved profile commit, measured source. |
| opt-level 0/3, debuginfo 2, checks=true | Cargo default versus existing owner-approved override; compiler log/runtime probes, not physics settings. |
| span20 m, chord1 m, AR20; y0/10 m; alpha .05 rad; Ny256, Nx8; zero offsets, twist, zero-lift angle and reference | Existing synthetic rectangular efficiency-study case; ytip=span/2 and AR=span/chord. |
| 2*pi per rad | Existing helper, ideal incompressible thin-airfoil theory, cited in data/reference/estimator/geometry.md. |
| u64::MAX (18446744073709551615), +1, false | Exact language integer boundary/boolean for caught overflow/assertion checks, not model inputs. |
| 240 s, 4 minutes, 2,000,000,000 bytes, 2.0 GB | Owner-confirmed wall-time limit and disclosed memory guard. |
| 100 ms | Measurement polling interval; OS high-water counters plus sampled private bytes; not an accuracy tolerance. |
| 17 decimal places; 16 hex digits | Round-trip-safe diagnostic print precision and f64 bit encoding; no acceptance threshold. |
| AR6/40, Nx1/8, Ny128/256/512/1024; 8192 panels | Existing largest test inputs, already ledgered in tests/fixtures/vlm_acceptance_test_values.md; count=1024*8. |
| All printed values, bit patterns, changes, resource bytes/times, totals above | Measured probe/unchanged-test outputs; changes computed as new-old and abs(change/old); not invented constants. |
| rustc1.94.0, Cargo1.94.0, LLVM21.1.8, x86_64-pc-windows-msvc | Measured local toolchain; no environment update performed. |

## Reproduction

Set TEMP and TMP to an existing ignored directory inside this checkout before
any Cargo test run. CARGO_INCREMENTAL=0 retains the previously disclosed
incremental-filesystem setting; it is not introduced silently here. Capture
executables from Cargo's compiler-artifact JSON, not by selecting old .exe
names. Verbose build logs may include build-script text: parse only JSON
records when locating the executable.

```powershell
$env:TEMP=(Resolve-Path target/t15b-profile/temp).Path
$env:TMP=$env:TEMP
$env:CARGO_INCREMENTAL='0'
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

### Identical diagnostic probe (not part of build/CI)

```rust
// DOCS-ONLY diagnostic in scratch copies. All case settings are SYNTHETIC.
// Same rectangle as the independent efficiency study: AR20, Ny256, Nx8.
use soas_core::units::Radians;
use soas_estimator::{planform::{WingPlanform,WingStation,thin_airfoil_slope_per_rad},
 vlm::{solve,VlmRequest,SpanwiseSpacing}};
#[test]
fn profile_probe() {
 println!("PROFILE debug_assertions_cfg={}",cfg!(debug_assertions));
 let da=std::panic::catch_unwind(||debug_assert!(std::hint::black_box(false),"intentional caught profile probe"));
 let ov=std::panic::catch_unwind(||std::hint::black_box(u64::MAX)+std::hint::black_box(1_u64));
 println!("PROFILE debug_assertion_panicked={} integer_overflow_panicked={}",da.is_err(),ov.is_err());
 let w=WingPlanform {data_note:"SYNTHETIC profile comparison, not real wing".into(),stations:[0.0,10.0].into_iter().map(|y|WingStation {
  y_m:y,chord_m:1.0,leading_edge_x_m:0.0,z_m:0.0,twist_rad:Radians::new(0.0).unwrap(),
  section_lift_curve_slope_per_rad:thin_airfoil_slope_per_rad(),zero_lift_angle_rad:Radians::new(0.0).unwrap(),
  section_source:"Cited ideal thin-airfoil slope; synthetic diagnostic".into()}).collect()};
 let g=w.geometry().unwrap();
 let out=solve(&w,VlmRequest{spanwise_panels:256,chordwise_panels:8,spanwise_spacing:SpanwiseSpacing::Cosine,
  alpha_rad:Radians::new(0.05).unwrap(),reference_x_body_m:0.0}).unwrap();
 for (name,value) in [
  ("CL",out.cl),("CDi",out.induced_drag_coefficient),
  ("e",out.cl.powi(2)/(std::f64::consts::PI*g.aspect_ratio*out.induced_drag_coefficient)),
  ("Cm",out.pitching_moment_coefficient),("Cl_roll",out.rolling_moment_coefficient),
  ("CL_alpha",out.d_cl_d_alpha_per_rad),("Cm_alpha",out.d_cm_d_alpha_per_rad),
  ("neutral_x_m",out.neutral_point_x_body_m),("solve_residual",out.relative_collocation_residual),
 ] {println!("OUTPUT,{name},{value:.17e},{:016x}",value.to_bits());}
}
```

### Resource monitor (not part of build/CI)

Configuration supplies the executable, argument list, working directory, output
paths, wall_limit_s=240, memory_limit_bytes=2000000000 and poll_ms=100. The
full-suite run uses null limits (no artificial interruption) and the same
monitor only for whole-command wall time; its parent-process memory counters
are not the descendant test binaries' peak memory.

```powershell
param([Parameter(Mandatory=$true)][string]$ConfigPath)
$ErrorActionPreference='Stop'
$c=Get-Content -Raw -LiteralPath $ConfigPath | ConvertFrom-Json
$info=[System.Diagnostics.ProcessStartInfo]::new()
$info.FileName=$c.executable
$info.WorkingDirectory=$c.workdir
$info.UseShellExecute=$false
$info.CreateNoWindow=$true
$info.RedirectStandardOutput=$true
$info.RedirectStandardError=$true
foreach ($argument in $c.arguments) { $info.ArgumentList.Add([string]$argument) }
$p=[System.Diagnostics.Process]::new()
$p.StartInfo=$info
$timer=[System.Diagnostics.Stopwatch]::StartNew()
$null=$p.Start()
$stdoutTask=$p.StandardOutput.ReadToEndAsync()
$stderrTask=$p.StandardError.ReadToEndAsync()
$peakWorking=0L
$peakCommit=0L
$observedPrivate=0L
$stopped=$null
while ($true) {
    if (-not $p.HasExited) {
        $p.Refresh()
        try {
            $peakWorking=[Math]::Max($peakWorking,$p.PeakWorkingSet64)
            $peakCommit=[Math]::Max($peakCommit,$p.PeakPagedMemorySize64)
            $observedPrivate=[Math]::Max($observedPrivate,$p.PrivateMemorySize64)
        } catch [System.InvalidOperationException] { }
        if ($null -ne $c.wall_limit_s -and $timer.Elapsed.TotalSeconds -gt $c.wall_limit_s) { $stopped='wall-time guard' }
        if ($null -ne $c.memory_limit_bytes -and [Math]::Max($peakWorking,$peakCommit) -gt $c.memory_limit_bytes) { $stopped='memory guard' }
        if ($null -ne $stopped) { $p.Kill($true); break }
    }
    if ($p.WaitForExit([int]$c.poll_ms)) { break }
}
$p.WaitForExit()
$timer.Stop()
$utf8=[System.Text.UTF8Encoding]::new($false)
[System.IO.File]::WriteAllText($c.stdout,$stdoutTask.GetAwaiter().GetResult(),$utf8)
[System.IO.File]::WriteAllText($c.stderr,$stderrTask.GetAwaiter().GetResult(),$utf8)
$result=[ordered]@{
    label=$c.label
    executable=$c.executable
    arguments=$c.arguments
    wall_s=$timer.Elapsed.TotalSeconds
    peak_working_set_bytes=$peakWorking
    peak_paged_commit_bytes=$peakCommit
    maximum_observed_private_bytes=$observedPrivate
    poll_ms=$c.poll_ms
    wall_limit_s=$c.wall_limit_s
    memory_limit_bytes=$c.memory_limit_bytes
    stopped=$stopped
    exit_code=$p.ExitCode
}
[System.IO.File]::WriteAllText($c.result,($result | ConvertTo-Json -Depth 5),$utf8)
$result | ConvertTo-Json -Depth 5
if ($null -ne $stopped) { exit 1 }
exit $p.ExitCode
```
