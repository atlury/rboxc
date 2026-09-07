"""Pinned runtime prerequisites for reviewed GNU Grep original tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
from comparison_profile import fingerprint

ROOT = Path(__file__).resolve().parents[1]

def prepare():
    coreutils = ROOT/'build/gnu-coreutils/src/coreutils'
    expected = json.loads((ROOT/'evidence/smoke.json').read_text())['gnu_binary_sha256']
    assert fingerprint(coreutils) == expected
    config = ROOT/'build/gnu-grep/config.h'
    assert '#define HAVE_LIBPCRE 1' in config.read_text()
    report_path = ROOT/'evidence/grep-test-locales.json'
    report = json.loads(report_path.read_text())
    collection = Path(report['runtime_path'])
    for locale in report['locales']:
        path = collection/locale['name']
        assert {str(p.relative_to(path)):fingerprint(p) for p in path.rglob('*') if p.is_file()} == locale['files']
        for alias in locale['aliases']:
            assert (collection/alias).readlink() == Path(locale['name'])
    environment = {'LOCPATH':str(collection), 'LOCALE_FR':'fr_FR.ISO-8859-1',
                   'LOCALE_FR_UTF8':'fr_FR.UTF-8', 'CONFIG_HEADER':str(config),
                   'PCRE_WORKS':'1'}
    metadata = {'coreutils':{'path':str(coreutils),'sha256':expected,'commands':['timeout','sleep']},
                'config_header':{'path':str(config),'sha256':fingerprint(config)},
                'locale_report':{'path':str(report_path.relative_to(ROOT)),'sha256':fingerprint(report_path)},
                'environment':environment, 'adapter_sha256':fingerprint(Path(__file__))}
    return coreutils, environment, metadata
