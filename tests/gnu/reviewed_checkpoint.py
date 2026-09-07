"""Durable completed-implementation checkpoints for named GNU test batches."""
# SPDX-License-Identifier: GPL-3.0-or-later
import copy
import fcntl
import hashlib
import json
import os


class RunCheckpoint:
    def __init__(self, path, root):
        self.path, self.root = path, root
        self.lock = path.with_suffix('.lock').open('a')
        try:
            fcntl.flock(self.lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            self.lock.close()
            raise RuntimeError(f'test batch already running: {path.name}') from None
        self.data = json.loads(path.read_text()) if path.exists() else {
            'version': 1, 'scope': 'Completed individual runs; not combined script passes.', 'runs': {}}
        assert self.data['version'] == 1, 'unsupported checkpoint version'

    def fingerprints(self, outcome):
        names = [outcome['log'], *(m['log'] for m in outcome.get('memory', []))]
        result = {}
        for name in names:
            path = (self.root / name).resolve()
            assert path.is_relative_to((self.root / 'evidence/raw').resolve()), 'log outside raw evidence'
            result[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        return result

    def get(self, script, context, implementation):
        run = self.data['runs'].get(script, {})
        if run.get('context') != context:
            return None
        saved = run.get('outcomes', {}).get(implementation)
        if saved is None:
            return None
        try:
            if self.fingerprints(saved['result']) != saved['logs']:
                return None
        except FileNotFoundError:
            return None
        return copy.deepcopy(saved['result'])

    def save(self, script, context, implementation, outcome):
        run = self.data['runs'].get(script)
        if run is None or run['context'] != context:
            run = {'context': copy.deepcopy(context), 'outcomes': {}}
            self.data['runs'][script] = run
        run['outcomes'][implementation] = {
            'result': copy.deepcopy(outcome), 'logs': self.fingerprints(outcome)}
        temporary = self.path.with_suffix('.json.tmp')
        with temporary.open('w') as stream:
            stream.write(json.dumps(self.data, indent=2) + '\n')
            stream.flush()
            os.fsync(stream.fileno())
        temporary.replace(self.path)
        directory = os.open(self.path.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)

    def close(self):
        self.lock.close()
