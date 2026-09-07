#!/usr/bin/env python3
"""Check completed-run recovery without accepting stale or missing evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import tempfile
import unittest
from reviewed_checkpoint import RunCheckpoint


class Recovery(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.raw = self.root/'evidence/raw'
        self.raw.mkdir(parents=True)
        self.path = self.raw/'batch.progress.json'
        self.log = self.raw/'gnu.log'
        self.log.write_text('completed original assertions\n')
        self.context = {'binaries': {'gnu': 'oracle', 'rboxc': 'candidate'},
                        'definition': {'script': 'test.sh', 'valgrind': True}}
        self.outcome = {'status': 0, 'log': 'evidence/raw/gnu.log'}
        self.checkpoint = RunCheckpoint(self.path, self.root)
        self.addCleanup(self.checkpoint.close)
        self.checkpoint.save('test.sh', self.context, 'gnu', self.outcome)

    def test_reopen_keeps_only_completed_side(self):
        self.checkpoint.close()
        recovered = RunCheckpoint(self.path, self.root)
        self.addCleanup(recovered.close)
        self.assertEqual(recovered.get('test.sh', self.context, 'gnu'), self.outcome)
        self.assertIsNone(recovered.get('test.sh', self.context, 'rboxc'))

    def test_changed_context_requires_new_run(self):
        for key in ('binaries', 'definition'):
            changed = {**self.context, key: {'changed': True}}
            self.assertIsNone(self.checkpoint.get('test.sh', changed, 'gnu'))

    def test_changed_or_missing_log_requires_new_run(self):
        self.log.write_text('incomplete replacement\n')
        self.assertIsNone(self.checkpoint.get('test.sh', self.context, 'gnu'))
        self.log.unlink()
        self.assertIsNone(self.checkpoint.get('test.sh', self.context, 'gnu'))

    def test_memory_logs_are_also_verified(self):
        memory = self.raw/'memory.log'
        memory.write_text('ERROR SUMMARY: 0 errors\n')
        outcome = {**self.outcome, 'memory': [{'log': 'evidence/raw/memory.log'}]}
        self.checkpoint.save('test.sh', self.context, 'gnu', outcome)
        self.assertEqual(self.checkpoint.get('test.sh', self.context, 'gnu'), outcome)
        memory.write_text('unfinished process\n')
        self.assertIsNone(self.checkpoint.get('test.sh', self.context, 'gnu'))

    def test_failure_is_not_converted_to_pass(self):
        failed = {**self.outcome, 'status': 1}
        self.checkpoint.save('test.sh', self.context, 'rboxc', failed)
        self.assertEqual(self.checkpoint.get('test.sh', self.context, 'rboxc')['status'], 1)

    def test_context_replacement_discards_old_other_side(self):
        changed = {**self.context, 'new_profile': True}
        self.checkpoint.save('test.sh', changed, 'rboxc', self.outcome)
        self.assertIsNone(self.checkpoint.get('test.sh', changed, 'gnu'))

    def test_incomplete_temporary_write_does_not_replace_checkpoint(self):
        self.path.with_suffix('.json.tmp').write_text('{')
        self.assertEqual(json.loads(self.path.read_text())['version'], 1)
        self.assertEqual(self.checkpoint.get('test.sh', self.context, 'gnu'), self.outcome)

    def test_second_writer_is_rejected(self):
        with self.assertRaisesRegex(RuntimeError, 'already running'):
            RunCheckpoint(self.path, self.root)


if __name__ == '__main__':
    unittest.main()
