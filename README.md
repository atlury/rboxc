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

The current release executable is 2,397,048 bytes (2.29 MiB), dynamically linked
on the recorded host profile. This does not include native shared-library
dependencies or command-specific runtime helpers such as GNU `stdbuf`'s library.
Cross-platform builds and release packaging remain open.

Recorded checks:

| Check | Result | Coverage limit |
| --- | --- | --- |
| GNU help/version comparisons | 428/428 pass | Both multicall and symlink entry forms |
| Valgrind help paths | 107/107 pass | Help only |
| Normal/error behavior fixtures | 257/257 match GNU | All 107 commands; streams, status, contents, modes, owners, link topology |
| Valgrind normal/error fixtures | 257/257 clean | Bounded fixtures; retained allocations recorded separately |
| Instrumented GNU comparisons | 257/257 pass | Saved Valgrind observations, assessed separately from native arithmetic |
| Original GNU cp tests | 89 pass, 13 prerequisite skips, 30 excluded | 66 scripts, root and ordinary-user profiles |
| cp mutation comparisons | 879 pass | Bounded local backup/removal/error fixtures |
| cp backup comparisons | 75 pass | Backup names and preserved fixture data |

Cleanup now releases `expr` results, `date` timezone/format storage,
non-following `tail` file records, and `tr` construct lists (including parse
failures). The reviewed original GNU runner now passes 206 scripts/selections:
172 shell scripts and 1,871 Perl cases. Nineteen Perl scripts run their complete
runtime case lists; 15 others retain explicit case selections. Case-count
checks also cover scripts that call GNU's Perl harness more than once.
GNU's three generic scripts pass across all 107 commands. New coverage includes
user lookup, processor counts, working directories, permission-sensitive touch
and truncate behavior, checksums, encodings, UTF-8 text processing, and tee.

Permission-sensitive scripts run as UID/GID 65534 with private staged binaries.
The user-lookup profile binds a local-files NSS configuration inside a private
mount namespace; the host configuration is checked unchanged after each run.
The explicit UTF-8 profile uses the installed fr_FR.utf8 locale. Conditional
branches still depend on the script's own platform prerequisites.
Results checkpoint atomically after each script. `--script` selects an exact
script; `--report-name` gives independent batches distinct evidence files.
New results record tested binary hashes, and exec-wrapper tests can request
Valgrind child tracing.

The reviewed Valgrind evidence records 169 clean results out of 171 scripts
or selections: 1,482 Perl cases and 4,065 candidate/descendant process logs.
Two env results remain open. The env script encounters shebang/argv differences
under instrumentation; the env -S script passes its assertions but records
memory and descriptors retained by host script interpreters. Both pass natively.
Matching GNU findings are not counted as clean.
A test-only C launcher uses Valgrind's client request to avoid nested Valgrind
when env clears PATH or launches another instrumented command; env-null passes.

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
`scripts/prepare-test-locales.py` builds six test locales, installs their
separate collection at `/usr/lib/locale/rboxc-tests`, and records file hashes.
Tests select it through LOCPATH. The system locale archive remains unchanged;
the standard data location permits Ubuntu's confined locale utility to read it.

GNU's original multicall test exposed different unknown-symlink diagnostics.
Alternate executable names now reach the translated GNU dispatcher, including
`ginstall` and names ending in `coreutils`. The `rboxc COMMAND` interface retains
its own command-selection behavior. Eleven dispatch checks pass, including
nine GNU comparisons under Valgrind; all 428 help/version comparisons and 107
Valgrind help paths pass after the change.

The complete pinned suite registration contains 733 scripts, including 41
root tests and 41 generated factor tests. `scripts/suite-inventory.py` reconciles
the original test evidence into `evidence/gnu-suite-coverage.json`: 234 scripts
passed, three passed with profile skips, 15 have selected-case coverage, five
skipped, 26 are excluded, and 450 remain pending. Partial selections and skips
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
errno. All 10 current `cp` behavior/Valgrind fixtures pass; the GNU baseline
still retains that descriptor. This does not certify every `cp` exit path.
Retained allocations and lost allocations are recorded separately. The test
runner returns failure when a selected fixture has an unresolved finding.

`numfmt` now frees its stdin line buffer after reading; `seq` frees the
allocated custom format after printing. The complete current set of 245
behavior fixtures was rerun after the latest ownership and descriptor changes. Numeric tests cover all five
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

Assembly copies the matching GNU `libstdbuf.so` (21,600 bytes) beside the release
executable. Keep that helper alongside the binary when moving it. The original
GNU buffering test passes. A narrow `freopen_safer` adaptation checks descriptor
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
9.11 source. Test dependencies include Valgrind 3.26, strace, Perl, and ordinary
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
