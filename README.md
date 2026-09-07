# Rbox C2Rust port

The objective is one Rust multicall executable targeting the 554 command names
in the existing Rbox inventory. GNU-equivalent commands come first. Programs
without a confirmed GNU provider are deferred. SELinux and systemd remain
excluded by operator direction.

This is a separate project in `/root/rboxc`. The original `/root/rbox` and its
uncommitted work are preserved. The previous safe-no_std-first objective has
been superseded by the user's 2026-09-06 instruction. The external goal tracker
still holds that old objective paused: its API rejected replacement of an
unfinished goal and exposes no cancellation operation. This document records
the operative new objective without falsely declaring the old work complete.

## Acceptance criteria

1. Translate pinned authoritative GNU sources with pinned C2Rust.
2. Compile and link command implementations into one executable, supporting
   both `rboxc COMMAND ...` and command-name symlinks.
3. Compare behavior with the matching GNU executables and run their original
   test suites. Preserve raw results and account for every failure and skip.
4. Run Valgrind on applicable command behavior and failure paths. Record actual
   memory/descriptor findings, including retained runtime allocations.
5. Commit source-sized increments. Keep translated, compiling, tested, and
   complete states separate; an inventory entry is not a completion claim.

Rust `std`, unsafe Rust, libc, and GNU C helper libraries are permitted for this
behavior-first phase. Safe ownership conversion and no_std are later work.
An external GNU executable used as an oracle is never counted as a port.

## Initial source profile

- GNU Coreutils 9.11 is the first provider. Its own single-binary build rules
  supply the command-specific entry names and symbol namespace.
- C2Rust revision `e1e5bf257863107c54e9f42b345c2aeccd925458` is pinned. The
  released 0.22.1 failed on generic selections in the current GNU/glibc headers;
  the pinned revision translated the trial successfully.
- The initial platform is Linux x86-64 with glibc 2.43 and Clang 21.1.8.
  C2Rust is built with Rust 1.93.0; the generated executable uses pinned
  `nightly-2026-01-22` for C variadic definitions.
- GNU's compiler configuration is retained for the C oracle/helper build.
  Translation uses the documented GNU17 parser adaptation where needed.

`inventory/applets.json` records all 553 observed existing names, their proposed
GNU providers, and their current state. Rbox's `APPLET_COUNT` constant says 554,
but its actual dispatcher and full binary list agree on 553 distinct commands;
the one-name discrepancy remains recorded in `inventory/sources.json`.
GNU Hello, Time, Which, Diffutils, and Grep are also source-pinned. Remaining
provider assignments require source/version confirmation before implementation.

