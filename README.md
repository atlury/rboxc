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
Provider assignments outside the pinned
Coreutils set still require source/version confirmation before implementation.

## Status

The executable registers 107 Coreutils commands, all with active Rust command
entries. No native C command entry remains, and assembly succeeds without the
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

The current release executable is 2,410,504 bytes (2.30 MiB), dynamically linked
on the recorded host profile. This does not include native shared-library
dependencies or command-specific runtime helpers such as GNU `stdbuf`'s library.
Cross-platform builds and release packaging remain open.

The current release dynamically links libacl and libattr for GNU metadata
helpers. A separate static-link trial against the previous 2,405,616-byte
build removes those two runtime dependencies and grows that executable by
7,784 bytes to 2,413,400 bytes. All five selected
original metadata scripts pass natively and under Valgrind on that trial.
It is separate evidence, not a change to the main release's link profile.
Building the ACL and Attr libraries from pinned sources and incorporating their
commands is planned after Coreutils; other shared dependencies remain.

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
failures). The reviewed original GNU runner now passes 633 of 640 scripts/selections.
The executed inventory contains 580 shell scripts and 5,958 Perl cases.
Forty-seven Perl scripts run their complete
runtime case lists; 13 others retain explicit case selections. Case-count
checks also cover scripts that call GNU's Perl harness more than once.
GNU's seven generic scripts pass across all 107 commands. New coverage includes
user lookup, processor counts, working directories, permission-sensitive touch
and truncate behavior, checksums, encodings, UTF-8 text processing, and tee.

Permission-sensitive scripts run as UID/GID 65534 with private staged binaries.
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
cgroup quotas. All four scripts pass natively; the quota test uses its original
disposable chroot without changing host cgroups. Its Valgrind runtime staging
remains pending. Twenty-one small shell scripts and generated factor test t37
now also pass Valgrind, covering dates, cat line endings, dd case conversion,
split suffixes, unique sorting, od byte order, printf hexadecimal escapes,
df block headings, chmod modes/options, arch, false/true statuses, printenv,
echo, and tail input positioning. Generated factor tests t11–t13 also pass
Valgrind. Sixteen further generated factor scripts (t14–t20 and t22–t30)
now pass their full original checksum comparisons under Valgrind. The remaining
13 generated factor comparisons are running in independently checkpointed jobs.
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

The reviewed Valgrind evidence records 563 clean results out of 594 scripts
or selections: 3,707 Perl cases and 21,154 candidate/descendant process logs.
The 31 open results comprise 24 with passing original assertions but unresolved
memory/descriptor evidence, two prerequisite skips, and five with assertion
failures under instrumentation in both GNU and rboxc. The status report records
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
profiles pass natively; their full Valgrind comparisons are running separately.
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

All 43 reviewed rm scripts pass natively and under Valgrind, including
interactive decisions, inaccessible directories, deep trees, and disposable
read-only and cross-filesystem fixtures. All five stty scripts and tty pass
natively with a private controlling terminal; the large stty pair matrix still
awaits instrumentation. Additional du coverage includes sparse and allocated
large files, filesystem boundaries, and bind-mount cycles.

Every registered df script has a native result: twelve pass and two skip
because this host lacks the requested user-namespace/proc and rootfs profiles.
The six newly applicable scripts also pass Valgrind; rootfs remains an explicit
instrumentation skip. The GNU Hurd-only id script skips on Linux. These two
Valgrind prerequisite skips account for two more open results above. The eighth
open result is ls/stat-free-symlinks: both GNU and Rust pass natively, while
Valgrind adds one stat call to both and violates the script's original syscall
count assertion. That instrumentation difference is not counted as a pass.

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
The complete 1,862-case head tail-elision matrix passes natively; its expanded
Valgrind run is pending. Original streaming memory-limit scripts pass for cut,
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
the original test evidence into `evidence/gnu-suite-coverage.json`: 622 scripts
passed, 13 have selected-case coverage, eight are skipped, 29 are excluded,
and 61 remain pending. No recorded native failures
remain in the executed selections. Three SELinux-only scripts whose names do
not identify the feature (id/context, id/no-context, and mkdir/restorecon) are
explicitly excluded with source hashes in inventory/gnu-suite-exclusions.json.
This scope correction adds no passes. Partial selections and skips
are not full-suite passes; passing scripts can contain platform-conditional
branches. No command is certified complete.

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

No command is certified complete. Full provider suites, missing prerequisites,
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
9.11 source, plus libacl and libattr development headers and libraries.
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
python3 scripts/translate-coreutils.py
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
