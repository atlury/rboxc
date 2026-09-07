#!/usr/bin/env python3
"""Check isolation and input-change detection for alternate comparison reports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import contextlib
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import comparison_profile as profile


class ProfileTests(unittest.TestCase):
    def setUp(self):
        self.workspace = tempfile.TemporaryDirectory(prefix='rboxc-profile-test-')
        self.addCleanup(self.workspace.cleanup)
        self.root = Path(self.workspace.name)
        for name in ('target/release/rboxc', 'candidate/rboxc', 'candidate/libstdbuf.so',
                     'build/gnu-coreutils/src/coreutils'):
            path = self.root/name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(name.encode())
        (self.root/'evidence/raw').mkdir(parents=True)
        self.candidate = self.root/'candidate/rboxc'

    def make(self, arguments):
        with patch.object(profile, 'ROOT', self.root), patch('sys.argv', ['fixture', *arguments]):
            return profile.ComparisonProfile('behavior', selections=True)

    def named(self):
        return self.make(['--candidate', str(self.candidate), '--report-name', 'candidate-behavior', 'cat'])

    def test_default_keeps_installed_paths(self):
        result = self.make(['cat'])
        self.assertEqual(result.binary, self.root/'target/release/rboxc')
        self.assertEqual(result.report, self.root/'evidence/behavior.json')
        self.assertEqual(result.logs, self.root/'evidence/raw')

    def test_named_candidate_has_separate_logs(self):
        result = self.named()
        self.assertEqual(result.options.commands, ['cat'])
        self.assertEqual(result.logs.parent, self.root/'evidence/raw/candidate-behavior')
        self.assertNotEqual(result.logs, self.named().logs)
        self.assertEqual(result.report, self.root/'evidence/candidate-behavior.json')
        self.assertEqual(result.metadata()['binary_sha256'], profile.fingerprint(self.candidate))

    def test_alternate_requires_name(self):
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            self.make(['--candidate', str(self.candidate)])

    def test_canonical_and_path_names_rejected(self):
        for name in ('behavior', 'smoke', 'valgrind', 'dispatcher', 'valgrind-equivalence', '../other'):
            with self.subTest(name=name), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                self.make(['--report-name', name])

    def test_replaced_candidate_invalidates_report(self):
        result = self.named()
        self.candidate.write_bytes(b'changed')
        with self.assertRaisesRegex(AssertionError, 'candidate changed'):
            result.metadata()

    def test_replaced_oracle_invalidates_report(self):
        result = self.named()
        result.oracle.write_bytes(b'changed')
        with self.assertRaisesRegex(AssertionError, 'GNU oracle changed'):
            result.metadata()

    def test_replaced_helper_invalidates_report(self):
        result = self.named()
        (self.candidate.parent/'libstdbuf.so').write_bytes(b'changed')
        with self.assertRaisesRegex(AssertionError, 'runtime helper changed'):
            result.metadata()


if __name__ == '__main__':
    unittest.main()
