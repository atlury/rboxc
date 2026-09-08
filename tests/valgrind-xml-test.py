#!/usr/bin/env python3
"""Exercise XML parsing against quiet Memcheck logs and incomplete reports."""
# SPDX-License-Identifier: GPL-3.0-or-later
from pathlib import Path
import subprocess
import tempfile
import unittest
import xml.etree.ElementTree as ET
from valgrind_xml import parse_memory_xml


class XmlTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.logs = {}
        with tempfile.TemporaryDirectory(prefix='rboxc-xml-test-') as directory:
            for name, command in {'clean':['/usr/bin/true'], 'reachable':['/bin/sh','-c',':'],
                                  'descriptor':['/bin/sh','-c','exec 3</dev/null']}.items():
                path = Path(directory)/(name+'.xml')
                subprocess.run(['/usr/bin/valgrind','--quiet','--xml=yes','--xml-file='+str(path),
                                '--track-fds=yes','--leak-check=full','--show-leak-kinds=all',
                                '--default-suppressions=no',*command], check=True,
                               capture_output=True, timeout=20, env={'PATH':'/usr/bin:/bin','LC_ALL':'C'})
                cls.logs[name] = path.read_bytes()

    def test_quiet_complete_clean_report(self):
        self.assertTrue(parse_memory_xml(self.logs['clean'])['memory_clean'])

    def test_reachable_allocations_are_separate(self):
        result = parse_memory_xml(self.logs['reachable'])
        self.assertTrue(result['memory_clean'])
        self.assertGreater(result['leak_bytes']['Leak_StillReachable'], 0)

    def test_descriptor_finding_even_without_errorcounts(self):
        result = parse_memory_xml(self.logs['descriptor'])
        self.assertFalse(result['memory_clean'])
        self.assertEqual(result['non_inherited_descriptors'], 1)
        self.assertEqual(result['error_counts'], {})

    def test_missing_final_status_rejected(self):
        root = ET.fromstring(self.logs['clean'])
        root.remove(root.findall('status')[-1])
        with self.assertRaises(AssertionError):
            parse_memory_xml(ET.tostring(root))

    def test_truncated_document_rejected(self):
        with self.assertRaises(ET.ParseError):
            parse_memory_xml(self.logs['clean'][:-30])

    def test_unknown_finding_is_not_clean(self):
        root = ET.fromstring(self.logs['descriptor'])
        root.find('error/kind').text = 'FutureFindingKind'
        self.assertFalse(parse_memory_xml(ET.tostring(root))['memory_clean'])


if __name__ == '__main__':
    unittest.main()
