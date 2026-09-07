"""Distinguish Time exit reports from fork headers before untraced exec."""
# SPDX-License-Identifier: GPL-3.0-or-later
import re
from comparison_profile import fingerprint


def observations(directory, parser, root):
    rows = []
    for path in sorted(directory.glob('*.log')):
        text = path.read_text()
        parent = re.search(r'^==\d+== Parent PID: (\d+)\s*$', text, re.M)
        rows.append({**parser(text, path.stem), 'pid': int(path.stem),
                     'parent_pid': int(parent[1]) if parent else None,
                     'log': str(path.relative_to(root)), 'sha256': fingerprint(path),
                     'header_only': all(re.fullmatch(
                         r'==\d+== (?:Memcheck, a memory error detector|Copyright .*|Using Valgrind-.*|Command: .*|Parent PID: \d+|\s*)', line)
                         for line in text.splitlines())})
    pids = {row['pid'] for row in rows}
    for row in rows:
        child = row['parent_pid'] in pids
        row['process_role'] = 'fork-child' if child else 'time-parent'
        row['assessment'] = ('untraced-exec-boundary' if child and row['header_only'] else
                             'exit-summary' if row['errors'] is not None and row['non_inherited_descriptors'] is not None
                             else 'incomplete')
    return rows


def clean_time_exits(rows):
    # Header-only fork records are retained as unassessed exec boundaries.
    # They are not clean child-process evidence. A missing Time parent summary
    # or a finding in a child that fails to exec still fails this assessment.
    roots = [row for row in rows if row['process_role'] == 'time-parent']
    measured = [row for row in rows if row['assessment'] == 'exit-summary']
    return bool(roots) and all(row['assessment'] == 'exit-summary' for row in roots) and all(
        row['assessment'] != 'incomplete' for row in rows) and all(
        row['errors'] == 0 and row['non_inherited_descriptors'] == 0 and not any(
            row['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
        for row in measured)
