"""Avoid closing descriptors already known closed during daemon startup."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import shutil
import subprocess
from entry_provider_helpers import fingerprint

def adapt(text):
 anchor='#include <fcntl.h>\n';assert text.count(anchor)==1
 text=text.replace(anchor,anchor+'#include <errno.h>\n')
 anchor='      for (i = 0; i < fdlimit; i++)\n\tclose (i);'
 assert text.count(anchor)==1
 return text.replace(anchor,'''      for (i = 0; i < fdlimit; i++)
        if (!(fcntl (i, F_GETFD) < 0 && errno == EBADF))
          close (i);''')

def prepare(root):
 pin=json.loads((root/'inventory/sources.json').read_text())['inetutils']
 source=Path(pin['source'])/'libinetutils/daemon.c'
 assert fingerprint(source)==pin['source_and_header_sha256']['libinetutils/daemon.c']
 stage=root/'build/inetd-daemon-cleanup';stage.mkdir(exist_ok=True)
 adapted=stage/'daemon.c';adapted.write_text(adapt(source.read_text()));output=stage/'daemon.o'
 records=[json.loads(p.read_text()) for p in (root/'build/inetutils-cc-records').glob('*.json')]
 record,=[r for r in records if r.get('file')==str(source)]
 args=record['arguments'].copy()
 args[args.index('-o')+1]=str(output);args[args.index(str(source))]=str(adapted)
 args[args.index('-MF')+1]=str(stage/'daemon.d')
 log=root/'evidence/raw/inetd-daemon-cleanup-build.log'
 with log.open('w') as out:
  subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
 original=root/'build/gnu-inetutils/libinetutils/libinetutils.a';archive=stage/original.name
 shutil.copy2(original,archive)
 subprocess.run(['ar','r',str(archive),str(output)],check=True)
 subprocess.run(['ranlib',str(archive)],check=True)
 report={'scope':'Probe descriptors with F_GETFD and skip only confirmed EBADF entries before GNU daemon closes its original descriptor range. Keep fork, session, chdir, redirection and signal semantics unchanged. Only inetd receives this helper adaptation.',
  'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(source),'adapted_sha256':fingerprint(adapted),
  'object_sha256':fingerprint(output),'original_archive_sha256':fingerprint(original),'adapted_archive_sha256':fingerprint(archive),
  'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
 (root/'evidence/inetd-daemon-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
 return {archive.name:archive}
