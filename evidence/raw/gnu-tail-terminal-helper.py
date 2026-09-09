"""Materialize reviewed ordinary sections of pinned GNU shell tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib

# Each excluded interval starts and ends at a unique original line. Everything
# else, including GNU's assertions and final Exit, remains byte-for-byte intact.
EXCLUDED = {
    'tests/ptx/ptx-overrun.sh': [
        ('# Trigger a heap-clobbering bug in ptx', 'Exit $fail\n')],
    'tests/ln/backup-suffix-traversal.sh': [
        ('# Test 1: Command line suffix with path traversal attempt',
         '# Test 3: Verify normal suffixes still work')],
    'tests/tail/follow-stdin.sh': [
        ('# Before coreutils-8.26 this would induce an UMR under UBSAN',
         '# Before coreutils-8.28 this would erroneously issue a warning')],
    'tests/shred/shred-passes.sh': [
        ('# Trigger an issue in shred before v8.27', 'Exit $fail\n')],
    'tests/od/od-N.sh': [
        ('# coreutils <= 9.7 would buffer overflow with',
         '# coreutils <= 9.7 would output nothing')],
    'tests/tac/tac-2-nonseekable.sh': [
        ('# Assume timeout is due to failure to close stdin', 'Exit $fail\n')],
    'tests/cksum/b2sum.sh': [
        ('# This would segfault from coreutils-8.26',
         '# This would fail before coreutil-9.4')],
    'tests/cksum/cksum-c.sh': [
        ('uses_strace_\n', 'shuf -i 1-10 > input'),
        ('# Ensure invalid length is handled appropriately',
         '# Ensure base64 in untagged format that matches tags is supported'),
        ('# Ensure I/O errors handled appropriately', 'Exit $fail\n')],
    'tests/ln/misc.sh': [
        ('# With coreutils-5.2.1, this would mistakenly access',
         '# Verify that -f and -i override each other')],
    'tests/ln/relative.sh': [
        ('# Expect this to fail with exit status 1, or to succeed quietly',
         'Exit $fail\n')],
    'tests/split/line-bytes.sh': [
        ('# Ensure memory is not allocated up front',
         '# Ensure correct operation with various split and buffer size combinations'),
        ('# Test hold buffer management with --line-bytes.', 'Exit $fail\n')],
    'tests/sort/sort-merge-fdlimit.sh': [
        ("# 'sort -m' should work in a limited file descriptor", 'Exit $fail\n')],
    'tests/ls/w-option.sh': [
        ('# Ensure that 0 line length', '# coreutils <= 8.24 could display')],
    'tests/csplit/csplit.sh': [
        ('# csplit could get a failed assertion', "# 'echo | csplit -b"),
        ('# Ensure that lines longer than the initial buffer length',
         '# Ensure file not created for empty input')],
    'tests/shuf/shuf.sh': [
        ('uses_strace_\n', 'seq 100 > in'),
        ('# coreutils-8.22 dumps core.', '# coreutils-6.12 and earlier would output'),
        ('# Ensure shuf exits with 1 if memory exhausted',
         "# Ensure shuf -n0 doesn't read any input"),
        ('# shuf 8.25 mishandles input', 'Exit $fail\n')],
}


def selection(source, script):
    original = (source / script).read_bytes()
    intervals = []
    for start, end in EXCLUDED[script]:
        start, end = start.encode(), end.encode()
        assert original.count(start) == original.count(end) == 1
        a, b = original.index(start), original.index(end)
        assert a < b and (a == 0 or original[a-1:a] == b'\n')
        assert original[b-1:b] == b'\n'
        intervals.append((a, b))
    assert intervals == sorted(intervals)
    selected, ranges, cursor = b'', [], 0
    for a, b in intervals:
        assert cursor <= a
        selected += original[cursor:a]
        ranges.append({'start_byte': a, 'end_byte_exclusive': b,
                       'sha256': hashlib.sha256(original[a:b]).hexdigest()})
        cursor = b
    selected += original[cursor:]
    assert selected.endswith(b'Exit $fail\n')
    return selected, {
        'original_sha256': hashlib.sha256(original).hexdigest(),
        'selected_sha256': hashlib.sha256(selected).hexdigest(),
        'excluded_ranges': ranges, 'full_script': False,
    }


def materialize_selection(root, source, row):
    assert not row.get('full_suite') and not row.get('generator_inputs')
    data, metadata = selection(source, row['script'])
    assert row['shell_selection'] == metadata
    assert row['sha256'] == metadata['original_sha256']
    path = root / 'build/gnu-shell-selections' / metadata['selected_sha256'] / row['script']
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists():
        assert path.read_bytes() == data, 'existing selection changed'
    else:
        path.write_bytes(data)
    return path
