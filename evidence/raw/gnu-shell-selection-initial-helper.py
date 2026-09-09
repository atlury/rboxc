"""Materialize reviewed ordinary sections of pinned GNU shell tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib

# Each excluded interval starts and ends at a unique original line. Everything
# else, including GNU's assertions and final Exit, remains byte-for-byte intact.
EXCLUDED = {
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
        ('# Ensure invalid length is handled appropriately',
         '# Ensure base64 in untagged format that matches tags is supported'),
        ('# Ensure I/O errors handled appropriately', 'Exit $fail\n')],
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
    path = root / 'build/gnu-shell-selections' / row['script']
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists():
        assert path.read_bytes() == data, 'existing selection changed'
    else:
        path.write_bytes(data)
    return path
