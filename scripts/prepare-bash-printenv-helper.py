#!/usr/bin/env python3
"""Link GNU's original printenv body with a standalone test entry point."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
stage=ROOT/'build/bash-printenv-helper';assert not stage.exists();stage.mkdir()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
source=stage/'main.c'
source.write_text('/* Standalone native GNU test oracle, not an applet port. */\n'
    'extern int single_binary_main_printenv(int, char **);\n'
    'int main(int argc, char **argv) { return single_binary_main_printenv(argc, argv); }\n')
objects=[ROOT/'build/gnu-coreutils/src/libsinglebin_printenv_a-printenv.o',
         ROOT/'build/gnu-coreutils/src/version.o',ROOT/'build/gnu-coreutils/lib/libcoreutils.a']
compiler=Path('/usr/bin/cc').resolve()
inputs={str(p):sha(p) for p in [compiler,source,*objects,
    Path('/opt/src/coreutils-9.11/src/printenv.c'),ROOT/'build/gnu-coreutils/lib/config.h']}
binary=stage/'printenv';log=stage/'build.log'
command=[str(compiler),str(source),*[str(p) for p in objects],'-o',str(binary)]
with log.open('w') as out:
    subprocess.run(command,stdout=out,stderr=subprocess.STDOUT,check=True)
assert all(sha(p)==h for p,h in inputs.items())
(ROOT/'evidence/bash-printenv-helper.json').write_text(json.dumps({
    'scope':'Standalone GNU 9.11 printenv test oracle. Delegates directly to the unchanged native body, preserving argv[0] even when exec -l prefixes it. Shared helper libraries are the pinned GNU build artifacts; this is not a translated command.',
    'driver_sha256':sha(__file__),'inputs':inputs,'command':command,
    'binary':str(binary),'binary_sha256':sha(binary),'log':str(log),'log_sha256':sha(log)},indent=2)+'\n')
print('Built original standalone printenv helper')
