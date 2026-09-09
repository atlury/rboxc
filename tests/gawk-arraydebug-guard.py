#!/usr/bin/env python3
"""Record GNU's unchanged array-debug capability guard for this build."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
source = Path('/opt/src/gawk-5.4.1')
makefile = ROOT/'build/gnu-gawk/test/Makefile'
text = makefile.read_text()
flags = re.search(r'^CFLAGS = (.*)$', text, re.M)[1]
assert 'ARRAYDEBUG' not in flags
recipe = re.search(r'^arraydebug-tests:.*?(?=\n\S|\Z)', text, re.M|re.S)[0]
assert 'else echo gawk is not compiled to support the array debug tests' in recipe
assert re.search(r'^ARRAYDEBUG_TESTS = arrdbg$', text, re.M)
fingerprint = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
paths = [Path(__file__), makefile, source/'test/Makefile.am', source/'test/Makefile.in',
         source/'test/arrdbg.awk', source/'awkgram.y', Path('/usr/bin/make'),
         Path('/bin/sh').resolve(), ROOT/'build/gnu-grep/src/grep']
inputs = {str(p):fingerprint(p) for p in paths}
report = ROOT/'evidence/gawk-arraydebug-guard.json'
saved = ROOT/'evidence/raw/gawk-arraydebug-guard'
assert not report.exists() and not saved.exists()
saved.mkdir()
with tempfile.TemporaryDirectory(prefix='rboxc-gawk-arraydebug-') as directory:
    work = Path(directory)
    (work/'grep').symlink_to(ROOT/'build/gnu-grep/src/grep')
    done = subprocess.run(['/usr/bin/make', '--no-print-directory', '-f', str(makefile), 'arraydebug-tests'],
                          cwd=work, stdin=subprocess.DEVNULL, capture_output=True, timeout=15,
                          env={'PATH':str(work)+':/usr/bin:/bin', 'HOME':directory, 'LC_ALL':'C'})
(saved/'stdout').write_bytes(done.stdout)
(saved/'stderr').write_bytes(done.stderr)
assert done.returncode == 0 and not done.stderr
assert done.stdout == b'gawk is not compiled to support the array debug tests\n'
assert all(fingerprint(Path(p)) == h for p, h in inputs.items())
report.write_text(json.dumps({
    'scope':'The unchanged GNU Make capability guard skips arrdbg because the recorded build lacks ARRAYDEBUG. No Gawk program or Valgrind process is executed; this is a build-profile observation, not an original applet pass.',
    'pass':True, 'cflags':flags, 'original_guard':'arraydebug-tests',
    'configured_recipe_sha256':hashlib.sha256(recipe.encode()).hexdigest(),
    'unavailable_input':'test/arrdbg.awk', 'applet_processes':0,
    'inputs':inputs, 'raw':{str(p.relative_to(ROOT)):fingerprint(p) for p in saved.iterdir()},
}, indent=2)+'\n')
print('Verified GNU ARRAYDEBUG capability guard; no applet execution counted')
