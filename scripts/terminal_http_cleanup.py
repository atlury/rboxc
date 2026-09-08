"""Adapt owned HTTP connection cleanup and terminal descriptor probing."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def prepare(root,name):
    pin=json.loads((root/'inventory/sources.json').read_text())[name]
    relative={'screen':'misc.c','wget':'src/http.c'}[name]
    source=Path(pin['source'])/relative
    assert fingerprint(source)==pin['source_and_header_sha256'][relative]
    text=source.read_text()
    if name=='screen':
        anchor='\t\t\tpfd[z++].fd = fd;'
        assert text.count(anchor)==1
        text=text.replace(anchor,'\t\t\tpfd[z++].fd = (fcntl(fd, F_GETFD) < 0 && errno == EBADF) ? -1 : fd;')
        anchor='\t\t\tif (!(pfd[z++].revents & POLLNVAL) && fd != except)\n\t\t\t\tclose(fd);'
        assert text.count(anchor)==1
        text=text.replace(anchor,'''            {
                struct pollfd *current = &pfd[z++];
                if (current->fd >= 0 && !(current->revents & POLLNVAL) && fd != except)
                    close(fd);
            }''')
        scope='Filter known-closed descriptors with F_GETFD before GNU Screen polls its descriptor batches. Preserve the original bounds, exception descriptor, poll error handling and closure of valid descriptors.'
    else:
        anchor='static void\nregister_persistent (const char *host, int port, int fd, bool ssl)'
        assert text.count(anchor)==1
        text=text.replace(anchor,'''static bool rboxc_connection_cleanup_registered;
static void
rboxc_release_connection(void)
{
    int saved_errno = errno;
    if (pconn_active) invalidate_persistent();
    errno = saved_errno;
}

'''+anchor)
        anchor='  pconn_active = true;'
        assert text.count(anchor)==1
        text=text.replace(anchor,'''  if (!rboxc_connection_cleanup_registered) {
    if (atexit(rboxc_release_connection) != 0) _exit(2);
    rboxc_connection_cleanup_registered = true;
  }
'''+anchor)
        scope='Release a remaining GNU persistent HTTP connection at exit through its existing invalidation routine. Preserve connection reuse and inherited descriptors.'
    stage=root/f'build/{name}-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/source.name;adapted.write_text(text);output=adapted.with_suffix('.o')
    records=[json.loads(p.read_text()) for p in (root/f'build/{name}-cc-records').glob('*.json')]
    matching=[r for r in records if r.get('file')==str(source)];assert len(matching)==1
    record=matching[0];args=record['arguments'].copy()
    args[args.index('-o')+1]=str(output);args[args.index(str(source))]=str(adapted)
    args+=['-iquote'+str(source.parent)]
    log=root/f'evidence/raw/{name}-descriptor-cleanup-build.log'
    with log.open('w') as out:subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    report={'scope':scope,'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(source),
            'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),
            'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/f'evidence/{name}-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {output.name:output}
