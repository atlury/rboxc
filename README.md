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
Named comparison runs now reject an existing report name before creating logs.
`evidence/comparison-report-preservation.json` records the rejection and verifies
that the previous report and log directories remain unchanged.

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
GNU Hello, Time, Which, Diffutils, Grep, Gzip, Sed, BC, Ed, Findutils, Tar, Sharutils, and Cpio are also source-pinned. Remaining
provider assignments require source/version confirmation before implementation.

The upstream release baselines below are the versions used for translation
and matching native test oracles. They are not claims about the latest upstream
release or full compatibility. [inventory/sources.json](inventory/sources.json)
is the authoritative machine-readable pin, including each release archive's
SHA-256; additional entry/helper hashes and signature records are recorded
there where available.

| GNU provider | Upstream baseline | Port scope at this baseline |
| --- | --- | --- |
| Coreutils | 9.11 | 107 installed command entries |
| Hello | 2.12.3 | `hello` installed |
| Time | 1.10 | `time` installed |
| Which | 2.25 | `which` installed |
| Diffutils | 3.12 | `cmp`, `diff`, `diff3`, `sdiff` installed |
| Grep | 3.12 | `grep`, `egrep`, `fgrep` installed |
| Gzip | 1.14 | `gzip`, `gunzip`, `uncompress`, `zcat` installed |
| Sed | 4.10 | `sed` installed; recorded candidate passes 66 unchanged originals, one fixture-adapted original and 56 focused comparisons; one platform skip; seven exclusions |
| BC | 1.08.2 | `bc`, `dc` installed |
| Ed | 1.22.6 | `ed` installed |
| Findutils | 4.11.0 | `find`, `xargs`, `locate` installed; `updatedb` and private `frcode` integrated in candidate |
| Tar | 1.35 | `tar` installed; 216 of 244 original groups validated across recorded profiles |
| Sharutils | 4.15.2 | `uuencode`, `uudecode` installed; both assigned originals validated |
| Cpio | 2.15 | `cpio`, `mt` installed; all 13 reviewed ordinary originals validated; tape-device operations untested |
| Gawk | 5.4.1 | Three aliases in a candidate; 85 focused comparisons and 448 reviewed originals pass; three original failures match GNU |
| Patch | 2.8 | 23 focused comparisons and 38 original scripts pass; two GNU expected failures match; nine scripts held out |
| Binutils | 2.47 | `ar`, `readelf`, `strings` compile and pass the selected local comparisons |
| Inetutils | 2.8 | All 13 entries compile; option, local client and read-only interface checks pass; service profiles open |
| Bash | 5.3 | All 28 names integrated; 44 focused checks pass with clean Valgrind; 17 reviewed originals pass with clean Valgrind; broader acceptance open |
| Less | 704 | Ten focused checks, one production terminal check and all 18 original screen replays pass across declared profiles |
| Screen | 5.0.2 | All three original targets pass across declared profiles; daemon cleanup and socket recovery pass Valgrind |
| Wget | 1.25.0 | Candidate passes 14 focused comparisons and 71 original scripts; 14 optional-feature skips and one upstream-disabled script accounted for |
| glibc | 2.43 | Both entries integrated; 50 focused checks, both getconf originals and three iconv buffer recipes pass |

Screen's remaining daemon cleanup findings are resolved in the recorded Linux
profile. It closes its owned server socket and successfully reopened standard
streams after GNU's terminal restoration, preserves inherited handles in
forked children, and handles failed reopens and socket replacement. Its ncurses
parameter cache is released before the current terminal description.

All three registered original targets are now covered across the declared
helper and terminal profiles. The original attach/detach test and the socket
recovery check pass with **12 clean candidate process logs**; **19 ownership
contracts produce 26 clean parent/child logs**. The earlier native GNU findings,
one incomplete native child log, and intermediate fixture failures remain
preserved. `evidence/screen-owned-validation.json` independently audits the
results and source transformations.

