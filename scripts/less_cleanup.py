"""Track ownership of GNU Less's keyboard descriptor through its existing close."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def adapt(text):
    assert text.count('#include "less.h"') == 1
    text = text.replace('#include "less.h"', '#include "less.h"\n#include <errno.h>')
    anchor = '#if !MSDOS_COMPILER\nstatic int open_tty_device(constant char* dev)'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '#if !MSDOS_COMPILER\n'
        'static int rboxc_last_opened_tty = -1;\n'
        'static int rboxc_owned_keyboard = -1;\n'
        'static int open_tty_device(constant char* dev)')
    anchor = '\treturn open(dev, OPEN_READ);'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '\tint fd = open(dev, OPEN_READ);\n'
        '\tif (fd >= 0) rboxc_last_opened_tty = fd;\n\treturn fd;')
    anchor = 'public int open_tty(void)\n{\n\tint fd = -1;'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor+'\n\trboxc_last_opened_tty = -1;')
    anchor = '\ttty = open_tty();'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor+'\n\trboxc_owned_keyboard = '
                        '(tty == rboxc_last_opened_tty) ? tty : -1;')
    anchor = 'public void close_getchr(void)\n{'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor+'''
#if !MSDOS_COMPILER
    int saved_errno = errno;
    int owned = rboxc_owned_keyboard;
    rboxc_owned_keyboard = -1;
    if (owned >= 0) {
        tty = -1;
        close(owned);
    }
    errno = saved_errno;
#endif
''')
    return text

def prepare(root):
    pin = json.loads((root/'inventory/sources.json').read_text())['less']
    source = Path(pin['source'])/'ttyin.c'
    assert fingerprint(source) == pin['source_and_header_sha256']['ttyin.c']
    stage = root/'build/less-cleanup'
    stage.mkdir(exist_ok=True)
    target = stage/'ttyin.c'
    target.write_text(adapt(source.read_text()))
    records = [json.loads(p.read_text()) for p in (root/'build/less-cc-records').glob('*.json')]
    records = [r for r in records if r.get('file') == str(source) and r.get('kind') == 'compile']
    assert len(records) == 1
    record = records[0]
    obj = stage/'ttyin.o'
    argv = record['arguments'].copy()
    argv[argv.index(str(source))] = str(target)
    assert '-o' not in argv
    argv += ['-iquote'+str(source.parent), '-o', str(obj)]
    log = stage/'build.log'
    with log.open('wb') as stream:
        subprocess.run(argv, cwd=record['directory'], stdout=stream, stderr=subprocess.STDOUT, check=True)
    report = {'scope': 'GNU Less Linux keyboard ownership: remember successful device opens, including descriptors '
              '0/1/2, independently of borrowed stdin/stderr fallbacks. Release the owned descriptor through the '
              'existing close_getchr after terminal restoration, invalidate ownership before closing and preserve errno. '
              'Original source and native oracle are unchanged.',
              'source': str(source), 'source_sha256': fingerprint(source),
              'adapted_source': str(target), 'adapted_sha256': fingerprint(target),
              'object': str(obj), 'object_sha256': fingerprint(obj), 'compiler_arguments': argv,
              'log': str(log.relative_to(root)), 'log_sha256': fingerprint(log),
              'driver_sha256': fingerprint(Path(__file__))}
    (root/'evidence/less-native-cleanup.json').write_text(json.dumps(report, indent=2)+'\n')
    return {'ttyin.o': obj}
