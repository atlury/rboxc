#!/usr/bin/env python3
"""Check that incomplete exec logs cannot become clean Valgrind evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('reviewed', Path(__file__).with_name('reviewed-original.py'))
reviewed = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reviewed)

HEADER = '==123== Command: fixture\n'
CLEAN = ('==123== FILE DESCRIPTORS: 3 open (3 inherited) at exit.\n'
         '==123== ERROR SUMMARY: 0 errors from 0 contexts\n')


class MemoryLogTests(unittest.TestCase):
    def parse(self, text):
        return reviewed.parse_memory_log(text, '123', exec_only=True)

    def test_exec_chain_with_final_summary(self):
        result = self.parse(HEADER * 3 + CLEAN)
        self.assertTrue(result['complete_exec_log'])
        self.assertEqual(result['exec_images'], 3)
        self.assertEqual(result['errors'], 0)
        self.assertEqual(result['non_inherited_descriptors'], 0)

    def test_header_only_is_incomplete(self):
        result = self.parse(HEADER)
        self.assertFalse(result['complete_exec_log'])
        self.assertIsNone(result['errors'])

    def test_earlier_summary_cannot_cover_unfinished_exec(self):
        self.assertFalse(self.parse(HEADER + CLEAN + HEADER)['complete_exec_log'])

    def test_forked_writer_is_rejected(self):
        self.assertFalse(self.parse(HEADER + CLEAN + HEADER.replace('123', '456'))['complete_exec_log'])

    def test_errors_and_resources_survive_later_clean_summary(self):
        earlier = (CLEAN.replace('0 errors', '2 errors').replace('3 open', '4 open')
                   + '==123== definitely lost: 1,024 bytes in 1 blocks\n')
        result = self.parse(HEADER + earlier + HEADER + CLEAN)
        self.assertEqual(result['errors'], 2)
        self.assertEqual(result['non_inherited_descriptors'], 1)
        self.assertEqual(result['heap_bytes']['definitely lost'], 1024)

    def test_missing_descriptor_summary_is_incomplete(self):
        self.assertFalse(self.parse(HEADER + CLEAN.splitlines(True)[1])['complete_exec_log'])

    def test_wrong_process_file_is_rejected(self):
        self.assertFalse(self.parse((HEADER + CLEAN).replace('123', '124'))['complete_exec_log'])


if __name__ == '__main__':
    unittest.main()
