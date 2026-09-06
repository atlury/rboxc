#!/usr/bin/env python3
"""Compare backup names, preserved fixture data, and rollback with GNU cp."""
# Copyright (C) 2026 Rbox contributors.
# SPDX-License-Identifier: GPL-3.0-or-later

import os
from pathlib import Path
import shutil
import stat
import subprocess
import tempfile


def run(command, options, layout, environment=None):
    with tempfile.TemporaryDirectory(prefix="rbox-cp-backups-") as work:
        root = Path(work)
        root.chmod(0o777)
        source, destination = "source", "target"
        if layout.startswith("long-"):
            destination = "t" * int(layout.split("-")[1])
        (root / source).write_bytes(b"new data\n")
        (root / destination).write_bytes(b"old data\n")
        versions = {
            "gap": ["1", "3"], "only-high": ["7"], "carry": ["9", "99"],
            "wide": ["18446744073709551615", "999999999999999999999999999999"],
            "invalid": ["0", "01", "-1", "1a"], "mixed": ["01", "2", "3a", "11"],
        }.get(layout, [])
        for version in versions:
            (root / f"{destination}.~{version}~").write_bytes(f"saved {version}\n".encode())
        if layout == "long-251-same-length":
            (root / f"{destination}.~1~").write_bytes(b"first numbered backup\n")
        elif layout == "long-251-carry":
            (root / f"{destination}.~9~").write_bytes(b"ninth numbered backup\n")
        elif layout == "long-255-collision":
            (root / (destination[:253] + "~")).write_bytes(b"retained short backup\n")
        if layout == "backup-directory":
            (root / f"{destination}.~4~").mkdir()
        elif layout == "backup-symlink":
            (root / f"{destination}.~4~").symlink_to("missing")
        elif layout.startswith("source-"):
            (root / "target~").write_bytes(b"backup source\n")
            if layout == "source-dot":
                source, destination = "target~", "./target"
            elif layout == "source-parent":
                (root / "alias").symlink_to(".")
                source = "alias/target~"
            elif layout == "source-hardlink":
                (root / "sub").mkdir()
                os.link(root / "target~", root / "sub/target~")
                source = "sub/target~"
            else:
                (root / "sub").mkdir()
                (root / "sub/target~").symlink_to("../target~")
                source = "sub/target~"
        elif layout == "rollback":
            (root / source).chmod(0)
        elif layout == "dot-directory":
            (root / "sub").mkdir()
            (root / "sub/file").write_bytes(b"directory source\n")
            source = "sub/."
        kwargs = ({"user": 65534, "group": 65534, "extra_groups": []}
                  if layout == "rollback" and os.geteuid() == 0 else {})
        env = {**os.environ, "LC_ALL": "C", "TZ": "UTC0"}
        env.pop("SIMPLE_BACKUP_SUFFIX", None)
        env.pop("VERSION_CONTROL", None)
        env.update(environment or {})
        result = subprocess.run(command + options + [source, destination], cwd=root,
                                stdin=subprocess.DEVNULL, capture_output=True, timeout=10,
                                env=env, **kwargs)
        (root / "source").chmod(0o644)
        snapshot = []
        for directory, dirs, files in os.walk(root, followlinks=False):
            for name in sorted(dirs + files):
                path = Path(directory) / name
                metadata = path.lstat()
                content = (os.readlink(path) if path.is_symlink() else
                           path.read_bytes() if path.is_file() else None)
                snapshot.append((str(path.relative_to(root)), stat.S_IFMT(metadata.st_mode),
                                 stat.S_IMODE(metadata.st_mode), metadata.st_uid,
                                 metadata.st_gid, content))
        return result.returncode, result.stdout, result.stderr, sorted(snapshot)


def main():
    original = Path(os.environ.get("RBOX_PATH", "target/release/rboxc")).resolve()
    oracle = str(Path(os.environ.get("RBOX_GNU_PREFIX", "/opt/gnu/coreutils-9.11")) / "bin/cp")
    cases = []
    for layout in ["none", "gap", "only-high", "carry", "wide", "invalid", "mixed",
                   "backup-directory", "backup-symlink", "long-250", "long-254", "long-255",
                   "source-dot", "source-parent", "source-hardlink", "source-symlink", "rollback",
                   "long-251-same-length", "long-251-carry", "long-255-collision"]:
        for policy in ["simple", "numbered", "existing"]:
            cases.append((layout, ["-v", f"--backup={policy}"], {}))
    for policy in ["simple", "numbered", "existing"]:
        cases.append(("dot-directory", ["-Rv", f"--backup={policy}"], {}))
    for suffix in [".bak", "", "bad/suffix", "/", ".", ".."]:
        cases.append(("none", ["-vb"], {"SIMPLE_BACKUP_SUFFIX": suffix}))
        cases.append(("none", ["-vb", "--suffix=" + suffix], {"SIMPLE_BACKUP_SUFFIX": ".env"}))
    passed = failed = 0
    with tempfile.TemporaryDirectory(prefix="rbox-cp-backups-bin-") as binaries:
        os.chmod(binaries, 0o755)
        candidate = str(Path(binaries) / "rbox")
        shutil.copyfile(original, candidate)
        os.chmod(candidate, 0o755)
        for layout, options, environment in cases:
            expected = run([oracle], options, layout, environment)
            actual = run([candidate, "cp"], options, layout, environment)
            ok = expected == actual
            passed += ok
            failed += not ok
            print(f"{'PASS' if ok else 'FAIL'} {layout} {options} {environment}")
            if not ok:
                print("GNU:", expected)
                print("Rbox:", actual)
    print(f"GNU cp backups: {passed} pass, {failed} fail")
    return bool(failed)


if __name__ == "__main__":
    raise SystemExit(main())
