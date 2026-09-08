"""Keep alternate-binary comparisons separate from installed-release evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
from pathlib import Path
import re
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class ComparisonProfile:
    def __init__(self, default_report, *, selections=False, oracle=None):
        parser = argparse.ArgumentParser()
        parser.add_argument('--candidate', type=Path, default=ROOT/'target/release/rboxc')
        parser.add_argument('--report-name', help='Separate report and memory-log directory for a candidate build')
        if selections:
            parser.add_argument('commands', nargs='*')
        self.options = parser.parse_args()
        self.binary = self.options.candidate.resolve(strict=True)
        if self.binary != ROOT/'target/release/rboxc' and not self.options.report_name:
            parser.error('an alternate candidate requires --report-name')
        name = self.options.report_name or default_report
        if not re.fullmatch(r'[a-z0-9][a-z0-9-]*', name):
            parser.error('invalid report name')
        if self.options.report_name in {'smoke', 'valgrind', 'dispatcher', 'behavior', 'valgrind-equivalence'}:
            parser.error('named comparisons cannot overwrite installed-release reports')
        self.report = ROOT/'evidence'/(name+'.json')
        if self.options.report_name and self.report.exists():
            parser.error('named comparison report already exists; choose a new --report-name')
        self.logs = ROOT/'evidence/raw'
        if self.options.report_name:
            self.logs = self.logs/name
            self.logs.mkdir(exist_ok=True)
            self.logs = Path(tempfile.mkdtemp(prefix='run-', dir=self.logs))
        self.binary_sha256 = fingerprint(self.binary)
        self.oracle = Path(oracle) if oracle is not None else ROOT/'build/gnu-coreutils/src/coreutils'
        self.oracle_sha256 = fingerprint(self.oracle)
        self.runtime_helpers = {path: fingerprint(path) for path in
                                [self.binary.parent/'libstdbuf.so', self.oracle.parent/'libstdbuf.so']
                                if path.exists()}

    def metadata(self):
        # Refuse to publish a report if either executable changed mid-run.
        assert fingerprint(self.binary) == self.binary_sha256, 'candidate changed during comparison'
        assert fingerprint(self.oracle) == self.oracle_sha256, 'GNU oracle changed during comparison'
        for path, expected in self.runtime_helpers.items():
            assert fingerprint(path) == expected, 'runtime helper changed during comparison'
        return {'runtime_helpers': {str(path): value for path, value in self.runtime_helpers.items()},
                'binary': str(self.binary), 'binary_sha256': self.binary_sha256,
                'gnu_binary_sha256': self.oracle_sha256}
