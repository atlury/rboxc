"""Bind original child commands to verified fork PIDs or bounded date retries."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
from pathlib import Path
import re


def resolve(row, files):
    mode = row.get('dynamic_children')
    expected = row.get('child_commands', {})
    if mode is None:
        return expected, None
    assert (row['target'], mode) in (
        ('fork', 'parent-pid'), ('fork2', 'child-pid'), ('strftime', 'date-retries'))
    records = {}
    for file in files:
        text = Path(file).read_text()
        pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
        assert len(pids) == 1
        pid = pids.pop()
        commands = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
        parents = re.findall(r'^==[0-9]+== Parent PID: ([0-9]+)$', text, re.M)
        assert len(commands) == len(parents) == 1
        assert pid not in records
        records[pid] = {'command': commands[0], 'parent': parents[0]}
    gawks = {p: r for p, r in records.items() if r['command'].startswith('gawk ')}
    if mode == 'date-retries':
        assert len(gawks) == 1
        date_commands = [r['command'] for r in records.values() if not r['command'].startswith('gawk ')]
        # The source retries at most ten times, and each date has one shell.
        shell = '/bin/sh -c date'
        count = date_commands.count(shell)
        assert 1 <= count <= 10 and len(date_commands) == 2 * count
        assert expected == {shell: 10, 'date': 10}
        return {shell: count, 'date': count}, {'mode': mode, 'retries': count}
    assert len(gawks) == 2
    edges = [(r['parent'], p) for p, r in gawks.items() if r['parent'] in gawks]
    assert len(edges) == 1
    parent, child = edges[0]
    assert all(r['command'] == 'gawk -f '+row['target']+'.awk' for r in gawks.values())
    pid = parent if mode == 'parent-pid' else child
    assert expected == {r'/bin/sh -c rm\ \ fork.tmp.{pid}': 1, 'rm fork.tmp.{pid}': 1}
    resolved = {k.replace('{pid}', pid): v for k, v in expected.items()}
    return resolved, {'mode': mode, 'parent_pid': parent, 'child_pid': child, 'filename_pid': pid}


def verify(row, outcome, root):
    if not row.get('dynamic_children'):
        assert outcome.get('dynamic_children') is None
        return row.get('child_commands', {})
    if not outcome['memory']:
        assert outcome.get('dynamic_children') is None
        return row.get('child_commands', {})
    expected, binding = resolve(row, [root/m['log'] for m in outcome['memory']])
    assert outcome['dynamic_children'] == binding
    return expected
