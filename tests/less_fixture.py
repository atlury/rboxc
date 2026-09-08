"""Read pinned screen-replay records as data for review and provenance checks."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re

def describe(data):
    assert data.startswith(b'!lesstest!\n!version 1\n')
    cursor = 0
    environment = {}
    files = []
    arguments = []
    keys = []
    frames = []
    run = False
    ended = False
    while cursor < len(data):
        end = data.index(b'\n', cursor)
        line = data[cursor:end]
        cursor = end+1
        if not line or line.startswith(b'!'):
            continue
        if line.startswith(b'F '):
            assert not run
            match = re.fullmatch(rb'F "([A-Za-z0-9_.-]+)" (\d+)', line)
            assert match and match[1] not in (b'.', b'..')
            size = int(match[2])
            contents = data[cursor:cursor+size]
            assert len(contents) == size
            cursor += size
            files.append({'name': match[1].decode(), 'bytes': size,
                          'sha256': hashlib.sha256(contents).hexdigest()})
        elif line.startswith(b'E '):
            assert not run
            match = re.fullmatch(rb'E "([A-Za-z0-9_@]+)" "([^"\n]*)"', line)
            assert match and match[1].decode() not in environment
            environment[match[1].decode()] = match[2].decode()
        elif line.startswith(b'A '):
            assert not run and not arguments
            match = re.fullmatch(rb'A "([A-Za-z0-9_.-]+)"', line)
            assert match
            arguments = [match[1].decode()]
        elif line.startswith(b'T '):
            assert not run
        elif line == b'R':
            assert not run
            run = True
        elif line.startswith(b'+'):
            assert run and not ended and re.fullmatch(rb'\+[0-9a-f]+', line)
            keys.append(int(line[1:], 16))
        elif line.startswith(b'='):
            assert run and not ended and len(line) < 10000
            frames.append(line[1:])
        elif line == b'Q':
            assert run and not ended
            ended = True
        else:
            raise AssertionError('unclassified replay record')
    assert run and ended and len(files) == 1 and arguments == [files[0]['name']]
    assert keys[-1] == ord('q') and len(frames) == len(keys)
    assert 1 <= int(environment['COLUMNS']) <= 200 and 1 <= int(environment['LINES']) <= 100
    return {'environment': environment, 'files': files, 'arguments': arguments,
            'keys': keys, 'frames': len(frames),
            'expected_frames_sha256': hashlib.sha256(b'\n'.join(frames)).hexdigest()}
