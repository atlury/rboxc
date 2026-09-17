# Upstream preservation checkpoint — 2026-09-17

This checkpoint saves the local source, scripts and structured test reports
before the owner removes the local workspace to reclaim disk space.

- `main` includes the previously uncommitted Bash failed-exec trap cleanup,
  privilege-launcher signal defaults, audit scripts and recorded reports.
- Tag `preserved-capability-20260917` preserves the separate capability
  worktree, including its previously uncommitted build profile and reports.
  This historical profile is preserved independently; it is not merged into
  the current implementation.
- The owner explicitly declined preservation of `evidence/raw/`. It remains
  ignored for new files, as do generated `build/`, `target/` and `.tools/`
  directories. Some raw files were already committed historically and remain
  in Git; this checkpoint neither adds the bulk raw directory nor rewrites
  history. A Git clone does not restore the full local evidence or historical
  build outputs.

The structured reports retain their original results, limitations, paths and
hashes. Referenced raw logs, fixtures and build outputs may no longer exist
after local deletion. A saved report is not a new test run or independent
verification of unavailable evidence. Rebuilds and fresh test runs are needed
to produce new evidence. This checkpoint does not claim full applet completion.

The Bash execscript report retains its expected-output and Valgrind E2BIG
instrumentation baselines. The existing audit records 68 focused trap
comparisons and 285 clean original candidate process images, with one
incomplete instrumented image; the whole original recipe remains outside
strict pass counts.
