"""Read complete Memcheck XML, including findings hidden by quiet text output."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import xml.etree.ElementTree as ET


def parse_memory_xml(contents):
    root = ET.fromstring(contents)
    assert root.tag == 'valgrindoutput'
    assert root.findtext('protocolversion') == '6'
    assert root.findtext('protocoltool') == root.findtext('tool') == 'memcheck'
    assert [s.findtext('state') for s in root.findall('status')] == ['RUNNING', 'FINISHED']
    assert root.find('errorcounts') is not None
    assert not root.findall('fatal_signal')
    pid = root.findtext('pid')
    assert pid and pid.isdecimal()
    arguments = root.find('args/argv')
    assert arguments is not None and arguments.findtext('exe')
    errors = root.findall('error')
    identities = [e.findtext('unique') for e in errors]
    assert all(identities) and len(set(identities)) == len(identities)
    kinds = Counter()
    leaks = Counter()
    for error in errors:
        kind = error.findtext('kind')
        assert kind
        kinds[kind] += 1
        if kind.startswith('Leak_'):
            amount = int(error.findtext('xwhat/leakedbytes'))
            assert amount >= 0
            leaks[kind] += amount
    counts = {}
    for pair in root.findall('errorcounts/pair'):
        identity = pair.findtext('unique')
        assert identity in identities and identity not in counts
        counts[identity] = int(pair.findtext('count'))
        assert counts[identity] > 0
    suppressed = sum(int(p.findtext('count')) for p in root.findall('suppcounts/pair'))
    assert suppressed >= 0
    findings = sum(count for kind, count in kinds.items() if kind != 'Leak_StillReachable')
    return {'pid': pid, 'argv': [arguments.findtext('exe'), *[a.text or '' for a in arguments.findall('arg')]],
            'valgrind_arguments': [a.text or '' for a in root.findall('args/vargv/arg')],
            'complete': True, 'kinds': dict(kinds), 'leak_bytes': dict(leaks),
            'error_counts': counts, 'finding_records': findings,
            'non_inherited_descriptors': kinds['FdNotClosed'], 'suppressed': suppressed,
            'memory_clean': findings == 0 and suppressed == 0}
