"""Close GNU's interface-enumeration socket after its final ioctl."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['inetutils']
    source=Path(pin['source'])/'ifconfig/system.c'
    linux=source.parent/'system/linux.c'
    for path in (source,linux):
        assert fingerprint(path)==pin['source_and_header_sha256'][str(path.relative_to(Path(pin['source']))) ]
    text=linux.read_text()
    anchor='  free (content);\n  return idx;'
    assert text.count(anchor)==1
    text=text.replace(anchor,'  int saved_errno = errno;\n  close (fd);\n  errno = saved_errno;\n'+anchor)
    anchor='  if (content == NULL)\n    return NULL;'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''  if (content == NULL) {
    int saved_errno = errno;
    close(fd);
    errno = saved_errno;
    return NULL;
  }''')
    stage=root/'build/ifconfig-cleanup';stage.mkdir(exist_ok=True)
    adapted_linux=stage/'linux.c';adapted_linux.write_text(text)
    adapted=stage/'system.c';adapted.write_text(source.read_text().replace('"system/linux.c"','"'+str(adapted_linux)+'"'))
    records=[json.loads(p.read_text()) for p in (root/'build/inetutils-cc-records').glob('*.json')]
    matching=[r for r in records if r.get('file')==str(source)];assert len(matching)==1
    record=matching[0];args=record['arguments'].copy();output=stage/'system.o'
    args[args.index('-o')+1]=str(output);args[args.index(str(source))]=str(adapted)
    args+=['-I'+str(source.parent),'-I'+str(linux.parent)]
    log=root/'evidence/raw/ifconfig-owned-enumeration-build.log'
    with log.open('w') as out:subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    report={'scope':'Close the Linux interface-enumeration socket on successful completion and failure to read the interface list. Preserve errno and the original interface queries.',
            'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(source),'linux_original_sha256':fingerprint(linux),
            'adapted_source_sha256':fingerprint(adapted),'linux_adapted_sha256':fingerprint(adapted_linux),
            'object_sha256':fingerprint(output),'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/'evidence/ifconfig-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {'system.o':output}
