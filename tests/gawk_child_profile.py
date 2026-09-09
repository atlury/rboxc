"""Identify declared GNU recipe children through their private helper paths."""
# SPDX-License-Identifier: GPL-3.0-or-later
from pathlib import Path
import re


def canonical_child(command, expected, directory=None, dependencies=None):
    dependencies = dependencies or {}
    if directory is not None:
        assert re.fullmatch(r'/tmp/rboxc-gawk-original-[A-Za-z0-9_-]+(?:/test)?', directory)
        prefix = directory + '/deps/'
        if command.startswith(prefix):
            candidate = command[len(prefix):]
            name = candidate.split()[0]
            assert re.fullmatch(r'[a-z][a-z0-9_-]*', name)
            assert name in dependencies, 'undeclared native child helper'
            assert Path(dependencies[name]['path']).is_absolute()
            command = candidate
    assert command in expected, 'unclassified child process: ' + command
    return command