This uses ncurses' exported internal `_nc_free_tparm` cleanup routine at exit.
[evidence/screen-ncurses-cleanup-abi.json](evidence/screen-ncurses-cleanup-abi.json)
records the host library hash, symbol version and upstream source reference.
The public terminal cleanup API alone retains the shared parameter cache in
this profile. Other termcap implementations require separate validation; see
[ncurses memory cleanup documentation](https://invisible-island.net/ncurses/man/curs_memleaks.3x.html).

The current 187-command candidate is **14,717,048 bytes**, at
`target/tar-map-probe-candidate/release/rboxc`, with a byte-identical independent
rebuild. All 428 Coreutils smoke checks, 11 dispatcher checks and 57 Tar
comparisons pass; preceding Screen evidence retains its actual candidate hash. The installed 133-command release remains unchanged;
full GNU-wide acceptance remains open.

Screen's original attach/detach test now passes natively and under Valgrind
against both GNU and rboxc. Its private profile uses short Unix socket paths,
a bounded descriptor limit and the pinned GNU sleep helper. The test exposed
an owned argument-length array discarded by `ClearAction`; the helper now
frees it with the argument strings. Six ownership contracts pass and the
original terminal run confirms the eight leaked bytes are gone.

At the preceding key-binding checkpoint, the daemon retained four owned
descriptors and 418 possibly-lost bytes in the terminal library; these findings
are resolved by the newer daemon cleanup above. Seven Screen processes
and two local test-dependency processes are clean in the final run.
`evidence/screen-key-validation.json` audits those findings, the original
assertions, source changes and preserved environment failures.

The preceding key-binding candidate is **14,715,568 bytes**, at
`target/screen-key-candidate/release/rboxc`, with a byte-identical independent
rebuild. All 428 Coreutils smoke checks, 11 dispatcher checks and five Screen
option comparisons pass. Only the namespaced Screen process helper changed;
previous candidates and evidence retain their actual hashes. The installed
133-command release remains unchanged and full GNU acceptance remains open.

Screen's two original helper units now pass against both the native GNU and
namespaced production helper objects. The private instrumented profile keeps
GNU's allocation mocks active through symbol renaming and initializes two
unused fixture bytes before the original preservation assertions read them.
All four final helper processes are clean under Valgrind. These tests validate
native helpers; the Rust entry's terminal integration is separate.
`evidence/screen-original-helpers-validation.json` checks source identities,
object transformations and raw logs, preserving the initial framework findings.

Less now passes **all 18 original screen replays**, covering **2,245
screen assertions**, in GNU's required `LESSTEST`/`USE_TERMCAP` configuration.
Seventeen retain their original environment. The last retains its original
input bytes, keystrokes and assertions with external hyperlink handlers
disabled; external browser/man integration remains untested.
The test-mode C2Rust entry is byte-identical to the original translation;
a separate executable uses the existing generated Rust entry and namespaced
GNU test helpers. The release-mode candidate passes ten focused comparisons
and a private controlling-terminal check, including exact display output and
terminal-mode restoration.

The original replays exposed a keyboard descriptor left open in both GNU and
rboxc. The helper now tracks successful device opens separately from borrowed
stdin/stderr fallbacks and closes owned handles through GNU's existing cleanup.
Eighteen contracts cover standard descriptor numbers, reuse, reopening,
borrowed fallbacks and errno preservation.
`evidence/less-screen-combined-validation.json` verifies the combined evidence:
**36 clean test-profile Less logs and 11 clean production Less logs**;
the contract processes are counted separately. Initial failures and native
GNU findings remain preserved. The final hyperlink replay executes no opener
child and covers in-document behavior.

The preceding Less checkpoint candidate is **14,716,952 bytes**, 224 bytes larger, at
`target/less-keyboard-candidate/release/rboxc`. An independent rebuild is
byte-identical. All 428 Coreutils smoke checks and 11 dispatcher checks pass.
Only the namespaced Less keyboard helper changed; earlier provider evidence
retains its actual binary hashes. The installed 133-command release remains
unchanged, and full GNU acceptance remains open.

Sed's remaining stdin fixture review is complete. The unchanged `stdin.sh`
creates `stdin-in` but later reads a missing `stdin`; both implementations exit
zero with that diagnostic. A separate private framework alias supplies the
original input bytes and the original assertions pass in both implementations.
`evidence/sed-stdin-memory-audit.json` verifies six clean candidate Sed logs
for the adapted fixture and six for the retained baseline. Helpers are outside
this profile's memory count. All 75 Sed registrations are now accounted for:
**66 unchanged passes, one fixture-adapted pass, one platform skip and seven
exclusions**. This closes the ordinary Sed review queue; full GNU acceptance
and release activation remain open.

One additional Sed original, `bug80573.sh`, passes in a dedicated profile
that retains its own `valgrind --quiet` command. XML logs provide complete
memory and descriptor findings. `evidence/sed-intrinsic-memory-audit.json`
verifies three clean candidate-profile processes and six XML parser checks;
the ASAN prerequisite's uninstrumented Sed call is outside that memory count.
The initial fork/XML formatting failure and native GNU findings are preserved.
The standard runner excludes this profile to prevent nested instrumentation.
This brings the recorded Sed originals to 66 passes and one platform skip
across the standard and intrinsic profiles; the fixture-adapted pass is separate.

Sed's preceding standard profile completes all **66 previously reviewed
original scripts** with integrated Coreutils helpers: **65 pass** and
`obinary.sh` retains its unchanged platform skip. The original Perl suites,
BSD compatibility tests and 2 GiB substitution test are included. Four private
batches retain the original programs, fixtures and assertions.

`evidence/sed-multicall-suite-memory-audit.json` verifies **992 clean candidate
process logs**, including 972 Sed invocations. Together with the 56 focused
comparisons, `evidence/sed-multicall-validation.json` records **1,048 clean
process logs** on the same candidate. Native GNU findings remain preserved.
Seven exclusions and the two separately recorded test profiles are outside
this standard suite. Full GNU acceptance and release
activation are still pending.

The preceding Sed checkpoint passes all **56 focused comparisons** and
the unchanged `execute-tests.sh` original with integrated rboxc Coreutils helpers.
That interoperation profile has **25 clean process logs**: 20 Sed invocations,
four system-shell children and one translated `cat` invocation. The same
original with native GNU helpers retains GNU `cat`'s 262,144-byte allocation
finding. Native GNU Sed findings also remain recorded. The original assertions
and native binaries are unchanged.

`evidence/sed-multicall-execution-audit.json` reparses 212 logs across both
implementations and reports 105 clean candidate-profile processes plus the
retained native-helper finding. `RBOXC_SED_MULTICALL_HELPERS=1` selects the
additional helper profile for `tests/sed-original.py`; its report records the
actual helper executable and hash. Earlier broader Sed original-suite results
remain on their recorded binaries; this checkpoint does not claim full Sed
acceptance or release activation.

The next-provider work is preserved before release activation. Gawk's candidate
uses GNU's `MEMDEBUG` per-object allocation mode; the earlier pooled native and
Rust binaries and their Valgrind observations remain available locally. A narrow
cleanup adapter closes only standard descriptors that GNU replaced with
`/dev/null` and that still identify the same replacement at exit. All 85 focused
comparisons and eleven reviewed original Make targets pass, including strict
candidate memory and descriptor checks. The remaining original inputs are
inventoried for review; Gawk is not certified complete. This configuration adds
GNU's readline dependency.

Seven further unchanged Gawk recipes pass for numeric and string formatting,
separator arrays, record reading, multibyte substitution, process identifiers and
script shebang dispatch. Recipe helpers use pinned GNU implementations; the
substitution recipe uses the verified English UTF-8 profile.
`evidence/gawk-format-file-coverage.json` audits **448 passing original recipes,
three unchanged GNU baseline failures and 560 clean processes**: 463 original
Gawk invocations, twelve shell children and 85 focused cases counted once.
Three auxiliary inputs are already covered by passing recipes; nine exclusions
and 171 pending inputs remain. The candidate and installed release are unchanged,
and full GNU acceptance remains open.

Five preceding unchanged Gawk recipes pass for directory entries, fixed-width
fields, message extraction, POSIX string comparisons and string-length updates.
The directory profile uses the pinned native `readdir` extension and GNU recipe
helpers; both selected AWK programs are instrumented. The two locale recipes use
a private English UTF-8 collection compiled from the pinned glibc 2.43 data and
verified before execution. The first locale probe and its diagnosis are retained
in `evidence/gawk-english-locale-initial.json`.

`evidence/gawk-directory-locale-coverage.json` records **441 passing original
recipes, three unchanged GNU baseline failures and 553 clean processes**:
456 original Gawk invocations, twelve shell children and 85 focused cases.
Three auxiliary inputs are verified as already executed by passing recipes;
they add no duplicate tests or processes. Nine exclusions and 178 inputs awaiting
individual accounting remain. The audit verifies the exact compiled Russian
locale used by earlier reports and explicitly records that the host `locale`
and `localedef` tools have since changed; historical build hashes are retained.
These reports keep their original candidate hashes. Full acceptance and release
activation remain open.

Five preceding unchanged Gawk recipes pass for pipe input/output, expression
precedence and child exit statuses. The driver requires the exact twelve shell
child command headers and invocation counts; both Gawk and its children must
have complete, clean Valgrind summaries. `evidence/gawk-shell-coverage.json`
audits **436 passing original recipes, three unchanged GNU baseline failures
and 547 clean processes**: 450 original Gawk invocations, twelve shell children
and the shared 85 focused cases counted once. Nine exclusions and 186 inputs
awaiting individual accounting remain. The initial command-header classification
attempt is retained in `evidence/gawk-shell-classification-initial.json`; it is
not counted as a passing run. These comparisons retain the exact immutable
Gawk ownership candidate recorded in the reports; release activation remains open.

Eight preceding unchanged Gawk recipes pass for nested and custom sorting,
typed-pattern substitution, shortest matches, byte-locale matching, lexicon
records and repeated file reading. The file-reading recipe runs both original
programs against its unchanged input. `evidence/gawk-pattern-coverage.json`
audits **431 distinct passing originals, three GNU baseline failures and 530
clean candidate processes** on the preserved Gawk ownership candidate: 445
original invocations plus 85 focused cases counted once. Nine exclusions and
191 inputs awaiting individual accounting remain; one of those inputs is the
auxiliary program already exercised by the file-reading recipe. The current
187-command candidate and installed 133-command release are unchanged.

Gawk's preceding argument and `BEGINFILE` originals also pass with their normal build
fixtures supplied privately: pinned GNU `cp` copies the input, and a private
copy of the configured GNU Makefile is read as data. Programs, recipes and
assertions remain unchanged. `evidence/gawk-current-coverage.json` audits all
four preserved checkpoints together: **423 distinct passing originals, three
unchanged failures matching GNU, and 521 clean candidate process logs**.
The memory count includes 436 original invocations and the shared 85 focused
cases once. Historical driver archives and every raw log are hash-checked;
native GNU findings remain recorded. Nine exclusions and 199 inventoried
inputs awaiting review remain separate. Full GNU acceptance is still open.

Thirteen preceding Gawk originals pass: twelve language recipes cover array
indexing and sorting, array arguments, CSV and pattern splitting, typed regex
semantics, formatting and diagnostics. The thirteenth retains GNU's time
extension, fixed date assertions and original 1.3-second sleep tolerance.
`evidence/gawk-language-time-memory-audit.json` verifies all thirteen clean
candidate logs alongside the existing 85 focused comparisons; those focused
logs are shared evidence and are not counted again. Across those preceding profiles,
**421 originals pass and three unchanged failures match GNU**. Nine reviewed
historical crash or memory reproductions are excluded, and that checkpoint left 201 inventoried
inputs for review, including helper fixtures and optional profiles.

Eight preceding Gawk extension originals pass, including filename matching,
file metadata, function-table dispatch and six fixed wrong-argument diagnostics.
`evidence/gawk-extension-expanded-memory-audit.json` verifies 13 clean original
invocations plus the retained 85 focused comparisons on the same candidate.
The extension libraries remain pinned native GNU helpers. Alongside the preceding
language checkpoint, this records 408 passing original recipes and three shared
GNU assertion failures across those preceding scopes; full Gawk acceptance remains open.

The preceding four extension originals pass using the pinned native `ordchr`,
`revoutput`, `readfile` and `filefuncs` extension libraries. Their unchanged
recipes resolve libraries through a private directory preserving GNU's relative
`AWKLIBPATH`. `evidence/gawk-extension-initial-memory-audit.json` verifies four
clean extension invocations plus the 85 retained focused comparisons. These
native extension helpers are separate from the translated Gawk entry and are
not counted as new Rust applets or bundled release libraries.

Original Gawk runs can select named, reviewed targets. A separate regression
check verifies ordinary execution, the private locale and an exact retained GNU
baseline failure under this selection mode. Previous broader reports retain
their original scopes; the 85 shared focused cases are not additional coverage.

Gawk's preceding language checkpoint passes **400 reviewed original Make recipes**. The expanded language
batch covers arrays, field splitting, multibyte diagnostics, date formatting,
regular expressions, numeric conversions and pretty-printing. All 403 reviewed
outcomes are accounted for by `evidence/gawk-language-expanded-memory-audit.json`,
which verifies **493 clean candidate process logs**: 408 original-program
invocations plus the 85 focused comparisons. Multi-invocation recipes retain
separate process logs.

Three original assertions fail identically in pinned GNU and rboxc: `typeof7`,
`elemnew2` and `matchuninitialized`. They remain failures, with their exact
original expected files and all four native/instrumented outputs preserved.
The independent audit can verify a newly discovered shared failure directly
from its original logs without rewriting its report or rerunning unchanged
programs. The other 231 program inputs and extension profiles remain open;
full GNU acceptance is still unfinished.

The preceding Gawk checkpoint passes **328 reviewed original Make recipes**. Two further originals,
`typeof7` and `elemnew2`, fail their unchanged assertions identically in pinned
GNU 5.4.1 and rboxc, with and without Valgrind. The latter prints `0` where its
expected file has an empty final line. Both failures remain separate from passes;
the original assertions are unchanged. The expanded coverage includes arrays,
parameter handling, regular expressions, CSV and pretty-printing.
`evidence/gawk-array-baseline-memory-audit.json` verifies all 330 recorded
outcomes and **415 clean candidate process logs**, including 85 focused comparisons.
The initial array run and its exact driver and outputs are retained. The other
304 program inputs and extension profiles remain open.

The preceding Gawk checkpoint passes **266 reviewed original Make recipes**; one additional original,
`typeof7`, fails its supplied assertion identically in GNU and rboxc. Both print
`test2 <0>` where the unchanged expected file says `test2 <>`. That result remains
an explicit baseline failure, not a passing original or a registered GNU XFAIL.
The new selections cover conversions, Unicode formatting, sorting, profiling,
record handling and a 200 KB substitution fixture. The missing Russian UTF-8
locale is built privately from pinned glibc data and selected only for its test;
the system locale archive is unchanged. Independent original selections now run
with four workers, each retaining a private working directory and raw logs.

`evidence/gawk-locale-baseline-memory-audit.json` verifies all 267 reviewed
selections against their recorded outcomes and **352 clean candidate process
logs**, including the 85 retained focused comparisons. The initial conversion run
and exact baseline output remain preserved. The other 367 program inputs and
extension profiles remain open; full Gawk acceptance is not claimed.

The preceding Gawk checkpoint passes **187 reviewed original Make recipes**, including further CSV
and record handling, namespaces, array and parameter diagnostics, numeric
formatting and profiling. The original programs, input files, expected output
and recipes are unchanged. Together with the retained 85 focused comparisons
on the same candidate, `evidence/gawk-record-memory-audit.json` verifies 272 clean
candidate process logs. The source-ownership contracts remain separately recorded;
other original inputs and extension profiles still require acceptance work.

The preceding Gawk checkpoint passes **105 reviewed original Make recipes** and all 85 focused
comparisons. Three added diagnostic cases exposed parser source descriptors left
open on fatal exit. The reproducible native helper adapter now closes still-owned
source inputs, including the one-line reader's stream, while preserving stdin.
Seven additional contracts verify exact GNU diagnostics and inherited-descriptor
preservation. `evidence/gawk-source-validation.json` audits 197 clean Gawk process
logs and seven separate descriptor inspections; all 428 multicall smoke checks
and 11 dispatcher checks pass. The initial findings remain recorded.

The current 187-command candidate is **14,716,728 bytes**, SHA-256
`6eca6aac2d29295f2d01f1fd797206c278ee8a7a7fb64fdda5f0d0afd0e29a89`.
An independent rebuild is byte-identical. The installed 133-command release is
unchanged. Original inputs beyond these 105 recipes and extension profiles remain
open; the checkpoints below refer to their recorded candidate binaries.

The preceding Gawk checkpoint passes **71 reviewed original Make recipes**, covering further fixed-width
fields, substitutions, typed regular expressions, profiling and expected diagnostics.
The original recipes and expected output remain unchanged. Together with 85
focused comparisons on the same binary, `evidence/gawk-fieldwidth-memory-audit.json`
verifies 156 clean candidate process logs. Remaining original inputs and extension
profiles still require acceptance work.

The preceding Gawk checkpoint passes **47 reviewed original Make recipes**, including additional
field splitting, numeric formatting, symbol-table/array operations, exit status
and expected diagnostic cases. Original recipes and expected output remain
unchanged. Together with the retained 85 focused comparisons on the same binary,
`evidence/gawk-language-memory-audit.json` verifies 132 clean candidate process
logs. Other original Gawk inputs and extension profiles remain open.

The preceding Gawk checkpoint passes **26 reviewed original Make recipes** and all 85 focused
comparisons on the current candidate. The additional originals cover field and
numeric conversions, regular expressions, Unicode whitespace, lint diagnostics,
8 KiB text input and profiling. The runner overrides GNU's `AWKPROG` variable,
retaining the original recipe's locale setup. The initial wrapper failure is
preserved in `evidence/gawk-expanded-original.json`; the corrected run and its
111 clean candidate process logs are verified in
`evidence/gawk-locale-memory-audit.json`. Other original Gawk inputs and extension
profiles remain open.

Wget's 87 registered Perl originals are now accounted for: **71 scripts pass,
14 IRI feature-gate skips match GNU, one web-of-trust script is unconditionally
disabled upstream, and one privacy reproduction remains held out**. The final
HTTPS selections cover certificate rejection/acceptance, revocation and fixed
local proxy authentication with and without connection reuse. Original scripts
and certificates remain unchanged; only the helper log path is private.
The client-certificate script accepts a failed first phase before testing a valid
key: both pinned OpenSSL builds report status 1 there, against the fixture's
internal expected status 5. That phase diagnostic is preserved and matched exactly.
`evidence/wget-certificate-memory-audit.json` verifies 114 clean candidate process
logs across the 86 reviewed selections and 14 focused comparisons. Initial harness
expectation failures remain in the preceding TLS reports.

All **17 configured original helper unit functions** also pass in GNU's separate
native `TESTING` builds, both pristine and with the recorded production HTTP
cleanup adapter. `evidence/wget-unit-memory-audit.json` verifies both instrumented
processes have zero errors, no new descriptors and zero heap bytes at exit.
These native unit executables supply helper evidence, not Rust/multicall entry
coverage. Disabled optional features and the held-out script remain untested.

The preceding Wget checkpoint passes **65 original HTTP/FTP/HTTPS scripts**, with the same 14 original
IRI feature-gate skips. Three unchanged HTTPS originals verify forward-secrecy
and TLS protocol selection using private copies of GNU's localhost certificate
fixtures; only the server helper's log path changes. All 14 focused comparisons
also pass. `evidence/wget-tls-initial-memory-audit.json` verifies 97 clean candidate
process logs, including feature probes. Seven HTTPS originals and the unit-test
profile remain open; one privacy reproduction remains held out.

The preceding Wget checkpoint passes **62 original HTTP/FTP scripts** and matches **14 original IRI
feature-gate skips**. Both pinned builds advertise `-iri`; the unchanged GNU
feature checks exit 77 before creating a server, so optional IRI behavior remains
untested. The added ordinary tests cover HTTPS-only link policy and byte-preserving
`--no-iri` filenames. All 14 focused comparisons also pass, and
`evidence/wget-feature-memory-audit.json` verifies 91 clean candidate process
logs, including feature probes. Ten original HTTPS scripts and the unit-test
profile remain open; one privacy reproduction remains held out.

The preceding Wget checkpoint passes **60 reviewed original HTTP/FTP scripts** and all 14 focused
comparisons. The added FTP originals cover transfer resumption, nested directories,
URL-list input, listing conventions, hidden files and interrupted passive setup
using fixed localhost fixtures. `evidence/wget-ftp-memory-audit.json` verifies
74 clean candidate process logs. One original privacy reproduction is held out;
HTTPS, optional IRI and remaining original profiles are still open.

The preceding Wget checkpoint passes **47 reviewed original HTTP scripts** and all 14 focused
comparisons. Coverage includes timestamp updates, response filename policy,
recursive downloads and link conversion, no-parent/nofollow rules, cookies,
local proxy authentication and URL-list input. The fixed fixtures use only local
servers. `evidence/wget-http-memory-audit.json` verifies 61 clean candidate
process logs; the preceding 32-script timestamp checkpoint is retained in
`evidence/wget-timestamp-memory-audit.json`. FTP, HTTPS and other original
profiles still require acceptance work.

The preceding Wget checkpoint passes **16 reviewed original scripts** and all 14 focused comparisons
on the latest candidate. New originals cover download resumption, byte ranges,
uppercase output names, unavailable URLs, missing upload input and write errors.
They retain GNU's original Perl assertions and fixed localhost servers.
`evidence/wget-resume-memory-audit.json` verifies 30 clean candidate process logs.
Other original Wget scripts and unit-test selections remain open.

Patch's 49 registered original scripts are now accounted for: **38 pass, two
match GNU's registered expected failures, and nine mixed reproduction scripts
remain held out**. The 40 reviewed selections include merge conflicts, timestamps,
Ed-format input and all three read-only policies. Read-only tests use uid/gid
65534 with private byte-identical executable copies. Ed-format tests retain
GNU's configured external `/usr/bin/ed` dependency (GNU Ed 1.22.4); that helper
is separate from the integrated Ed entry.

The expanded tests found three issues also visible in native GNU: an abandoned
output stream on syntax-error exit, a descriptor created when restoring stdin
after an Ed script, and an uninitialized merge search limit. The Rust entry now
tracks and releases pending output ownership. Native cleanup finalizes the
restored stdin descriptor only while its recorded identity still matches, and
the merge helper initializes its search limit using the pinned GNU Diffutils
3.12 policy. Original patch application logic and test assertions are retained.
The earlier 33/35 and 37/40 results remain in `evidence/patch-merge-original.json`
and `evidence/patch-output-ownership-original.json`.

`evidence/patch-merge-initialization-memory-audit.json` verifies **201 clean
candidate Patch process logs**, 23 focused comparisons, 428 smoke checks and
11 dispatcher checks. The new 187-command candidate is 14,716,368 bytes and its
independent rebuild is byte-identical. The installed 133-command release remains
unchanged; broader GNU acceptance is still open. Each provider checkpoint
records its own tested candidate hash.

The preceding Patch checkpoint passes **24 reviewed original scripts**, and exactly matches the two
failures that GNU Patch 2.8 registers in `XFAIL_TESTS` (`context-format` and
`dash-o-append`). Those two retain their failing assertion counts and identical
GNU/candidate output; they are not counted as ordinary passes. All 23 focused
comparisons also pass. `evidence/patch-registration-memory-audit.json` verifies
89 clean candidate Patch process logs. The initial 15/17 strict run is preserved
in `evidence/patch-expanded-original.json`. The diagnostic-name test instruments
the unchanged shell driver so GNU's expected executable path stays intact; native
test-helper logs are retained separately. Other original scripts remain open.

Cpio's original-suite coverage now includes valid symlink archive round-trips,
long target names and `--to-stdout` extraction. All **13 reviewed original
selections** and all 52 focused comparisons pass on the current 187-command
candidate. `evidence/cpio-symlink-memory-audit.json` independently verifies 104
clean candidate process logs. The 17 registered original selections are now
accounted for as 13 passing and four explicit reproductions left unexecuted;
none of those four is counted as passing. Tape-device and broader platform
profiles remain open.

The latest Bash cleanup checkpoint passes **all 17 reviewed complete original
recipes**, with 606 clean candidate process logs. The original GNU expected
outputs remain unchanged. All 44 focused Bash checks, 69 shell-adapter/updatedb
checks, 428 smoke checks and 11 dispatcher checks pass too.
`scripts/audit-bash-original.py` rechecks original outputs, reparses memory
summaries and binds 2,401 raw files in
`evidence/bash-unwind-ownership-validation.json`.

The C2Rust Bash entry is unchanged. GNU helper adaptations now release function
subshell payloads, expansion buffers and words abandoned during error handling,
and command trees discarded by the parser. The four fixed tilde tables use
static storage. Tilde testing uses glibc's file-based passwd provider for both
implementations through `tests/bash-nss-files.c`; this test helper is absent from
the product. Build it with
`gcc -O2 -g -fPIC -shared -Wall -Wextra -Werror tests/bash-nss-files.c -o build/bash-test-helpers/nss-files.so`.
The host SSSD finding and ineffective cache-disable attempt remain recorded in
the earlier reports. The broader host-NSS profile remains open.

The candidate is `/root/rboxc/target/bash-unwind-ownership-candidate/release/rboxc`,
14,715,648 bytes, SHA-256
`1715ea3348e9e5f6608d9e6a8e6581c00206e2ceb8b6445544f06db7649345cb`.
Its independent rebuild is byte-identical. This closes the three reviewed Bash
originals previously left open, but 71 other top-level recipes and broader GNU
acceptance remain open. The installed release and deferred commands are unchanged.

The preceding candidate closes an abandoned `iconv` temporary spool when GNU's
conversion-error branch skips output flushing to preserve overlapping input.
All three unchanged GNU 2.43 buffer recipes pass: default, one-byte buffer,
and the original large-file mode with 22 input-size doublings. Each recipe
executes 59 conversions in each native/Valgrind mode. The 50 focused glibc
utility checks, 428 smoke checks and 11 dispatcher checks also pass.
`scripts/audit-iconv-original.py` verifies 227 clean candidate process logs,
original inputs and raw evidence in `evidence/iconv-spool-validation.json`.
The initial failures remain in `evidence/iconv-buffer-reviewed-original.json`.

This 187-command candidate is `/root/rboxc/target/iconv-spool-candidate/release/rboxc`,
14,714,024 bytes, SHA-256
`5b48b3ff8338534e81a8357cff156f3ef1149851a2e9a13e57eb8db4467993d2`.
An independent rebuild is byte-identical. The installed 133-command release
and 354 deferred commands remain unchanged. Other iconv inputs, charmaps,
provider originals and platform profiles remain open.

The preceding batch of four Bash originals (`appendop`, `ifs`, `nquote4`, `quote`) pass
with clean candidate Valgrind process trees in
`evidence/bash-quoting-original.json`. All six scripts in this batch match GNU
expected output, but `braces` and `nquote1` expose further owned-memory cleanup
gaps and remain open. Together with the preceding originals, 14 of 17 reviewed
recipes pass strict memory checks. GNU's fixed `zecho` test helper is built with
`make -C build/gnu-bash zecho`; it is shared by oracle and candidate fixtures and
is not counted as a port. The inventory still contains 88 top-level recipes.

Two more complete Bash originals, `herestr` and `rhs-exp`, pass against GNU's
expected output with clean Valgrind process trees on the same ownership
candidate. This brings the reviewed passing original scripts to ten. The
additional `tilde` script matches GNU output, but remains open: both the native
GNU shell and candidate lose a 24-byte tilde table, and the host's
`libnss_sss.so.2` reports an invalid descriptor during name lookup. The raw
findings are retained in `evidence/bash-expansion-original.json`; they are not
counted as passes. Eleven of the 88 top-level recipes have now been reviewed.

The preceding Bash ownership checkpoint passes **eight complete reviewed original
scripts** (`arith-for`, `attr`, `case`, `casemod`, `invert`, `precedence`, `shopt`,
`strip`) with clean Valgrind process trees. This includes every nested dependency
of the selected scripts. It also passes 44 focused Bash checks, all 69 shell
adapter/updatedb checks, 428 Coreutils smoke comparisons and 11 dispatcher checks.
The 187-command executable is **14,713,960 bytes** (14.03 MiB), SHA-256
`c45278c9081eb9b94ee46d177c62d9a46bb9aae7f488a87659095abd193673ba`;
an independent rebuild is byte-identical.

The additional cleanup owns temporary expanded word lists across error recovery,
releases reader commands and process-substitution paths abandoned by interpreter
restart, and frees replaced completion strings and failed increment/decrement
values. Redirection backup ownership is forgotten after every returned Linux
close result, including late errors. Seven native/Valgrind contracts verify
owned exit cleanup, errno preservation and reuse of the same descriptor for the
same file. `evidence/bash-ownership-validation.json` binds the exact final build,
source and process evidence; intermediate findings are retained separately.

The original Bash `recho` test helper is built with
`make -C build/gnu-bash recho`, pinned in `inventory/bash-tests.json`, and shared by
both implementations. It is a test dependency and is not counted as an applet.
The same manifest still accounts for 80 other top-level recipes without claiming
passes. Whole scripts containing explicit reproduction inputs remain unexecuted;
ordinary selections require separate accounting. Broader Bash/GNU acceptance,
interactive profiles and release activation remain open. The 354 deferred
provider assignments are unchanged.

The preceding Bash original-suite cleanup checkpoint passes all **five reviewed
original scripts**, including strict Valgrind checks for the complete candidate
process trees. It releases rejected arithmetic-for syntax trees and assignment
strings abandoned by arithmetic error recovery, and tracks redirection backup
descriptors so command-substitution children close their own copies at exit.
Normal closes remove ownership records. The test driver now selects integrated
sed/grep helpers for the candidate and pinned native GNU helpers for the oracle;
the earlier native-helper findings remain preserved.

This 187-command candidate is 14,713,248 bytes, with SHA-256
`c307e24a374127a40faf4a2e3f0d6fdd15708207b84ae50bda063c6892214ab6`.
It also passes all 44 focused Bash checks, 69 shell-adapter/updatedb checks,
428 Coreutils smoke comparisons and 11 dispatcher checks. An independent rebuild
is byte-identical. `evidence/bash-original-cleanup-validation.json` records the
exact source and raw-evidence hashes. The other Bash originals and broader GNU
acceptance remain open; none of the deferred commands has been reassigned.

The preceding Bash restart-cleanup checkpoint remains a **187-command candidate** and now
passes **44/44 Bash comparisons** with clean per-process Valgrind results, plus
**69/69 shell-adapter comparisons**, including all eight updatedb cases. Its
14,711,648-byte executable has SHA-256
`40d34f468100d784c800ad761e3add7297b2a7bd1d75653f356503ad5bc5b31f`;
an independent rebuild is byte-identical. Coreutils smoke checks pass 428/428
and dispatcher checks pass 11/11 with the complete candidate provider list.
The first dispatcher run used the installed 133-command expectation and is
preserved as a 9/11 configuration mismatch.

Bash's C2Rust entry is unchanged. Adapted GNU helpers now release restart caches,
dynamic associative snapshots, owned argument vectors, discarded unwind payloads
and expansion buffers. Normal unwind callbacks retain their original behavior;
exec restart releases specified payloads before the nonlocal transfer without
restoring abandoned frames. Trap-string disposal preserves ignored signal
dispositions. Owned script inputs and standard-descriptor replacements are
finalized on normal exit, and redundant pipeline closes retain GNU's errno and
return value. The assembly driver records adapted source/object hashes and can
recover missing configured object compile records without relinking the oracle.

The first five reviewed Bash originals (`arith-for`, `casemod`, `invert`,
`precedence`, `strip`) all match their original expected output natively and under
Valgrind. Two pass the strict whole-process memory/descriptor gate. The remaining
findings include arithmetic/parser cleanup, inherited redirection backups in
command substitutions, and native GNU sed/grep helper allocations. The initial
reports retain these findings. `inventory/bash-tests.json` inventories 88
original top-level recipes; 83 remain unreviewed, and nested standalone inputs
and interactive profiles remain open. This is acceptance progress, not full
Bash or GNU-provider completion. `evidence/bash-restart-cleanup-validation.json`
binds the build, source changes and raw validation evidence. The installed
133-command release and deferred-provider assignments remain unchanged.

The preceding source checkpoint built a **187-command candidate**: all **54 of 54**
previously queued names now compile, with **zero integrations remaining** in that
queue. This is integration coverage, not full acceptance. The candidate is
14,708,688 bytes (14.03 MiB); its SHA-256 is
`257c7fadad46e70c7e9694082a2f7912e552dc62e3e8b3881644c9c6181bf456`. An independent rebuild is byte-identical.
The final encoder cleanup passes all ten frcode checks under native execution
and Valgrind, plus 428 smoke and 11 dispatcher checks. All eight updatedb
comparisons still match GNU. `evidence/all-integrations-validation.json` records
these results and hashes the source files and preserved process logs.
All 22 selected combined-regression jobs pass on the preceding 187-command
candidate, along with
318 saved instrumented Coreutils comparisons. The separate shell adapter checks on that build
pass all 61 builtin/conditional cases. Eight updatedb cases match GNU, including
LOCATE02/slocate output and database reads, with strict Bash pipeline findings
still open. The initial host-tool oracle failures are preserved; the final oracle
uses pinned GNU sort and other pipeline tools. The 32 Bash cases retain 27 strict
passes and five baseline findings. `evidence/gnu187-regression-summary.json`
records the actual combined-build results. All 88 generated gate/result structure
layouts match C; the native and instrumented recovery contract verifies normal
return, errno, both jump buffers and signal-mask restoration. Rebuilding the
separate C lowering oracle also produces an identical executable.

The last additions are the four Inetutils entries, all 28 Bash shell/builtin
names, and Findutils `updatedb`. The four Inetutils entries pass 36 focused
native/Valgrind comparisons on their 158-command candidate. Their actual Rust
entry logic is retained; narrowly outlined nonlocal recovery loops stay in C.
Bash's translated entry retains the configured GNU main decisions, with four
recovery checkpoints and 87 C call boundaries returning explicit outcomes.
Signals are deferred during Rust computation and restored to GNU's logical mask
at each C call. All 32 initial Bash cases match original GNU and a separately
lowered C executable, including recovery and no-shebang execution; 27 pass the
strict memory/descriptor gate. The five open cases reproduce findings in original
GNU too. Full Bash original suites, interactive job control, loadable builtins,
and broader signal profiles remain open.

Standalone Bash builtin adapters execute one `builtin NAME "$@"` in a fresh
shell process; changes to cwd, variables and jobs belong to that process.
The `[[` adapter accepts literal argv operands and explicit conditional operator
tokens, with an optional final `]]`. Pattern and regex right operands retain GNU
expansion semantics; operands are passed through positional parameters. Invalid
adapter grammar returns status 2 with an adapter diagnostic. Full shell syntax
remains available through `bash -c`. A command named `.` uses multicall dispatch,
since a filesystem symlink cannot have that basename.

`updatedb` embeds GNU Findutils 4.11.0's shell program, using ordinary GNU
configure substitutions and the linked translated Bash interpreter. Its private
`frcode` helper is a C2Rust translation and is excluded from the public command
count. Private symlinks select this executable's find, sort, sed and other
pipeline commands. Explicit GNU environment overrides remain supported. The
default database is `/var/lib/rboxc/locatedb`; its parent must already exist.
The user-switch helper `su` remains external and that profile is untested.
No external updatedb executable is counted as a port. C helpers own adapter
arguments and remove the private command directory on the owning process's normal
exit; forked children cannot remove their parent's directory.

After preparing the pinned native providers, the final entries can be regenerated
with the following commands. The private encoder preparation refreshes its exact
GNU compile and link records, including recovery from older record formats.

```sh
python3 scripts/prepare-frcode.py
for command in tftpd tftp ftpd telnet bash frcode; do
    python3 scripts/translate-entry-provider.py "$command" || exit
    python3 scripts/assemble-entry-provider.py "$command" || exit
done
python3 scripts/embed-updatedb.py
CARGO_TARGET_DIR=target/regenerated-integrations cargo +nightly-2026-01-22 build --release
cp target/release/libstdbuf.so target/regenerated-integrations/release/libstdbuf.so
```

The previously validated source checkpoint built a 154-command candidate.
The installed release remains at 133 commands pending activation. The previous combined
152-command build passes 428 Coreutils smoke checks, 107 instrumented help
checks, all 318 native/instrumented behavior comparisons and 11 dispatcher
checks. Focused regressions for every installed additional provider and Gawk
also pass, along with all 137 selected newer-entry comparisons. Gawk and Patch
reviewed originals pass again on this exact candidate. `evidence/gnu-assigned-progress.json` records these separate states.
No new full-provider completion is claimed. The 149-command intermediate
candidate passes all 108 selected native/Valgrind comparisons. Binutils tests
use valid compiler-produced ELF objects and fixed input mtimes. Archive-index
creation timestamps are checked against each invocation's time window, with
deterministic archives requiring zero; raw archive bytes remain in the evidence.
The enum bitfield adapter delegates to the original integer representation, and
Inetd's dispatcher adapter passes the process environment to GNU's entry.

Patch's eight reviewed original scripts pass all 54 assertions in each execution
mode. Its strict audit reparses 50 clean candidate process logs across the 23
focused cases and 27 original invocations. Cleanup closes its patch input,
unused temporary output descriptors and directory cache, and frees queued
outputs on normal exit while preserving the signal-handler path. Ifconfig
closes the socket used to enumerate interfaces. Original observations remain
preserved. The current Inetutils runtime evidence covers option parsing and
read-only listing, with service behavior and full suites still open.

Screen uses GNU's `--disable-pam` configuration because PAM headers are absent;
PAM integration is untested. Its build date is fixed through
`SOURCE_DATE_EPOCH=1700000000`. Wget uses GNU's OpenSSL configuration and its
native fallback cookie checks; GnuTLS and libpsl are absent from this profile.
The 152-command candidate is 12,675,832 bytes (12.09 MiB), and a separate
build is byte-identical. Screen now skips descriptors already known to be
closed before GNU's poll probe; its five focused option checks are clean, and
an isolated contract verifies ten GNU/adapted outcomes for preserved standard
and exception descriptors. Wget closes its owned persistent HTTP connection at
exit, retaining reuse during normal operation. Its 14 focused cases include
private HTTP/1.1 reuse, and four unchanged GNU Perl tests pass; the strict audit
reparses all 18 clean candidate process logs. Full Screen, Less and Wget suites
remain open. Earlier findings and candidate binaries are preserved.

The glibc baseline matches the host's 2.43 release and is a fixed port baseline,
not a claim about the latest available upstream version.

The glibc utility candidate is 12,758,424 bytes (12.17 MiB), an increase of
82,592 bytes over the validated 152-command build. A separate build is
byte-identical. Both command entries are translated Rust; getconf needs no
native command helpers and iconv retains nine GNU helper objects. The process
still loads the existing host `libc.so.6` and conversion modules. The local
full glibc build only prepares authoritative utility objects and headers; its
CRT objects, libc archives and dynamic loader are excluded from Rboxc's link
inputs. Binutils supplies only `ar`, `readelf` and `strings` in this inventory;
its assembler and linker are build tools, not additional Rboxc applets.

The utility adapters bind libc invocation names to dispatched argv storage and
register iconv's GNU version callback with libc's argp parser. GNU's supported
error-prefix callback preserves full invocation paths. Iconv closes successful
encoding probes, its owned conversion descriptor and output buffer, and the
print-list nodes after their final use. The print-list strings remain owned by
libc. All 50 focused native/Valgrind comparisons pass, and both unchanged GNU
getconf tests pass in all four modes. The strict audit reparses 242 clean
candidate process logs. Thirteen iconv interface structures also match pinned
GNU C sizes, alignments and field offsets. These results cover the host glibc
2.43 profile; iconv originals, charmaps and other profiles remain open. The
initial diagnostic and allocation findings are retained. All 20 selected
combined-regression jobs now pass on this 154-command build: 428 Coreutils
smoke checks, 107 instrumented help checks, 318 native and instrumented
behavior comparisons, 11 dispatcher checks, 137 newer-entry comparisons, and
the focused checks for every installed additional provider and Gawk.
`evidence/gnu154-regression-summary.json` verifies the completed report counts
and exact candidate hashes. Earlier original-suite results retain their actual
candidate hashes; they are not represented as reruns on the new binary.

For a later upstream fix, identify its upstream commit or patch and the release
baseline in this table. Record the affected commands, upstream reference, local
commit, and validation evidence in this README when applying it. Keep the
baseline version unchanged for a backport, and record that backport explicitly.
For a release upgrade, update the version and archive pin together, rebuild the
matching native oracle, regenerate the translation, and rerun the affected
original and compatibility tests. Preserve the previous evidence with its
actual source and binary hashes.

Existing local translation and ownership adaptations live in the provider
`scripts/translate-*.py` scripts and shared/provider `scripts/*cleanup.py`
helpers, with incremental Git commits and evidence recording their changes.
They must be reviewed when rebasing onto a newer GNU release; the version table
alone does not imply an unmodified upstream implementation. GNU ACL and Attr
have not yet been source-pinned or incorporated as providers.

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
125-command executable. Its native helpers and Rust-owned diff state share 369
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
125-command executable. The matcher helpers and Rust-owned state use 338 private
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
Valgrind on the retained 117-command build: 93 complete scripts and 11 selected diagnostics from one Perl script.
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
`evidence/grep-activation.json`. The 125-command build passes all 68 fresh
focused comparisons. Its Grep source, native helper objects, and compiler inputs
are verified unchanged; the original results retain their actual earlier
binary hash and are not counted as a new original-suite run.

The unchanged Gzip empty-suffix original now passes as well: it rejects the
invalid option and preserves the compressed input.
`evidence/gzip-empty-suffix-audited.json` verifies both candidate processes
under Valgrind. This adds one full original on the current Tar map/probe
candidate; earlier Gzip results retain their own binary hashes.

GNU Gzip 1.14 is also pinned from its verified signed archive. Its native
oracle and compiler records are prepared for the `gzip`, `gunzip`, `uncompress`,
and `zcat` inventory entries. The latter entries use upstream shell adapters
and an installed alias; their compatibility work is separate from Grep.
The 30 original Gzip registrations remain inventoried for individual review.
The installed 125-command executable includes the translated C entry,
three internal shell-alias adaptations, and 170 private helper/state symbols.
Its three GNU input/output/window buffer alignments are retained and verified
in the executable. All 56 focused comparisons now pass, including compression,
decompression, file metadata, internal aliases, and output errors. Exit cleanup
releases the directory cache and unfinished input ownership; alias write-error
messages retain the configured Bash profile and pinned script line numbers.
Seventeen reviewed original selections pass natively and under Valgrind:
sixteen full scripts, including the 4 GiB size and unprivileged write-error
tests, plus the help/version script scoped to the three configured ported
programs. The strict audit verifies 235 complete, clean candidate process logs.
The combined Coreutils and prior-provider checks pass within their recorded
scopes. `evidence/gzip-activation.json` records activation and retained
artifacts. Auxiliary shell programs and the remaining originals stay open.

GNU Sed 4.10 is pinned from its verified signed archive. Its native oracle
and compiler records are prepared with SELinux explicitly disabled. The
75 original shell/Perl registrations are inventoried for individual review.
The translated entry is active in the 125-command release with 301
private helper/state symbols and no native command entry. All 56 focused
comparisons match GNU and have clean Valgrind results, including regex reuse,
partial compilation, directory input errors, and in-place edits. Ownership
tracking invokes GNU's regex destructor and closes remaining registered streams
through the existing exit callback. The initial 29/51 memory-clean result and
candidate are retained. The first seventeen originals pass natively and under Valgrind. The next
22-script batch records 17 clean passes, four assertion passes with retained
empty replacement allocations, and one O_TEXT platform skip. A further
cleanup tracks every replacement allocation base, including zero-byte
allocations: all four affected originals now pass with clean Valgrind results.
The next cleanup releases temporary multibyte transliteration lengths. All 56
focused checks pass on that candidate, as do the three affected locale originals
and the original 2 GiB substitution test. Both full Perl suites (57 miscellaneous
and 198 debug cases) passed on the retained preceding candidate. Original tests
use seven isolated locales and pinned native locale probes. Native GNU cat,
touch, sleep, and dd replace host test dependencies whose dispatch failed under
traced Valgrind execution. Six prerequisite checks now pass their assertions;
five have clean all-process memory results. The sixth retains a 262,144-byte
allocation finding in the native GNU cat child, recorded separately from Sed.
All 66 reviewed original selections are now consolidated on the multibyte
candidate: 64 strict all-process passes, the native-cat child finding above,
and one Linux O_TEXT platform skip. Sed itself has clean memory results in all
65 assertion-passing selections. The audit retains every traced process finding
and command image. Two originals still need dedicated fixture/instrumentation
profiles; seven are excluded. A fresh run on the activated 124-command release
reproduces these results.

GNU BC 1.08.2 is pinned from its verified signed archive. Both `bc` and `dc`
entries are active in the 125-command release. Each calculator retains
its own native arithmetic helper copy and symbol namespace (283 private
symbols total), because their standalone sources define overlapping helper
names. Optional Readline/libedit support is disabled in the recorded native
profile. All 44 focused calculator comparisons now match GNU with clean
Valgrind results. Exit cleanup closes BC's owned scanner input on fatal read
errors and explicit quit, while preserving inherited stdin. The initial
41/42 memory-clean result is retained. The distribution registers no automated
runtime suite. All 19 reviewed historical arithmetic inputs now match the
pinned native calculator with clean Valgrind results, including the original
Signum integer-remainder assertions. Its initial math-library invocation changed
the scale and failed those assertions on both implementations; the corrected
default-scale invocation and earlier observations are retained. The original
definition-only testfn input has a declared bounded invocation. The timing
wrapper's ten input files are covered individually; the wrapper itself is not
counted as another test. The combined checks pass within their recorded scopes;
`evidence/bc-activation.json` records activation and retained artifacts.
Four controlling-terminal profiles also pass with clean Valgrind results: BC/DC
arithmetic, bounded BC_ENV_ARGS math options, and BC's SIGINT handler after an
observed output marker.

GNU Ed 1.22.6 is pinned from its verified signed archive. Its Rust command
entry is active in the 125-command release with 89 private helper/state
symbols. A small wrapper adapts dispatcher arguments to Ed's const argv type.
All 44 initial focused observations match GNU, with their original memory and
descriptor findings retained. Parser storage now remains valid through exit,
and cleanup invokes GNU's parser and initialized scratch-buffer destructors.
All 44 focused checks pass with clean Valgrind results after cleanup. The full
original check script and all 90 editing/diagnostic input files have been reviewed.
Every assertion passes on native GNU and the candidate, both normally and under
Valgrind. The full run exposed a temporary filter-command allocation retained
after use; freeing it after write_file returns leaves all 242 Ed process logs
clean. Thirteen external child findings remain explicit: native GNU cat/sort
allocations and shell descriptor observations during redirection. The earlier
241/242 editor-memory result is preserved. The combined checks pass;
`evidence/ed-activation.json` records activation and retained artifacts.

GNU Findutils 4.11.0 is pinned from its verified signed archive and built with
SELinux disabled. The translated find, xargs, and locate entries compile in a
validated 128-command release. Their native helpers have separate command
namespaces (1,465 symbols), and their C entry objects are excluded. The native
build retains GNU's libm linkage. The original shell/DejaGNU inventory contains
240 test and harness inputs under individual review. DejaGNU is installed
as a test prerequisite. At that checkpoint, updatedb and its private frcode
encoder were separate porting work; both are now integrated in the 187-command
candidate described above. The three earlier translated commands are installed.

The Findutils candidate passes 68/68 focused behavior and Valgrind checks.
The preserved baseline matched behavior but passed only 23/68 strict checks;
cleanup now releases saved directories, samefile references, execdir storage,
xargs input files and replacement lengths, and locate search buffers and
names. Diagnostics preserve GNU's full invocation pathname. A separate helper
comparison verifies unchanged descriptor callbacks, O_PATH handling, early
return, and errno while avoiding polls of known closed descriptors. The first
nine unchanged DejaGNU selections pass (22 original assertions in all four
GNU/Rust/native/Valgrind profiles); expanded original tests are in progress.
The expanded pass now covers 57 original selections and 139 assertions.
A further ownership pass handles repeated database searches, partial regex
compilation, and failed xargs children with argument files; all 81 focused
checks pass on that candidate. Both earlier candidates and reports remain
available. Additional reviewed original selections are running.
All 66 supplemental original selections pass their assertions; 62 pass strict
checks for every traced process. Two retain system-shell file descriptors,
one exposed final execdir-batch ownership, and one reached a host NSS module
that probes an invalid descriptor. The execdir and early-output-file cleanup
fixes pass an expanded 83/83 focused profile. The original NSS case now passes with a private local-files configuration.
The final candidate passes all 83 focused comparisons and all assertions in
233 reviewed original selections (746 assertions in each GNU/Rust/native/
Valgrind profile), including every registered xargs and locate test. A fresh
log audit verifies 982 clean Findutils processes. Strict all-process results
are 230/233: three selections retain 488 findings in traced native shells and
helpers. These remain explicit in `evidence/findutils-original-memory-audit.json`.
Four historical Find reproductions are excluded and unexecuted; the three
harness inputs are separate. The 7,200-file exec-nogaps selection instruments
Find itself but does not instrument its pinned native exec children; other
selections trace children. The permission and large-exec profiles run as
uid/gid 65534 in private owned trees. An earlier 180-second harness timeout
remains an unsuccessful attempt, followed by a completed longer profile.

The final 128-command candidate also passes all combined regressions: 428
help/version comparisons, 107 Valgrind help checks, 318 behavior and
instrumented-equivalence checks, 11 dispatcher checks, all prior-provider
focused profiles, and four BC terminal profiles. The hash-verified activation
in `evidence/findutils-activation.json` retains the previous 125-command binary
and reports. Original suites from earlier providers retain their actual
binary hashes under unchanged-input validation and fresh focused checks.
Findutils' excluded originals and external-child findings still prevent a
full-provider completion claim. Updatedb/frcode integration is now complete,
with updatedb's strict shell-pipeline findings recorded separately.

GNU Tar 1.35 is pinned from its signature-verified GNU archive. The native
reference builds with SELinux disabled and ACL/xattr support retained. C2Rust
has translated its entry with 90 helper imports and 105 Rust definitions.
An isolated Rust type check passes against the root lockfile dependency
versions. Tar now links in a separate 129-command candidate (4,445,984 bytes),
with 845 private helper/state symbols and no native C entry. The initial Rust
run matches 26/30 focused behaviors and passes 6/30 strict checks. Four
diagnostic-prefix differences and memory/descriptor ownership findings remain
open in that preserved baseline; the 128-command installed release is unchanged.
The next candidate preserves full invocation-path diagnostics and frees the
default-settings help string after copying it into the obstack. All 30 focused
behaviors now match GNU, with 11/30 strict checks passing. Remaining findings
concern native helper ownership of directory handles and file-selection records.
The following ownership candidate releases those resources while preserving
all 30 behaviors; 28/30 strict checks pass. The two remaining comparisons
lose the allocation base of GNU Tar's page-aligned comparison buffer.
Retaining and freeing that base produces 30/30 strict focused passes.
The first three unchanged Autotest selections pass their assertions, including
all five archive formats in the append test. Version passes Valgrind; append
and exclusion tests expose additional ownership work in old-style arguments
and TAR_OPTIONS processing. These original findings remain open in
`evidence/tar-original-reviewed.json`. The option-ownership candidate now passes
44/44 focused checks and all three initial originals, with 31 clean
instrumented Tar processes. Old-style argument allocations and environment
option strings are retained until exit; consumed wordsplit nodes are freed
after their text is copied. These are local adaptations against GNU Tar 1.35,
including its bundled `lib/wordsplit.c`, whose source hash is pinned separately.
One additional environment-option fixture unexpectedly aborted in the native
reference. Its observation and an interrupted follow-up batch are retained
and unassessed; it is excluded from further execution. Eight additional reviewed
option/positional original tests also pass. The 11 completed original
selections contain 53 clean instrumented Tar processes in total; broader
original coverage remains open. The next eight originals pass their assertions;
seven pass strict checks and the add-file selection exposes borrowed option
strings read from file lists. Keeping those words until exit fixes that
selection while retaining 44/44 focused passes. A fresh 24-selection original
batch, including five more ordinary file-list tests, now passes on that
candidate. The consolidated report `evidence/tar-original-file-options-audited.json`
verifies 275 clean Tar process logs. Ten more unchanged originals now pass,
covering deletion, matching, extraction overwrite choices, directory symlinks,
and standard-output extraction. The combined 34-selection audit in
`evidence/tar-original-extraction-audited.json` verifies 474 clean Tar processes,
using preserved driver bytes to validate earlier observations. All 44 focused
checks, 11 dispatcher checks, and 428 Coreutils help/version comparisons pass
on these same bytes. This candidate is 4,448,560 bytes (4.24 MiB)
and remains separate from the installed release; broader original and combined
provider coverage are still open. Earlier candidate results keep their actual
binary hashes. The following 15 originals all match GNU assertions; 14 are
strictly clean, while the closed-input label test identifies a standard
replacement descriptor left open. The new `tar-standard-stream-cleanup`
candidate finalizes only the standard streams opened by GNU `stdopen`, retaining
inherited streams and errno. A first raw-descriptor cleanup attempt exposed
libc's later buffered-output flush; both that observation and the corrected
stream finalization are retained. The resulting candidate is 4,449,392 bytes
(4.24 MiB), with 52/52 focused comparisons clean, including eight closed-stream
cases. Ten unchanged permission and directory-metadata originals also pass as
uid/gid 65534 using byte-verified private executable copies. The fresh combined
audit, `evidence/tar-original-standard-stream-audited.json`, now validates
59/59 unchanged original selections and 754 clean instrumented Tar processes
on these same bytes. This is reviewed coverage, not full Tar certification;
broader originals and other-provider regressions remain open. The prior
candidate additionally passes 107/107 Coreutils Valgrind help checks and
318/318 behavior checks; those reports retain its earlier binary hash. Fresh
checks on the corrected candidate now also pass 428/428 Coreutils help/version
comparisons, 107/107 Valgrind help checks, 318/318 behavior checks, 318/318
instrumented equivalence assessments, and 11/11 dispatcher checks. A 30-case
archive-operation harness matches GNU Tar against itself in every comparison;
only 9/30 native control cases are strict memory-clean, with the other GNU
findings preserved as baseline observations. This control is not Rust port
evidence. Recursive discovery now inventories all 237 original `.at` inputs,
including the checkpoint and Star subdirectories previously absent from the
inventory. Existing reviews are preserved; eleven further selections have
source reviews recorded at that stage. Those eleven subsequently match GNU
assertions; nine are strictly clean and two expose an incremental-snapshot
obstack and an index-file stream left open. The `tar-snapshot-index-cleanup`
candidate releases the snapshot parser workspace after its copied records are
no longer needed and finalizes its owned index stream at exit, preserving GNU
output-error status behavior. It passes 57/57 focused comparisons, including
five index-file normal/error cases, 11/11 dispatcher checks, and 17/17 unchanged
originals covering checkpoints, output routing, and all exclusion-tag variants.
The initial new-original audit verifies 76 clean Tar processes. The completed
combined audit, `evidence/tar-original-snapshot-index-final-audited.json`, now
verifies all 76 selected originals and 830 clean Tar processes on this candidate.
Preserved driver bytes support auditing reports that predate a formatting
change, and incomplete batches are explicitly rejected. The prior reports keep
their actual binary hashes. Source reviews for the next multi-volume-label and
owner/group originals are recorded without claiming execution. This candidate
is 4,449,352 bytes (4.24 MiB) and is not installed; broader originals and current
combined-provider validation remain open. The parser explicitly supplies the C17 spellings
for boolean and static-assert constructs detected by the native C23 build.

The next `tar-archive-exit-cleanup` candidate passes 57/57 focused checks and
eight further unchanged GNU originals: multi-volume labels, explicit archive
owner/group metadata, label rejection during append, and four top-level
extraction/listing variants. It releases temporary PAX continuation metadata on
read and write, and closes owned local archive handles on fatal exit while
invalidating every explicitly closed handle. The owner original uses files-only
NSS in a private mount namespace; its earlier host SSSD diagnostic remains in
the baseline report. Earlier allocation and descriptor findings are preserved.
Twenty-one additional GNU Tar groups pass for incremental archives, timestamp
restoration, directory changes, concatenation and renamed directory trees.
`listed03` runs without root privileges. GNU's original `ckmtime` helper was
missing in the first attempt, so fifteen groups skipped their main assertions;
that run is preserved. The original timestamp and sparse-file helpers now build
and pass their own native and Valgrind checks. The completed unchanged rerun has
207 clean Tar processes, audited alongside the earlier 13 groups in
`evidence/tar-pending-current-audited.json`: **34 groups and 401 clean Tar
processes on the current 187-command candidate**. Native GNU findings remain
separate in `evidence/tar-incremental-recovery.json`.

Thirty-two further unchanged groups pass for update behavior, long names,
timestamps, option syntax and removal of archived input files through relative
and absolute directory changes. Their audit reparses 134 clean Tar processes.
`evidence/tar-pending-current-paths-audited.json` combines all 66 disjoint
current-candidate groups and 535 clean Tar processes without counting earlier
reruns twice.

Nineteen unchanged multi-volume and sparse-file groups also pass, including
8-GiB logical sparse files, SEEK_HOLE detection, and all three PAX sparse
multi-volume formats. The shared macro is fingerprinted and counted as an
auxiliary input. Large retained fixtures preserve sparse holes.
`evidence/tar-pending-current-sparse-audited.json` now verifies 85 disjoint
current-candidate groups and 676 clean Tar processes.

The `tar-map-probe` candidate fixes two findings from the next originals:
map parsing now frees its getline buffer, and compression detection initializes
its probe block before reading an empty or partial archive. Only Tar’s private
`map.o` and `buffer.o` change; the Rust entry and GNU oracle remain unchanged.
It passes 26 further original groups (400 clean Tar processes), 57 focused
comparisons, 428 Coreutils smoke checks and 11 full dispatcher checks, with a
byte-identical rebuild. This includes hard links, numeric owner mappings, ACLs,
extended attributes and unprivileged read-only extraction.

`evidence/tar-map-probe-validation.json` preserves both initial findings and
one unchanged capability-test failure shared with GNU: current `getcap` omits
the historical equals prefix expected by the original. That original contributes
no passing group. The initial dispatcher run used the installed 133-command
expectation; its two list mismatches remain separate from the passing explicit
187-command profile. Earlier passing reports retain their actual binary hashes.

Eight additional originals pass for remounted directories, files removed or
changed during archiving, and one million exclusion patterns. Remounting stays
inside a private mount namespace. The combined current-candidate audit now
contains 34 groups and 455 clean Tar processes. The separate SIGPIPE original
passes every assertion, but its signal-termination snapshot retains an owned
archive descriptor; strict-clean acceptance remains open for that group.

Thirteen further extraction and compression groups pass, with 81 clean Tar
processes and 59 clean native compressor/shell child processes. Five compression
registrations pass using pinned native dependencies; XZ’s assertions also pass,
but its own thread-storage and descriptor findings remain separate. Two
compression-failure originals expose a Tar child diagnostic-prefix mismatch.
The audit identifies each dependency through its exact private execution path
and pinned executable hash; these dependencies add no applet ports.

The registration inventory now distinguishes **244 registered groups** from
237 distributed `.at` files. Two groups (`exclude17` and `exclude18`) exist in
the generated suite but their standalone sources are absent from the signed
release archive. `sparsemvp.at` supplies shared macros rather than its own group;
`testsuite.at` directly registers six compression groups. Across recorded
candidate profiles, 216 groups from 211 input files are validated. The remaining
28 groups without a passing original stay explicit in `inventory/tar-groups.json`; full
acceptance and release activation remain open.

Thirteen preceding unchanged GNU Tar groups pass on the current 187-command
candidate, covering recursion toggles, long-name appends, create/append archive
equality, changed blocking factors, deletion, backups, link comparison, old
archive format and verification. `evidence/tar-pending-archive-audited.json`
reparses 194 clean Tar process logs. The driver checks selected source hashes
against the review inventory and runs independent private selections with four
workers. Across the preserved candidate profiles, 97 original groups from 96
input files are now validated; `T-recurse.at` registers two groups. The other
141 inventoried input files remain open. These counts do not imply all earlier
tests were rerun on the current binary. The installed release is unchanged.

The preceding completed audit verifies all 84 selected originals and 905 clean Tar
processes on these bytes. Coreutils help/version checks pass 428/428. This
candidate is not installed or fully certified.

The subsequent 131-command Sharutils candidate passes both assigned GNU
original scripts, with 14 clean instrumented command processes. It tracks and
closes reopened input/output streams and preserves full invocation names in
GNU error diagnostics. Its 72-case focused run was interrupted: the saved
report records 47 passes, but only 21 rows retain intact supporting logs after
filesystem damage. That run requires repetition before certification.

The completed recovery rerun passes all 72 Sharutils focused cases. A further
run uses a private full-device fixture because decoding applies output modes;
it also passes 72/72. The combined audit verifies both unchanged originals and
all 86 candidate process logs. Coreutils checks pass 428/428 for help/version,
107/107 for Valgrind help, and 318/318 for both behavior and instrumented
equivalence. All eleven other provider focused batches pass on this same
131-command candidate, together with 11 dispatcher checks and four calculator
terminal profiles. Retaining the prior Findutils originals now verifies their
separate memory audit as well as unchanged source/helper inputs; a fresh audit
again checks its 982 clean applet processes and retains 488 native child
findings. Tar's fresh rerun passes all 84 reviewed originals with 905 clean
processes. The 131-command build was installed before the Cpio increment; a separate source rebuild
produces identical executable bytes, and the shared-library dependency list is
unchanged. `evidence/sharutils-candidate-proof.json` binds 30 validation reports
to the installed executable and its recorded source/helper inputs.

On 2026-09-08, recovery preserved the damaged Git directory, working sources,
and raw evidence under `/root/backups/rboxc-recovery-20260908T064111Z`.
Six empty file objects and three directory-tree objects were reconstructed with
their exact hashes. The empty final commit object was replaced by a recovery
commit retaining the recovered staged tree and all readable parent history.
Git's full integrity check then passed. The Tar and Sharutils original-suite
inputs and logs remain intact; damaged partial evidence is retained separately.
See `evidence/git-recovery-20260908.json` for the recovery record and limitations.

GNU Cpio 2.15 is pinned from its signed GNU release archive. Both `cpio`'s
`main.c` and `mt.c` are translated, with independent native Pax/Gnulib helper
namespaces and both C entry objects excluded. The first focused run matches
GNU on all 52 cases; one `mt` early-exit descriptor finding is fixed by tracking
and closing only its owned local descriptor. The corrected run passes 52/52.
Ten reviewed unchanged Cpio originals pass their assertions; the first memory
run identifies one native copy-pass allocation leak, also present in GNU.
A copied helper adaptation frees the temporary current-directory string after
its contents are copied. The final audit passes all ten reviewed originals
and all 52 focused cases, verifying 95 clean candidate processes. All installed
providers pass fresh focused regressions. The combined Coreutils assessment
passes 318/318: 314 cases passed initially, and four `stdbuf` cases pass after
staging the existing byte-identical `libstdbuf.so` companion. Both runs remain
recorded. The installed build now has 133 commands; an independent rebuild
produces identical bytes, with no added shared-library dependencies.
`evidence/cpio-candidate-proof.json` binds 32 validation reports to this build.
Seven other registered Cpio originals and actual tape operations remain
unvalidated; these results do not certify full-provider compatibility.

## Status

Of the 553 observed names, 189 currently have GNU provider assignments, including
the excluded SELinux commands `chcon` and `runcon`. After those exclusions,
54 GNU-assigned names remain to install. Another 354 names remain deferred for provider review, and
12 SELinux commands in total are excluded. These are implementation counts;
full compatibility validation remains separate.

The installed executable registers 133 commands: 107 Coreutils entries, GNU Hello,
GNU Time, GNU Which, four GNU Diffutils commands, three GNU Grep commands, four GNU Gzip commands, GNU Sed, GNU bc/dc, GNU Ed, Findutils (`find`, `xargs`, `locate`), GNU Tar, Sharutils (`uuencode`, `uudecode`), and Cpio (`cpio`, `mt`),
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

The current release executable is 4,940,736 bytes (4.71 MiB), dynamically linked
on the recorded host profile. This does not include native shared-library
dependencies or command-specific runtime helpers such as GNU `stdbuf`'s library.
Cross-platform builds and release packaging remain open.

Original-suite results for unchanged earlier providers retain their actual binary
hashes: the 124-command release, the 125-command Ed release, the earlier
117-command Grep release, the 128-command Findutils release, or the 131-command
Tar/Sharutils release. Verified
source/helper identity and complete fresh focused comparisons support retaining
those results. They are not reported as original-suite reruns on the 133-command
executable. The input-identity validator also rejects incomplete focused batches.

The current release dynamically links libacl, libattr, and libcap for GNU metadata
helpers, libpcre2-8 for Grep PCRE matching, and libm for Findutils. A separate static-link trial against the previous 2,405,616-byte
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

The upstream repository is [atlury/rboxc](https://github.com/atlury/rboxc).
Incremental commits preserve implementation and evidence separately from
installed-release certification. Native build trees and raw test logs remain
local and require separate backups.
