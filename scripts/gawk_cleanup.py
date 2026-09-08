"""Finalize only GNU Gawk's owned standard-descriptor replacements."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['gawk']
    source=Path(pin['source'])/'io.c'
    assert fingerprint(source)==pin['source_and_header_sha256']['io.c']
    text=source.read_text()
    anchor='/* remap_std_file --- reopen a standard descriptor on /dev/null */'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''/* Keep the identity of standard descriptors replaced by this command. */
static bool rboxc_owned_standard[3];
static struct stat rboxc_standard_identity[3];
static bool rboxc_standard_cleanup_registered;
static void
rboxc_release_standard_replacements(void)
{
    int saved_errno = errno;
    FILE *streams[3] = { stdin, stdout, stderr };
    for (int fd = 0; fd < 3; fd++) {
        struct stat current;
        if (rboxc_owned_standard[fd]
            && fstat(fd, &current) == 0
            && current.st_dev == rboxc_standard_identity[fd].st_dev
            && current.st_ino == rboxc_standard_identity[fd].st_ino
            && current.st_rdev == rboxc_standard_identity[fd].st_rdev) {
            if (fileno(streams[fd]) == fd)
                fclose(streams[fd]);
            else
                close(fd);
        }
        rboxc_owned_standard[fd] = false;
    }
    errno = saved_errno;
}

'''+anchor)
    start=text.index('static int\nremap_std_file(')
    end=text.index('/* iop_close',start)
    part=text[start:end]
    anchor='\treturn ret;'
    assert part.count(anchor)==1
    part=part.replace(anchor,'''    int saved_errno = errno;
    if (ret >= 0 && oldfd >= 0 && oldfd < 3
        && fstat(oldfd, &rboxc_standard_identity[oldfd]) == 0) {
        rboxc_owned_standard[oldfd] = true;
        if (!rboxc_standard_cleanup_registered) {
            if (atexit(rboxc_release_standard_replacements) != 0)
                _exit(2);
            rboxc_standard_cleanup_registered = true;
        }
    }
    errno = saved_errno;
'''+anchor)
    text=text[:start]+part+text[end:]
    stage=root/'build/gawk-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/'io.c';adapted.write_text(text);output=stage/'io.o'
    records=[json.loads(p.read_text()) for p in (root/'build/gawk-cc-records').glob('*.json')]
    records=[r for r in records if r.get('file')==str(source)];assert len(records)==1
    record=records[0];args=record['arguments'].copy()
    args[args.index('-o')+1]=str(output);args[args.index(str(source))]=str(adapted)
    args+=['-I'+str(source.parent)]
    log=root/'evidence/raw/gawk-owned-standard-build.log'
    with log.open('w') as out:
        subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    report={'scope':'Track successful GNU remaps of standard descriptors to /dev/null and finalize matching owned replacements at exit. Preserve errno and inherited descriptors; skip descriptors replaced with another file.',
            'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(source),
            'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),
            'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/'evidence/gawk-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {'io.o':output,**prepare_source_inputs(root,pin)}


def prepare_source_inputs(root,pin):
    """Close still-owned parser source inputs when a fatal diagnostic exits early."""
    source=Path(pin['source'])/'awkgram.c'
    assert fingerprint(source)==pin['source_and_header_sha256']['awkgram.c']
    text=source.read_text()
    anchor='static int one_line_close(int fd);'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'\nstatic void rboxc_release_parser_inputs(void);\nstatic bool rboxc_parser_cleanup_registered;')
    anchor='\t\tsourcefile->fd = fd;'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'''
        if (!rboxc_parser_cleanup_registered) {
            if (atexit(rboxc_release_parser_inputs) != 0)
                _exit(2);
            rboxc_parser_cleanup_registered = true;
        }''')
    anchor='static FILE *fp = NULL;'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'''

/* Source descriptors remain in the active parser list until GNU closes them.
   Fatal parser diagnostics bypass next_sourcefile. Never close source stdin. */
static void
rboxc_release_parser_inputs(void)
{
    int saved_errno = errno;
    if (srcfiles != NULL) {
        for (SRCFILE *s = srcfiles->next; s != srcfiles; s = s->next) {
            int fd = s->fd;
            if (fd > INVALID_HANDLE && fd != fileno(stdin)) {
                s->fd = INVALID_HANDLE;
                if (fp != NULL && fileno(fp) == fd) {
                    FILE *owned = fp;
                    fp = NULL;
                    fclose(owned);
                } else
                    close(fd);
            }
        }
    }
    errno = saved_errno;
}
''')
    stage=root/'build/gawk-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/'awkgram.c';adapted.write_text(text);output=stage/'awkgram.o'
    records=[json.loads(p.read_text()) for p in (root/'build/gawk-cc-records').glob('*.json')]
    records=[r for r in records if r.get('file')==str(source)];assert len(records)==1
    record=records[0];args=record['arguments'].copy()
    args[args.index('-o')+1]=str(output);args[args.index(str(source))]=str(adapted)
    args+=['-I'+str(source.parent)]
    log=root/'evidence/raw/gawk-source-cleanup-build.log'
    with log.open('w') as out:subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    report={'scope':'Finalize parser-owned source descriptors left open by fatal diagnostics in the active source list. GNU normal parsing still invalidates descriptors as it closes them. Preserve source stdin and errno; close the debug one-line FILE through fclose and clear ownership before closure.',
            'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(source),
            'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),
            'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/'evidence/gawk-source-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {'awkgram.o':output}
