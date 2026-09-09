"""Release replaced RETURN strings while retaining the active handler's borrow."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess

from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    original = root/'build/bash-cleanup/trap.c'
    baseline_path = root/'evidence/bash-native-cleanup.json'
    baseline = json.loads(baseline_path.read_text())
    record, = [r for r in baseline['files'] if Path(r['source']).name == 'trap.c']
    assert fingerprint(original) == record['adapted_sha256']
    text = original.read_text()

    def replace(old, new):
        nonlocal text
        assert text.count(old) == 1, old
        text = text.replace(old, new)

    replace('static void change_signal (int, char *);',
            'static void change_signal (int, char *);\n'
            '/* Borrowed by the current RETURN handler; never free on replacement. */\n'
            'static char *rboxc_active_return_trap;')
    replace('  if ((sigmodes[sig] & SIG_INPROGRESS) == 0)\n    free_trap_command (sig);',
            '  if ((sigmodes[sig] & SIG_INPROGRESS) == 0 ||\n'
            '      (sig == RETURN_TRAP && trap_list[sig] != rboxc_active_return_trap))\n'
            '    free_trap_command (sig);')
    replace('\t  (sigmodes[sig] & SIG_INPROGRESS) == 0)\n\tfree_trap_command (sig);',
            '\t  (sigmodes[sig] & SIG_INPROGRESS) == 0 ||\n'
            '          (sig == RETURN_TRAP && trap_list[sig] != rboxc_active_return_trap))\n'
            '\tfree_trap_command (sig);')
    anchor = '_run_trap_internal (int sig, char *tag)\n{'
    replace(anchor, anchor+'\n  char *rboxc_previous_return_trap;')
    replace('      old_trap = trap_list[sig];\n      old_modes = sigmodes[sig];',
            '      old_trap = trap_list[sig];\n'
            '      rboxc_previous_return_trap = rboxc_active_return_trap;\n'
            '      if (sig == RETURN_TRAP) rboxc_active_return_trap = old_trap;\n'
            '      old_modes = sigmodes[sig];')
    replace('      if (sigmodes[sig] & SIG_CHANGED)\n\t{',
            '      if (sig == RETURN_TRAP)\n'
            '        rboxc_active_return_trap = rboxc_previous_return_trap;\n\n'
            '      if (sigmodes[sig] & SIG_CHANGED)\n\t{')
    stage = root/'build/bash-return-trap-cleanup'
    stage.mkdir(exist_ok=True)
    source = stage/'trap.c'
    source.write_text(text)
    obj = stage/'trap.o'
    command = list(record['compile_arguments'])
    command[command.index(str(original))] = str(source)
    command[command.index('-o')+1] = str(obj)
    log = stage/'build.log'
    with log.open('w') as output:
        subprocess.run(command, cwd=root/'build/gnu-bash', stdout=output,
                       stderr=subprocess.STDOUT, check=True)
    assert 'trap.o' in adapted
    adapted['trap.o'] = obj
    report = {'scope': 'The active RETURN handler retains its borrowed old string until GNU releases it. Subsequent restored or replaced strings are freed before replacement/reset. Handler text, flags and normal unwind order are preserved. Other trap kinds retain the existing implementation.',
        'driver_sha256': fingerprint(Path(__file__)),
        'baseline_profile_sha256': fingerprint(baseline_path),
        'baseline_adapted_source_sha256': fingerprint(original),
        'baseline_object_sha256': record['object_sha256'],
        'source': str(source), 'source_sha256': fingerprint(source),
        'object': str(obj), 'object_sha256': fingerprint(obj),
        'compile_command': command, 'build_log': str(log), 'build_log_sha256': fingerprint(log)}
    (root/'evidence/bash-return-trap-cleanup.json').write_text(json.dumps(report, indent=2)+'\n')