GNU Hello 2.12.3 is pinned from the [official GNU release archive](https://ftp.gnu.org/gnu/hello/).
Its signature verifies against the GNU-published keyring; archive, signature,
keyring, signing-key fingerprint, and entry-source hashes are recorded in
`inventory/sources.json`. `scripts/prepare-hello.sh` builds its native oracle
and captures compilation commands. Hello is now an active Rust applet. All seven
original scripts pass natively and under Valgrind: the ambient run records the
long-greeting calendar skip, and a separate declared fixed-calendar input
exercises that unchanged original script. All 17 focused checks pass, including
path-based diagnostics, invalid UTF-8, and output errors. Its helpers retain
177 private symbol names, with eight imports from Rust and no native C main.
The port releases greeting storage before conversion-error diagnostics and uses
GNU's public error callback to preserve Hello's pathname prefix. Initial
observations remain recorded separately. `evidence/hello-original.json` and
`evidence/hello-behavior.json` establish completion for the recorded Linux
x86-64/glibc profile, including the calendar fixture; they do not certify other
platforms. Inventory refresh preserves existing progress and additional pins.

GNU Time 1.10 is pinned from the official signed GNU archive. Its entry is
translated in the installed 109-command release, with 38 namespaced native
helper symbols and no native C entry. Nine reviewed originals pass natively;
eight pass their assertions under Valgrind with clean Time-process exits.
The max-RSS script fails its instrumented delta assertion in both builds because
children inherit Valgrind's memory footprint. One original historical format
reproduction remains excluded and unexecuted. All 23 focused comparisons pass.
The port closes its output stream on normal exit, fatal output errors, and
failed child exec while preserving GNU's status and diagnostics. Header-only
fork logs remain explicitly unassessed exec boundaries. These results do not
certify Time complete. The exact candidate also passes all Coreutils baseline
checks and the Hello original/focused comparisons. `evidence/time-activation.json`
records activation with previous artifacts retained.

GNU Which 2.25 is source-pinned with its verified GNU release signature. The
installed 110-command release translates its command entry and keeps 23
provider symbol names private, including five Rust definitions referenced by
its helpers. The port frees alias/function records and the final explicit-path
buffer after lookup. All 46 native/Valgrind fixtures match GNU, with
zero live candidate heap in those fixtures; GNU's retained allocations remain
in the report. The distribution registers no runtime test suite, so this is
focused compatibility evidence rather than a full original-suite pass. The
combined Coreutils, Hello, and Time regressions pass with Time's existing
instrumentation gap retained. Activation and prior artifact backups are recorded
in `evidence/which-activation.json`.

GNU Diffutils 3.12 is pinned from its verified signed archive. All four command
entries (`cmp`, `diff`, `diff3`, and `sdiff`) are active in the installed
117-command executable. Its native helpers and Rust-owned diff state share 369
private symbols. The translation retains C23 null-pointer semantics through a
GNU17 parser adaptation and preserves provider pathname diagnostics. All 33
registered original scripts are inventoried. Of 30 reviewed originals, the
native GNU and candidate runs each have 28 passes, one prerequisite skip, and
one upstream expected failure. Three originals remain excluded from this profile.
All 56 ordinary native/Valgrind formatting, comparison, merge, allocation-growth,
and I/O-error fixtures pass after ownership cleanup. Cleanup releases cmp inputs,
replaced diff regex programs and directory descriptors, and diff3 allocations,
merge input, and child pipes. The final original Valgrind run has 27 passes,
one prerequisite skip, one matching expected failure, and cmp's timing assertion
open in both builds. A separate measurement confirms both builds pass natively
within 0.4 seconds and under Valgrind with five seconds. All 349 candidate logs
from the final original and focused runs have complete, clean final images.
The combined Coreutils, Hello, Time, and Which regressions pass within their
recorded scopes. `evidence/diffutils-activation.json` records integration, prior
artifact backups, and the installed-path dispatcher check. Earlier failures
and candidate binaries are retained; full-provider completion remains open.

GNU Grep 3.12 is pinned from its signed official archive, and its native oracle
is built with PCRE2 10.46 support. The original registration contains 128 tests,
recorded for individual review before execution. Grep has one C command entry;
`egrep` and `fgrep` are shell aliases in the GNU source. Their warnings and option
insertion now dispatch internally to the translated grep entry in the installed
117-command executable. The matcher helpers and Rust-owned state use 338 private
symbols. All 68 focused native/Valgrind comparisons pass, including basic,
extended, fixed, and PCRE matching; both aliases; quiet recursive searches;
compiler errors; long lines; and output errors. Cleanup releases completed
and partially compiled regex/PCRE objects, case-folding tables, color storage,
input buffers, pattern-file streams, and directory traversals. It runs before GNU's final stdout
check, which can exit without invoking later callbacks. Native matcher
destructors retain the original matching logic and track allocation bases
separately from interior pointers. Input buffers are initialized without
changing logical bounds; this closes a PCRE JIT Valgrind finding also observed
in the GNU oracle. All 94 reviewed original selections pass natively and under
Valgrind: 93 complete scripts and 11 selected diagnostics from one Perl script.
The strict audit verifies 3,823 complete, clean candidate process logs. This
includes the byte/locale matrices, PCRE originals, and 100,000-entry traversal.
The driver uses pinned GNU timeout/sleep, nine private locales, and explicit
PCRE configuration. Earlier prerequisite skips and failing candidates are
preserved. Twenty-two originals remain excluded, twelve remain pending, and
the Perl stress case is unexecuted, so full-provider completion is still open.
A bounded 8.7 MB cached-input observation found after/before median runtime
ratios of 0.83–1.01 across four matchers; concurrent tests and process startup
limit precision, so this is not a performance certification. Activation,
prior artifact backups, and installed-path checks are recorded in
`evidence/grep-activation.json`.

GNU Gzip 1.14 is also pinned from its verified signed archive. Its native
oracle and compiler records are prepared for the `gzip`, `gunzip`, `uncompress`,
and `zcat` inventory entries. The latter entries use upstream shell adapters
and an installed alias; their compatibility work is separate from Grep.
The 30 original Gzip registrations remain inventoried for individual review.
A separate 121-command candidate now compiles with the translated C entry,
three internal shell-alias adaptations, and 170 private helper/state symbols.
Its three GNU input/output/window buffer alignments are retained and verified
in the executable. All 56 focused comparisons now pass, including compression,
decompression, file metadata, internal aliases, and output errors. Exit cleanup
releases the directory cache and unfinished input ownership; alias write-error
messages retain the configured Bash profile and pinned script line numbers.
All fifteen originals in the first reviewed batch pass natively and under
Valgrind, including valid legacy unpacking, environment options, metadata, and
an unprivileged write-error check. The original 4 GiB size test is running. The installed release remains the validated
117-command Grep build.

GNU Sed 4.10 is pinned from its verified signed archive. Its native oracle
and compiler records are prepared with SELinux explicitly disabled. The
75 original shell/Perl registrations are inventoried for individual review.
Entry translation and runtime compatibility remain separate work.

## Status

The installed executable registers 117 commands: 107 Coreutils entries, GNU Hello,
GNU Time, GNU Which, four GNU Diffutils commands, and three GNU Grep commands,
all with active Rust command entries. No native C command entry remains, and assembly succeeds without the
C-entry opt-in. `printf`, `sort`, `od`, `numfmt`, and `seq` use native numeric
helpers: floating values stay inside GNU C functions and cross the boundary
only as bytes or text. This preserves the host's x87 `long double`
representation without using C2Rust's incompatible IEEE binary128 ABI.
`numfmt` keeps options and field processing in Rust; `seq` keeps its command
control and integer generation path in Rust. Their numeric helpers share
prefixed Rust-owned option state. Native C assertions check the opaque storage
and operand layout. See `evidence/translation.json` for source and helper hashes.

GNU helper bodies remain native C. For example, `cp.c` is translated, while
`copy.c` and its data-copy helpers remain C. Every active Rust command's C entry
object is removed from the linked helper archives. This is a behavior-first
port in progress, not a claim that every implementation body is already Rust.

The current release executable is 2,895,248 bytes (2.76 MiB), dynamically linked
on the recorded host profile. This does not include native shared-library
dependencies or command-specific runtime helpers such as GNU `stdbuf`'s library.
Cross-platform builds and release packaging remain open.

The current release dynamically links libacl, libattr, and libcap for GNU metadata
helpers, and libpcre2-8 for Grep PCRE matching. A separate static-link trial against the previous 2,405,616-byte
build removes those two runtime dependencies and grows that executable by
7,784 bytes to 2,413,400 bytes. All five selected
original metadata scripts pass natively and under Valgrind on that trial.
It is separate evidence, not a change to the main release's link profile.
Building the ACL and Attr libraries from pinned sources and incorporating their
commands is planned after Coreutils; other shared dependencies remain.

Capability support is now integrated in the installed build. All nine original
capability, color, ACL, and xattr-call-count scripts pass natively and under
Valgrind, with 80 clean logs per build. Comparing the translated ls function
bodies found only the intended `has_capability` change; the remaining 207 bodies
are identical. The GNU preparation script requires libcap development headers
and checks `HAVE_CAP`; existing builds without support need `--reconfigure`.
The earlier isolated candidate and branch remain preserved. The combined
108-command candidate passes the entire Coreutils baseline and Hello checks.
`evidence/capability-hello-activation.json` records executable/helper identity,
retained previous binaries and reports, and reuse of the validated observations.
The previous GNU oracle is preserved under `build/history` with its build
profile. This integration closes three recorded original-script skips.

Recorded checks:

| Check | Result | Coverage limit |
| --- | --- | --- |
| GNU help/version comparisons | 428/428 pass | Both multicall and symlink entry forms |
| Valgrind help paths | 107/107 pass | Help only |
| Normal/error behavior fixtures | 318/318 match GNU | All 107 commands; streams, status, contents, modes, owners, link topology |
| Valgrind normal/error fixtures | 318/318 clean | Bounded fixtures; retained allocations recorded separately |
| Instrumented GNU comparisons | 318/318 pass | Saved Valgrind observations, assessed separately from native arithmetic |
| Original GNU cp tests | 91 pass, 11 prerequisite skips, 30 excluded | 66 scripts, root and ordinary-user profiles |
| cp mutation comparisons | 879 pass | Bounded local backup/removal/error fixtures |
| cp backup comparisons | 75 pass | Backup names and preserved fixture data |

Cleanup now releases `expr` results, `date` timezone/format storage,
`tail` file records on return (including ignored follow mode), and `tr` construct lists (including parse
failures). The reviewed original GNU runner now passes 650 of 655 scripts/selections.
The executed inventory contains 595 shell scripts and 5,958 Perl cases.
Forty-seven Perl scripts run their complete
runtime case lists; 13 others retain explicit case selections. Case-count
checks also cover scripts that call GNU's Perl harness more than once.
GNU's seven generic scripts pass across all 107 commands. New coverage includes
user lookup, processor counts, working directories, permission-sensitive touch
and truncate behavior, checksums, encodings, UTF-8 text processing, and tee.

Permission-sensitive scripts run as UID/GID 65534 with private staged binaries.
The original ownership preserve-root script now passes natively and under
Valgrind inside a copied disposable root. Private mount/PID namespaces supply
its procfs and device filesystem; the test process runs as UID/GID 65534 with
no supplementary groups or privilege gains. All seven chown/chgrp/chmod
operations are matched to instrumented command logs, with nine clean logs
per build including version and framework cleanup. Parent-root metadata is
checked unchanged. `evidence/private-root-ownership.json` records the copied
inputs, root identities, credentials, commands, and earlier staging attempts.
The user-lookup profile binds a local-files NSS configuration inside a private
mount namespace; the host configuration is checked unchanged after each run.
The explicit UTF-8 profile uses the installed fr_FR.utf8 locale. Conditional
branches still depend on the script's own platform prerequisites.
Original-script totals aggregate saved runs across incremental builds; each
result records the binary hash it tested.
Named batches checkpoint each completed implementation atomically, so a
completed GNU run survives a disconnect during its rboxc comparison. `--resume`
reuses a side only when the test definition, binaries, drivers, runtime profile,
and saved output/Valgrind log hashes match. Failed results remain failures.
Eight checkpoint tests cover invalidation, missing/changed logs, interrupted
writes, and exclusive batch locks. An original dispatcher test verifies resume
with both completed sides and with only GNU completed.
`--script` selects an exact script; `--report-name` gives independent batches
distinct evidence files. `scripts/run-reviewed-batch.py` accepts an explicit list
of reviewed scripts, `--report-prefix`, and `--jobs` (1–8); it resumes each script
independently. Launcher and preload-adapter builds use private per-run directories.
Two concurrent original launcher tests pass and reuse their saved runs.
New results record tested binary hashes, and exec-wrapper tests can request
Valgrind child tracing.

Additional original-script coverage includes random-sort permutations,
compressed sort output and compressor process handling, and nproc's simulated
cgroup quotas. All four scripts pass natively. The quota test now also passes
all original assertions under Valgrind: each implementation completes 16 chroot
invocations and produces 20 memory logs. The reviewed x86_64 profile stages the
real nproc ELF, matching Valgrind runtime, and loader/libc debug symbols inside
the original disposable chroot. Private PID and mount namespaces supply real
procfs for Valgrind while binding only PID 1's scheduler/cgroup inputs to the
original fixture files. Host cgroups are unchanged. Every original NPROC call
must have a matching instrumented invocation; binary/runtime hashes, quota
values, scheduler policies, and thread overrides are recorded. The result
remains memory-open: the original preload fixture leaves /proc/self/sched open
with a 472-byte FILE allocation in 15 logs per implementation. Earlier runtime
staging failures remain in the observation history. Twenty-one small shell scripts and generated factor test t37
now also pass Valgrind, covering dates, cat line endings, dd case conversion,
split suffixes, unique sorting, od byte order, printf hexadecimal escapes,
df block headings, chmod modes/options, arch, false/true statuses, printenv,
echo, and tail input positioning. Generated factor tests t11–t13 also pass
Valgrind. Twenty-seven further generated factor scripts (t04–t08, t14–t20, and t22–t36)
now pass their full original checksum comparisons under Valgrind. The final
two generated factor comparisons now also pass under Valgrind: GNU took
2,530/2,532 seconds and rboxc took 4,850/4,828 seconds, with six clean logs per
build for each original checksum. Both finished within the extended two-hour
deadline. Earlier 30-minute interruptions remain in the observation history.
`evidence/factor-extended-deadlines.json` records the report and log hashes.
Native deadlines are unchanged.
Extended locale branches run where configured.

The complete floating-point-limit sort script passes natively in C and French
locales. Under Valgrind, both GNU and rboxc misorder the same minimum long-double
values around zero; this remains an assertion failure. The original nohup
script now also runs with a private terminal: all assertions pass, while
the two host-shell descriptor findings remain open. The nohup entry closes
its replacement /dev/null input on exit after setup or exec failure, preserving
errno and GNU diagnostics/statuses. Successful exec transfers the descriptor
to the command. All three original failure-path descriptor findings are now
clean for rboxc; GNU retains them. The source-hashed C2Rust translation applies
this ownership change through scripts/nohup_cleanup.py. Compression assertions pass with full Valgrind child tracing: all 7,304 logs
per implementation contain summaries. The result remains open because the
external compression shells and GNU tr helper retain resources. The earlier
measurement with incomplete exec logs is preserved in the observation history.
The separate compressor-process script now also passes every original assertion
under Valgrind, with 9,868 complete logs per implementation. Candidate sort
processes are clean; the result remains open for external compression-shell,
GNU tr, and GNU expr findings (8,094 candidate descendant logs).
The complete I/O-error script also passes its assertions, with 1,336 logs per
implementation. The latest installed-build comparison passes all assertions
and remains memory-open for 263 shell logs and three cat/dd/tac SIGPIPE exits.
The cleanup build below repairs the two normal
cat/tac write exits; the other three logs follow SIGPIPE termination.
The original large-directory memory test now also passes natively for rm,
du, and chmod over 200,000 entries. Its additional 35,000 KiB allowance remains
unchanged: GNU measures a 14,004 KiB baseline and uses a 49,004 KiB limit;
rboxc measures 15,004 KiB and uses 50,004 KiB for each traversal. Both complete
the script in 3.413 seconds in this run.
`evidence/directory-traversal-memory.json` records the measured limits and log
hashes. This native resource measurement is separate from Valgrind coverage.
The separate original write-error responsiveness script now passes natively in
both builds: 37 writer configurations across 27 commands complete 74 bounded
/dev/full and closed-pipe checks per implementation. Its original memory limits
and per-writer deadlines remain in effect. evidence/write-responsiveness.json
records the command coverage and hashed logs; this native resource measurement
is separate from the instrumented I/O-error script.

The cat/tac cleanup now closes its named input and releases
working buffers on normal and fatal exits. Tac also closes its cached temporary
stream. Ownership slots are cleared before close/free, and cat clears a buffer
slot before replacement allocation can fail. Cleanup preserves errno and GNU's
existing diagnostics. The changes are applied by scripts/write_cleanup.py during
C2Rust regeneration. All 34 focused GNU comparisons pass: 32 normal-exit cases
have clean Valgrind memory/descriptors, and two closed-pipe cases preserve GNU's
SIGPIPE termination with the resulting resource findings recorded separately.
Coverage includes /dev/full, pipes, regex separators, buffer growth, and multiple
operands. Seven selected native original scripts and six Valgrind originals
also pass, with 60 clean candidate process logs. The earlier candidate remains at
`target/write-cleanup/release/rboxc`; evidence/write-cleanup-candidate.json records
its source and binary hashes. Its changes are included in the activated cleanup
build described below. The full original
I/O-error rerun now also passes all assertions for the candidate: GNU takes
967.858 seconds and rboxc 913.393 seconds, with 1,336 logs each. Both repaired
normal cat/tac write exits are memory-clean. The strict comparison remains
open for 263 external-shell logs and three SIGPIPE exits in cat, dd, and tac.
The separate candidate report preserves these findings without replacing the
historical observation above. The full activated-build rerun now also passes
all assertions: GNU took 931.210 seconds and rboxc took 863.037 seconds, with
1,336 logs each. `evidence/installed-cleanup-io.json` verifies both repaired
normal exits and records the remaining 266 finding logs. The earlier 268-log
observation is retained in history.
The staged candidate also passes 428 help/version comparisons, 107 Valgrind
help checks, 11 dispatcher checks, and all 318 native behavior, Valgrind behavior,
and instrumented-equivalence comparisons. These baseline drivers accept
`--candidate` with a required separate `--report-name`; named runs have separate
log directories and cannot overwrite installed-release reports. Reports reject
binary or runtime-helper changes during execution. Seven profile checks cover
report isolation, input changes, and separate log directories. The equivalence
assessment accepts `--observations` for the separately saved behavior report.

GNU's original `shuf` reservoir test now passes all 81 input/output-size
combinations in both builds. The original test already invokes Valgrind, even
in the runner's default profile. A separate capture profile preserves those
options and adds descriptor tracking and exec tracing: all 81 rboxc sampling
processes finish with zero memory errors, owned descriptors, or lost bytes.
GNU's reference reports descriptor findings and exits 1 with those added
checks, so this stricter comparison remains open.
`evidence/shuf-reservoir.json` records every matrix case and log hash; each
build has 83 captured logs including version and Valgrind prerequisite checks.

The original sleep-parameter script now also passes Valgrind, with 28 clean
logs per build and its short timeout assertions unchanged. The initial
instrumented yes script passes its assertions but exposes an 8 KiB owned
buffer leak on a normal output-error exit. The Rust entry now frees that
buffer after its diagnostic, while preserving borrowed argument storage.
A separate candidate passes the full native original with syscall fallback
checks enabled, plus 11 memory-clean output-error fixtures and two SIGPIPE
equivalence fixtures. The instrumented original remains open: its injected
pipe failures prevent Valgrind semaphore initialization in both builds.
The earlier sandbox run skipped those conditional strace branches.
`evidence/yes-cleanup-candidate.json` records both profiles and the focused
checks; this candidate also contains the prior cat/tac cleanup changes.
Reapplying the current transformation recipes to the saved C2Rust outputs
reproduces all three staged entries byte for byte.
`evidence/write-cleanup-regeneration.json` records the input, recipe, and output
hashes; this check does not rerun the C2Rust translator.
The latest candidate also passes the complete baseline: 428 help/version
comparisons, 107 Valgrind help checks, 11 dispatcher checks, and 318 native
behavior, Valgrind behavior, and instrumented-equivalence comparisons. These
reports use the candidate binary hash and its matching stdbuf helper. After
both factor runs completed and their results were merged, that exact build was
activated at `target/release/rboxc`. `evidence/cleanup-activation.json` records
the old binary/report backups and hash-verified reuse of the complete baseline
reports. Their origin is preserved; reuse does not claim a rerun. All 11
dispatcher checks were then rerun successfully from the installed path.

The full original `truncate` size-limit validation now passes natively and
under Valgrind, with five clean logs per build. The original two-rotation
`tail` watch-release check also passes its assertions in both modes. Its
SIGTERM cleanup leaves one inotify descriptor and 4,574 reachable bytes in
both builds, so its strict memory result remains open. Report and log hashes
are recorded in `evidence/tail-truncate-originals.json`.

The unchanged original `split --filter` script now passes its assertions
natively and under Valgrind. Its instrumented run records 23 logs per build;
external shell and xz findings keep the strict result open. The original
references an unset `FILE` variable and skips its regular-file branch. Three
supplemental cases cover one, two, and three regular-file chunks with distinct
boundary bytes and filters that stop after one byte. All outputs match GNU;
all split processes and head children are clean, while six external shell
children retain redirected descriptors. `evidence/split-filter-original.json`
and `evidence/split-filter-regular-current.json` retain these separate results.

The reviewed Valgrind evidence records 594 clean results out of 632 scripts
or selections: 5,729 Perl cases and 46,580 candidate/descendant process logs.
The 38 open results comprise 29 with passing original assertions but unresolved
memory/descriptor evidence, two prerequisite skips, seven with assertion
failures under instrumentation. No current result is interrupted by a watchdog deadline. The status report records
these categories separately; they do not change the strict clean-pass count.
Two env results remain open. The env script encounters shebang/argv differences
under instrumentation; the env -S script passes its assertions but records
memory and descriptors retained by host script interpreters. Both pass natively.
The separate env signal-handler script also passes natively; its Valgrind run
has timing/signal assertion differences and incomplete logs in both builds.
The original unknown-command dispatcher test now passes natively and under
Valgrind. The buffering script now also has clean candidate memory evidence;
its previous two preload-helper allocations remain in the observation history.
Matching GNU findings are not counted as clean.
Three more results remain open: dd's intentionally closed-stderr diagnostics,
install's external strip children with host-shell descriptors and host-tool heap findings,
and cat's injected pipe-creation failure interfering with Valgrind startup.
The complete basic install and CHLD-handling scripts now pass natively and with
clean candidate Valgrind evidence. Their original strip inputs are staged as
real ELF files under a separate build-input directory; executable launchers
remain on the test PATH. This preserves both strip coverage and GNU's verbose
program-name diagnostics. Each result hashes the ELF inputs. Earlier launcher
and diagnostic findings are retained. The five additional generic option/help
profiles pass natively. The help/version ordering and option-documentation
reference scripts now also pass Valgrind. The complete 106-case invalid-option
suite passes in both modes. Its Perl framework uses GNU env to bypass shell
builtins; that framework invocation now starts each command's Valgrind launcher
outside instrumentation, preserving short argv[0] in diagnostics. The actual
`env -/` case remains instrumented. The runner requires a matching Valgrind
command log for every one of the 106 cases, in addition to GNU's case-count and
output assertions. Earlier path-prefixed diagnostic differences remain saved.
The full help/normal-output comparison now passes with the extended one-hour
Valgrind deadline: GNU finishes in 1,747.649 seconds and rboxc in 1,707.440
seconds, with 1,151 clean candidate process logs. The earlier GNU timeout
remains in observation history. The source-option inventory comparison also
passes with its extended deadline: GNU finishes in 2,320.199 seconds and
rboxc in 2,268.521 seconds, with 1,213 clean candidate process logs. Eight
reference-process findings and both earlier timeouts remain recorded. Native
limits are unchanged, and timeouts do not count as assertion passes.
The 54 reviewed tac Perl cases now also pass Valgrind, with 55 clean candidate
process logs. This remains a case selection, not a full Perl-suite pass.
All 46 registered move scripts now pass natively and under Valgrind.
The GNU oracle and helper build explicitly enable ACL and extended-attribute
support. `evidence/gnu-build-profile.json` records configuration and oracle
hashes. Original move, mkdir, ls, and xattr metadata scripts pass in both modes;
previous prerequisite skips remain in the observation history.

Reproducible cp/mv finalizers now release the destination-directory descriptor
and per-command source/destination record tables on normal and fatal exits.
Eight additional behavior fixtures cover multiple operands and setup errors.
The original cp rerun remains at 91 passes and 11 prerequisite skips, with no
GNU mismatches. Move coverage includes cross-filesystem hard links, metadata,
special files, casefolding, overwrite decisions, and permission failures.
A private controlling-input terminal enables the original interactive test;
its driver forwards termination to its child process group. Root tests that
launch ordinary-user children use staged binaries and a private shared TMPDIR.
The one-case Perl move script uses a separate fixture directory so its file
named src does not collide with the harness's executable directory.

All 29 reviewed du scripts pass natively and under Valgrind. A reproducible
cleanup now closes named filename-list streams on read errors while preserving
GNU's diagnostics and normal close behavior. Five new behavior fixtures cover
empty, missing, directory and stdin filename lists. The original threshold
script runs on a verified disposable ext4 image in a private mount namespace.
All three basenc scripts pass natively, including all 154 Perl cases and the
20 MiB base58 input. The Perl and large-input scripts also pass Valgrind; the
virtual-memory-limited streaming test remains a native resource measurement.
A test-only C launcher uses Valgrind's client request to avoid nested Valgrind
when env clears PATH or launches another instrumented command; env-null passes.

All 22 registered dd scripts pass natively. Nineteen of the 20 instrumented
scripts pass; the closed-stderr script still records invalid-descriptor warnings.
The two native-only scripts respectively constrain virtual memory below
Valgrind's startup needs and preload an upstream test helper that leaves its
own FILE open. These limits are explicit in the manifest. dd now closes named
input/output descriptors on fatal setup errors, transferring ownership away
before its normal close path. The original ftruncate/fstat error test passes
under Valgrind with this fix. Device-offset coverage runs on a new 32 MiB ext4
image in a private mount namespace and verifies the loop device's exact backing
image before launching either implementation.

Five additional chgrp scripts pass natively and under Valgrind with child-only
supplementary groups 0 and 2; the parent group list remains unchanged. All seven
install scripts pass natively, including ordinary-user permissions and strip
inputs. The two originals that strip staged executables run natively because
instrumentation replaces those paths with shell launchers.

install now releases its destination-directory descriptor, its destination
bookkeeping table (including entries), and owned strip-program strings. Repeated
strip-program options release the preceding value. Tracking begins at each
acquisition and covers early option failures as well as normal completion.
The original multi-file directory-creation test now passes under Valgrind.
Six new behavior fixtures cover existing and newly created directories, a
missing source, invalid modes, and repeated strip-program options. All seven
install originals still pass natively; four of its five instrumented originals
are clean, with the external-strip observation retained separately.

Additional native coverage includes four cat scripts, five df scripts, three
shred scripts on disposable files, all three wc filename-list/parallel scripts,
the complete 53-case printf coverage suite, and thirteen ls display scripts.
Completed instrumentation results and remaining observations are recorded
individually; native success alone does not mark a Valgrind pass.

wc now closes its owned filename-list stream after streaming input and on
early failure. Its regular-file path removes ownership before fclose, and its
metadata pass preserves EBADF for a stdin descriptor already closed by that
path without querying the invalid descriptor again. Borrowed streaming stdin
retains GNU's handling. All eight registered wc scripts pass in full, natively
and under Valgrind on this host, including all 16 basic and 14 filename-list
Perl cases. Five new behavior fixtures cover empty, nonregular, missing, and
unreadable-as-data filename lists plus piped input.

All fourteen reviewed split scripts now pass natively and under Valgrind.
The entry tracks reopened input, output descriptors still inside create(),
its aligned buffer, temporary stream, and initialized round-robin records.
Normal close paths remove ownership first, including failed fclose calls.
Cleanup frees each output name and its table in linear time. Six additional
behavior fixtures cover round-robin output/extraction, empty output, failed
creation, and suffix exhaustion. The original full-temporary-filesystem test
runs entirely inside a disposable outer filesystem and private mount namespace;
its own small ext2 image also remains within that fixture.

The runner now materializes all 41 generated factor scripts using GNU's pinned
create-test.sh and run.sh, recording generator and output hashes without editing
the original source tree. Expensive execution is explicit in each manifest row.
All 43 registered factor scripts now pass natively: all 41 generated ranges,
the complete 51-case Perl suite, and the parallel script. A reproducible bound
ends trial division after the final prime-table block, avoiding a terminal
lookahead beyond the table when double-limb division leaves an unaligned index.
This resolves all sixteen previously failing ranges; their earlier results
remain in the observation history. Generated t21 and t38 through t40, the Perl suite,
and the parallel script also pass Valgrind. The first four generated ranges
(0 through 40 million, with their shared endpoints) now also pass Valgrind and
match GNU's expected complete-output checksums. The 20–30 million range finishes
with six clean candidate process logs: GNU takes 396.216 seconds and rboxc
403.839 seconds on the recorded host. The 30–40 million range also has six clean
candidate process logs, taking 740.765 seconds for GNU and 749.676 seconds for
rboxc. Execution times are recorded for
new runs. Named native batches
can be merged with scripts/merge-reviewed-evidence.py --native after completion.

Forty-seven reviewed rm scripts pass natively and under Valgrind, including
interactive decisions, inaccessible directories, deep trees, and disposable
read-only and cross-filesystem fixtures. All five stty scripts and tty pass
natively with a private controlling terminal. The complete stty pair matrix
now also passes Valgrind with 9,030 clean logs in each build; GNU takes
5,706.347 seconds and rboxc 5,137.156 seconds with the original matrix intact. Additional du coverage includes sparse and allocated
large files, filesystem boundaries, and bind-mount cycles.
The full original 400,000-file deletion benchmark now passes on a verified
512 MiB ext4 image with 524,288 inodes. GNU and rboxc each remove the directory
in 4 seconds natively; Valgrind measurements are 7 and 8 seconds respectively,
with three clean process logs per implementation. These are single observations
on this host, with the original 60-second minimum threshold unchanged. The
existing 32 MiB image profile still passes the original du threshold script.
The benchmark summary and report hashes are in evidence/rm-ext4-benchmark.json.

Every registered df script has a native result: twelve pass and two skip
because this host lacks the requested user-namespace/proc and rootfs profiles.
The six newly applicable scripts also pass Valgrind; rootfs remains an explicit
instrumentation skip. The GNU Hurd-only id script skips on Linux. These two
Valgrind prerequisite skips account for two more open results above. The eighth
open result is ls/stat-free-symlinks: both GNU and Rust pass natively, while
Valgrind adds one stat call to both and violates the script's original syscall
count assertion. That instrumentation difference is not counted as a pass.

Six further full original scripts now pass natively and under Valgrind as
UID/GID 65534: repeated removal failures, readlink path resolution, ls symlink
diagnostics, removal of 250 long filenames, early sort permission errors, and
rm directory-read failures using the original preload helper. All 34 logs per
build are memory/descriptor-clean. `evidence/filesystem-diagnostics.json`
records these comparisons and their source and log hashes.

Additional original ls tests cover color, locale, timestamps, removed working
directories, d_type, symlinks, and extended attributes. The complete 123-case
tail Perl script passes natively and under Valgrind. The date debug script now
passes after owned timezone and adjusted-format storage gained cleanup on
fatal exits; tail also frees its table when follow mode is ignored for piped
stdin. Four new behavior fixtures cover these paths. Original option alias,
usage/getopt consistency, and documentation-reference scripts pass across all
107 commands. A native Valgrind launcher can bypass its shell wrapper for
removed-directory tests, and alternative candidate binaries require separately
named reports to avoid overwriting main-release evidence.

The original sort merge and filename-list suites now pass natively and under
Valgrind. Reproducible ownership cleanup releases argv pointer arrays,
merge-file arrays, named filename-list streams, and the token obstacks whose
strings those arrays borrow. A failed temporary-file node is freed before
GNU's fatal diagnostic while preserving errno. Tail now finalizes named file
descriptors, inotify state, event buffers, its file table, and its PID list on
normal or fatal exit. Its original FIFO/PID test and complete Perl suite pass
Valgrind. Fourteen added behavior fixtures cover these sort and tail paths.
Signal termination still retains GNU's signal semantics; these finalizers do
not claim cleanup on SIGTERM or SIGKILL.

All nine additional sort option/locale scripts pass natively, including
Swedish grouping and French/Japanese months. Three more private locales and
the GB18030 locale extend the test collection to twelve. The non-UTF-8 cut,
numfmt, and tac scripts and all 57 multibyte expr cases pass in both modes.
The complete 1,862-case head tail-elision matrix passes natively and under
Valgrind: both implementations finish all cases, with 1,863 complete candidate
process logs and clean memory/descriptor evidence. Original streaming memory-limit scripts pass for cut,
expand, unexpand, and pr, with instrumentation intentionally excluded from
those native memory-budget measurements.

The outer test watchdog now uses the absolute host timeout path, preventing
a tested timeout applet from instrumenting the entire shell harness. Results
record the watchdog hash and selected shell. The original timeout group test
uses Bash for its required signal syntax. Timeout parameter, init-parent, and
blocked-alarm tests pass in both modes. Its basic and group tests pass
natively, but intentional SIGKILL and external child execution leave incomplete
Valgrind reports. The FIFO/mode test disables Valgrind's optional debugger
socket because its restrictive umask prevents that socket from opening; the
original script and its mode assertions remain unchanged and pass both modes.

The two chroot scripts pass natively. The credentials test gives only its child
supplementary group 0, matching the original test's assumption; the parent's
groups remain unchanged. Its Valgrind profile now keeps one pre-opened log
descriptor across credential changes and disables optional vgdb files. All 32
candidate process logs, spanning 95 exec images, finish cleanly. The launcher
stops on the first memory error; the parser sums every recorded summary and
requires the last exec image to finish, rejecting mixed-PID or truncated logs.
Seven parser regression checks pass, and reassessing 20,983 existing GNU and
candidate logs changes no recorded results. The separate chroot failure script
now uses GNU's explicit multicall dispatcher while tracing children: this keeps
its expected command name in diagnostics, and all 17 candidate process logs
finish cleanly. Both original chroot scripts now pass natively and under
Valgrind. Generic read-error,
warning-output, write-error, closed-stdout, and line-buffer responsiveness
scripts pass natively across their registered applicable command sets.
The generic read-error script now passes Valgrind after owned-input cleanup
for cat, csplit, date, join, shuf, sort, tail, and uniq. The cleanup also closes
cat's splice pipe and frees date's batch line buffer on fatal read errors.
Thirteen additional behavior fixtures pass native GNU comparisons and Valgrind;
all eight selected original regression scripts also pass natively. Native
launchers now trace the warning-output and line-buffer responsiveness scripts
through exec; both pass Valgrind. GNU's complete option-alias script also passes
under Valgrind with 456 clean candidate process logs, including its text-processing
helpers. The complete help-option recognition script passes natively and under
Valgrind with private local-files NSS: all 1,923 candidate process logs finish
cleanly, and the host configuration is checked unchanged after each run.
An earlier run recorded descriptor warnings in the host's libnss_sss user-lookup
library in both builds; that partial run is explicitly marked interrupted and
retained in observation history.
The harness stages and hashes each build's own stdbuf library beside its test
executables. The closed-stdout script passes
its assertions but stays open under Valgrind: printf, verbose cp, and two mktemp
invocations report operations on intentionally closed stdout, without lost heap
or leaked descriptors. Install's strip-program assertions also pass with child
tracing; findings in the host shell, sed, and grep remain recorded as open.
Seven additional original tail-follow scripts now have Valgrind evidence.
Append-only operation on the disposable ext4 image and follow-name termination
pass cleanly. Header updates, missing directories, rename tracking, descriptor
following, and initial flushing pass their assertions but keep descriptors
when the scripts terminate tail with SIGTERM; all nine affected candidate
processes record zero lost heap.
The two tail
prerequisite reruns now execute their assertions successfully, but retain
descriptors when follow mode is terminated by a signal; pipe-f also probes
intentionally closed stdout. Six additional tail scripts pass natively, covering
overlapping event headers, retry, symlinks, zero-count sleeping, truncation, and
waiting. The overlapping-header script also passes Valgrind with normal PID-based
shutdown. Four of the other new scripts retain resources on intentional SIGTERM;
the wait script keeps its native 0.1-second deadlines without instrumentation.
The full 50-iteration rotation and directory-recreation originals also pass
natively and satisfy their instrumented assertions. Their interrupted tail
processes retain descriptors at SIGTERM with no lost heap. There are now
twenty-four open reviewed Valgrind results. The observation files retain all
superseded failures and prerequisite skips; none are converted into passes.

The complete 63-case head and 12-case dircolors Perl suites pass in both modes.
Reviewed selections add 33 MD5, 17 SHA-1, and 26 base64 checksum cases, all clean
under Valgrind; these selections are explicitly partial. The original mktemp
randomness/fallback script also passes in both modes. Corrected cp prerequisite
lists enable the non-UTF-8 filename and sparse-2 originals, which pass natively
and under Valgrind as an ordinary user. The complete cp/mv permission matrix
passes natively and under Valgrind with its expensive-test switch enabled.
All 902 candidate processes are clean across permission, umask, overwrite,
and preservation combinations. These three cp results supplement the earlier two-profile cp report
and close three prerequisite skips in the overall script inventory.

GNU's terminal EOF original now passes all 70 cases across 34 commands, both
natively and under Valgrind. Its standalone Perl driver records each actual
Expect spawn and verifies the complete command sequence with and without
input; the upstream EOF, output, and exit-status assertions remain unchanged.
Expect 1.38 is a test dependency. The env signal-handling script passes natively
under Bash, and root-relative symlink coloring passes in both modes. Sort's
seven-descriptor merge test passes natively under Bash; the recorded /bin/sh
prerequisite skip came from shell redirection before sort ran. The original
limit explicitly excludes Valgrind. A reviewed 265-case date selection adds
calendar parsing, formatting, timezone conversion, and debug-mode checks; it
passes natively and under Valgrind. The full date Perl suite remains partial.

The full join suite passes all 147 cases with C and French UTF-8 coverage in
both modes. The ls miscellaneous suite passes both complete 50-case runs, with
and without the generated color configuration, as an ordinary user under
local-files NSS. All four xstrtol diagnostics exercised through pr also pass
in both modes. The original tac temporary-storage-full test passes against a
1 MiB tmpfs inside a private mount namespace. The runner records different
parent/child mount namespace IDs, verifies the parent remains unchanged, and
rejects unknown execution profiles.

The complete expand, fmt, fold, and uniq Perl suites now pass natively,
including 1,020 uniq cases and 47 fold cases with UTF-8 coverage. Their expanded
Valgrind runs also pass in full. Test locale aliases explicitly map normalized encoding names to the
prepared data, preventing silent C-locale fallback. Ordinary-user tests can now
enter the private local-files NSS namespace before dropping credentials. The
new chgrp, touch, truncate, mkdir, and id permission scripts pass Valgrind.

The runner preserves exit statuses for GNU assertions and then assesses memory
and descriptors separately. `scripts/merge-reviewed-evidence.py` merges completed
named batches from `evidence/raw/` and retains replaced unresolved results in
`evidence/gnu-reviewed-valgrind-observations.json`. Source and binary hashes,
unique suite logs, and per-process observations support later review.


The directory/link tests exposed an owned `ln` directory descriptor retained
at exit. Its cleanup now also covers fatal backup-option errors. The full
`comm` tests exposed input files retained on fatal ordering errors; successfully
opened inputs are tracked and closed, with borrowed stdin excluded. `tsort`
now frees edges removed while reporting a cycle and closes a reopened input
on early errors. All changes are reproducible postprocessing steps; GNU's
original sources and oracle are unchanged. The three additional `ln` behavior
fixtures cover successful linking, an existing destination, and invalid backup
options. These fixtures and the full reviewed scripts pass under Valgrind.

The complete encoding, nl, and tee scripts exposed files retained on early
errors. The Rust entries now track owned input streams and output descriptors,
release them on fatal exits, and leave borrowed standard streams to GNU's
normal finalizers. Minus-prefixed chmod modes now release their concatenated
argument storage, including early argument errors. These are reproducible
postprocessing changes to six Rust entries; the GNU oracle remains unchanged.
Eleven added regression fixtures cover these paths. The seven newly reviewed
chmod scripts pass under Valgrind, with local-file NSS lookup for numeric
ownership diagnostics. All 257 behavior fixtures and instrumented comparisons
pass; the reviewed original scripts for the changed implementations pass.

The stat default-format strings are now released on exit, including rejected
filesystem queries for standard input. Borrowed explicit formats retain GNU's
ownership. Five new fixtures cover default and terse formats and error exits;
all six reviewed stat scripts pass normally and under Valgrind, including the
full 18-case printf suite. Six more wc, mkdir, and rmdir scripts also pass in
both modes. The superseded stat leak report remains in the observation history.

The chown entry now releases its owned user/group names, including reference
lookups and group-only diagnostic names; chgrp uses the same implementation.
The mktemp entry tracks its template and randomized destination separately,
releasing both after success or failure. Seven additional fixtures pass native
and instrumented comparisons. All four newly reviewed ownership scripts and
all three mktemp scripts pass natively. The complete 30-case mktemp Perl suite
and the Unicode filename script also pass under Valgrind; the write-error
script now passes with the shared-stream adaptation described below.

The mktemp Perl Valgrind profile uses a test-only preload constructor to
restore the client's original TMPDIR after Valgrind starts in a valid private
directory. This permits the suite's nonexistent-TMPDIR assertions to execute
in both implementations. Invalid, empty, valid, and unset TMPDIR restoration
were checked separately. The adapter is compiled with warnings as errors and
its source hash is recorded with the results; it is not linked into rboxc.

GNU's shared error helper now uses F_GETFD for descriptor-validity queries.
On this glibc profile, standard-stream finalization skips a redundant close
when the descriptor is already closed and the stream has neither pending
output nor a prior error. Other streams and output failures retain GNU's
normal close path. Nine direct helper comparisons preserve GNU status and
streams and pass Valgrind; all 257 behavior fixtures and instrumented
comparisons pass. The two closed-stdout touch scripts and mktemp write-error
script now pass Valgrind, with their earlier observations retained.

The head Rust entry now closes its owned input descriptor on fatal output
errors. Borrowed stdin remains under GNU's normal finalizer. Its original
write-error script, seek-position script, and reviewed Perl selection pass
natively and under Valgrind after this fix.

Thirty-six additional original scripts/selections now pass natively, covering
date, fmt, head, od, seq, pathchk, sync, and ls; all also pass Valgrind.
The od alignment matrix checks all 400 format pairs for each implementation.
The new complete date Perl suite adds three cases, and od retains a reviewed
17-case partial selection. The extended locale profile exercises Ethiopian,
Iranian, and Thai calendars and UTF-8, ISO-8859-1, and KOI8-R text handling.
`scripts/prepare-test-locales.py` builds eight test locales, installs their
separate collection at `/usr/lib/locale/rboxc-tests`, and records file hashes.
Tests select it through LOCPATH. The system locale archive remains unchanged;
the standard data location permits Ubuntu's confined locale utility to read it.

GNU's complete random-sort benchmark now passes natively and under Valgrind.
Each implementation sorts 500,000 random 100-character lines (50.5 MB including
newlines), and the original script compares the complete output against Perl's
independent sort. Both candidate process logs finish cleanly. The script
generates fresh random data per run, so its elapsed times are not a comparison
on identical input.

GNU's original multicall test exposed different unknown-symlink diagnostics.
Alternate executable names now reach the translated GNU dispatcher, including
`ginstall` and names ending in `coreutils`. The `rboxc COMMAND` interface retains
its own command-selection behavior. Eleven dispatch checks pass, including
nine GNU comparisons under Valgrind; all 428 help/version comparisons and 107
Valgrind help paths pass after the change.

The complete pinned suite registration contains 733 scripts, including 41
root tests and 41 generated factor tests. `scripts/suite-inventory.py` reconciles
the original test evidence into `evidence/gnu-suite-coverage.json`: 639 scripts
passed, 13 have selected-case coverage, five are skipped, 29 are excluded,
and 47 remain pending. No recorded native failures
remain in the executed selections. Three SELinux-only scripts whose names do
not identify the feature (id/context, id/no-context, and mkdir/restorecon) are
explicitly excluded with source hashes in inventory/gnu-suite-exclusions.json.
This scope correction adds no passes. Partial selections and skips
are not full-suite passes; passing scripts can contain platform-conditional
branches. No Coreutils command is certified complete.

`pr` now releases its filename list. `tac` frees the base of its working
buffer after all operands and closes its cached temporary stream after the
final use, including stdin reuse. The Rust `sort` entry releases its
independent argv-name vector on normal completion; its token-file ownership
path remains unchanged. Native source adaptations are separately hashed and
compiled into helper copies; the GNU oracle is preserved. All current
behavior/Valgrind fixtures pass, including bounded buffer growth and stdin.
This does not establish cleanliness for every option or failure path. The `env` fixture
now runs the pinned GNU `printenv`; its earlier host-child findings remain in
the previous committed evidence. Baseline equivalence does not count as Valgrind
cleanliness. A follow-up cleanup closes the directory descriptor owned by
`cp` after copying and restoring parent metadata, preserving GNU status and
errno. All 14 current `cp` behavior/Valgrind fixtures pass; the GNU baseline
still retains that descriptor. This does not certify every `cp` exit path.
Retained allocations and lost allocations are recorded separately. The test
runner returns failure when a selected fixture has an unresolved finding.

`numfmt` now frees its stdin line buffer after reading; `seq` frees the
allocated custom format after printing. The complete current set of 318
behavior fixtures is checked after ownership and descriptor changes. Numeric tests cover all five
`od` float formats and byte swapping, `printf` precision and errors, general
numeric sorting, `numfmt` rounding/units/fields, and finite `seq` paths.
Some long-double outputs differ between native execution and Valgrind in both
GNU and rboxc. Native comparisons establish numerical equivalence; the separate
instrumented assessment compares their Valgrind streams and fixture effects,
requires rboxc's native exit status, and requires clean memory/descriptor results.

Every registered command now has at least one bounded behavior or argument-error
fixture; `mktemp` and `uptime` currently exercise argument rejection. This is
entry coverage, not full option coverage. `yes` is tested through a consumer
that reads only 64 bytes and closes its pipe. `hostname` frees its result,
`df` frees operand statistics, and `shuf` releases random-source, permutation,
input, and reservoir storage, including an unused buffer after early EOF.
`stdbuf` now releases its borrowed environment strings when exec fails.

Assembly builds `libstdbuf.so` (16,144 bytes) from the pinned GNU source with a
source-hashed buffer-ownership adaptation. Keep that helper alongside the binary
when moving it. On GNU libc, successfully installed malloc-backed buffers are
transferred to the stream using `_IO_setb`; close, replacement, and final cleanup
then release them. The buffer remains available for pending output and application
exit handlers. The adaptation uses GNU libc internals and is validated on glibc
2.43; other libc behavior is unchanged. All 24 lifetime cases match GNU natively
and under Valgrind with clean candidate memory/descriptor results. Cases cover
unused streams, ordinary I/O, explicit close, replacement with static buffers,
reopening, and late exit output at four buffering configurations. An earlier
fixture that left reopened files open is retained in raw evidence.
GNU's complete original buffering script also passes natively and under Valgrind.
The main rboxc executable is unchanged by this helper-only update.

Forty-five more original cp scripts now pass natively and with clean Valgrind
evidence, bringing cp's reviewed Valgrind script coverage to 48. These cover
backup policies, source identity, hard/symbolic links, mode and timestamp
preservation, ACLs, read-only directories, sparse extents, reflink fallback,
interactive overwrite options, and special permission bits. Ordinary-user tests
run as UID/GID 65534; root ownership tests stage binaries and writable log/runtime
directories for their children. The original group-preservation script clears
PATH and copies its launcher; the harness now invokes Valgrind by absolute path.
Its earlier launcher lookup failure is retained in observation history.
Checkpoint fingerprints also include GNU init.cfg and the Valgrind driver/runtime.
Four additional original optional-feature scripts return matching prerequisite
skips: two require Smack and two require libcap-enabled ls. These are recorded
as skipped rather than passed.

A narrow `freopen_safer` adaptation checks descriptor
validity with `fcntl(F_GETFD)` instead of self-duplication, preserving GNU's
reopen/protection flow and avoiding Valgrind's self-duplication findings.
All 24 descriptor-preservation cases pass, including close-on-exec flags,
closed neighboring streams, failed opens, and Valgrind checks.

Initial host NSS-library findings, native GNU child allocations, and the
previous descriptor-probe observation are retained in
`evidence/host-dependency-findings.json`; they are not passing fixtures.
Ownership fixtures now use explicit numeric IDs, and the stdbuf child fixture
uses the pinned GNU printf. No Valgrind suppressions are used.

No Coreutils command is certified complete. Full provider suites, missing prerequisites,
additional memory/descriptor paths, and other GNU providers remain outstanding. Excluded original tests are listed with reasons
and source hashes in `inventory/gnu-cp-tests.json`; they are not counted as passes.
Build products and raw test logs stay outside Git; source, scripts, pins, and
result summaries are committed. `evidence/status.json` records the binary hash
and current results.

## Build and reproduce

The shared aligned-allocation adapter rounds backing sizes to alignment
multiples, checks rounding overflow, and gives zero-size requests distinct
allocations. Callers keep their original logical byte counts. `cat` and `split`
now release their owned buffers. The adapter passes 24 bounded allocation checks
and an overflow rejection check under Valgrind, with zero live heap.

The current host needs GCC, GNU Make, Python 3, binutils, Clang/LLVM 21 development
libraries, CMake, Rust 1.93.0 with rustfmt, the pinned nightly, and GNU Coreutils
9.11 source, plus libacl, libattr, and libcap development headers and libraries.
For an existing build without metadata support, run
`sh scripts/prepare-coreutils.sh --reconfigure` before translation and assembly.
Test dependencies include Valgrind 3.26, strace, Perl with Expect 1.38, attr tools, and ordinary
GNU shell utilities. Optional filesystem/locale prerequisites produce recorded
skips. `inventory/sources.json` pins the GNU archive hash and C2Rust revision.
The default GNU source location is `/opt/src/coreutils-9.11`; set
`GNU_COREUTILS_SOURCE` consistently to use another location.

```sh
sh scripts/bootstrap-c2rust.sh
sh scripts/prepare-coreutils.sh
sh scripts/prepare-hello.sh
sh scripts/prepare-time.sh
sh scripts/prepare-which.sh
python3 scripts/translate-coreutils.py
python3 scripts/translate-hello.py
python3 scripts/translate-time.py
python3 scripts/translate-which.py
python3 scripts/assemble-coreutils.py
cargo build --locked --release
target/release/rboxc --list
target/release/rboxc cp --help

python3 tests/coreutils-smoke.py
python3 tests/coreutils-valgrind.py
python3 tests/coreutils-behavior.py
python3 tests/valgrind-equivalence.py
python3 tests/aligned-alloc.py
python3 tests/freopen-safer.py
python3 tests/gnu/cp-original.py
python3 tests/gnu/reviewed-original.py
python3 tests/gnu/reviewed-original.py --valgrind
python3 tests/gnu/cp-backups.py
python3 tests/gnu/cp-mutations.py
python3 tests/hello-original.py
python3 tests/hello-behavior.py
python3 tests/time-original.py
python3 tests/time-behavior.py
python3 tests/which-behavior.py
python3 scripts/update-evidence.py
```

Assembly without `--allow-c-entries` refuses an incomplete translation. The
current 107-command Coreutils set needs no opt-in. The two imported cp differential scripts currently use
the separately installed GNU oracle at `/opt/gnu/coreutils-9.11`; their environment
overrides are documented in the script variables.

Reproducible adaptations live in `scripts/postprocess.py` and the translation
driver: GNU17 parsing of current headers, opaque pointer declarations, pinned
Rust `va_list` and pointer APIs, and GNU's portable arithmetic fallback for
`factor`. C bridges preserve GNU's CPU-feature query and variadic formatting
helper. Build warnings concerning generated ABI declarations and pointer
comparisons remain visible; compilation is not a safety proof.

This repository has local incremental commits and no configured remote. The old
Rbox remote is not used as an implicit publication destination for this project.
