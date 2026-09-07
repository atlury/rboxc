"""Pinned test-only helpers and isolated locales for GNU Sed originals."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
from comparison_profile import fingerprint

ROOT=Path(__file__).resolve().parents[1]


def prepare():
    path=ROOT/'evidence/sed-test-prerequisites.json'
    report=json.loads(path.read_text())
    locale_path=ROOT/'evidence/sed-test-locales.json'
    assert fingerprint(locale_path)==report['locale_report_sha256']
    locales=json.loads(locale_path.read_text())
    collection=Path(locales['runtime_path'])
    for locale in locales['locales']:
        directory=collection/locale['name']
        assert {str(p.relative_to(directory)):fingerprint(p) for p in directory.rglob('*') if p.is_file()}==locale['files']
        assert all((collection/n).readlink()==Path(locale['name']) for n in locale['aliases'])
    for helper in report['helpers'].values():
        assert fingerprint(Path(helper['path']))==helper['sha256']
    config=ROOT/'build/gnu-sed/config.h'
    assert fingerprint(config)==report['config_header_sha256']
    environment={'LOCPATH':str(collection),'LOCALE_FR':'fr_FR.ISO-8859-1',
                 'LOCALE_FR_UTF8':'fr_FR.UTF-8','LOCALE_JA':'ja_JP.EUC-JP',
                 'CONFIG_HEADER':str(config)}
    return report['helpers'],environment,{'path':str(path.relative_to(ROOT)),
            'sha256':fingerprint(path),'environment':environment,'adapter_sha256':fingerprint(Path(__file__))}
