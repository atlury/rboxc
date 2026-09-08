"""Finalize GNU Patch's owned patch input stream on normal and error exit."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['patch']
    source=Path(pin['source'])/'src/pch.c'
    assert fingerprint(source)==pin['source_and_header_sha256']['src/pch.c']
    text=source.read_text()
    anchor='/* Open the patch file at the beginning of time. */'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''static int rboxc_pending_patch_fd = -1;
static bool rboxc_patch_cleanup_registered;
static void
rboxc_release_patch_input(void)
{
    int saved_errno = errno;
    if (pfp && pfp != stdin) {
        FILE *owned = pfp;
        pfp = NULL;
        fclose(owned);
    }
    if (rboxc_pending_patch_fd >= 0) {
        close(rboxc_pending_patch_fd);
        rboxc_pending_patch_fd = -1;
    }
    errno = saved_errno;
}

'''+anchor)
    anchor='    off_t file_pos = 0;'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''    if (!rboxc_patch_cleanup_registered) {
        if (atexit(rboxc_release_patch_input) != 0)
            _exit(2);
        rboxc_patch_cleanup_registered = true;
    }
'''+anchor)
    anchor='\tpfp = fdopen (fd, "w+b");'
    assert text.count(anchor)==1
    text=text.replace(anchor,'\trboxc_pending_patch_fd = fd;\n'+anchor+'\n\tif (pfp) rboxc_pending_patch_fd = -1;')
    texts={'pch.c':text}
    source_safe=Path(pin['source'])/'src/safe.c'
    assert fingerprint(source_safe)==pin['source_and_header_sha256']['src/safe.c']
    safe=source_safe.read_text()
    anchor='static void init_dirfd_cache (void)'
    assert safe.count(anchor)==1
    safe=safe.replace(anchor,'static void rboxc_release_directory_cache(void);\n\n'+anchor)
    anchor='  if (!cached_dirfds)\n    xalloc_die ();\n}'
    assert safe.count(anchor)==1
    safe=safe.replace(anchor,anchor[:-1]+'  if (atexit(rboxc_release_directory_cache) != 0) _exit(2);\n}')
    safe += """
+static void
+rboxc_release_directory_cache(void)
+{
+    int saved_errno = errno;
+    if (cached_dirfds) hash_free(cached_dirfds);
+    cached_dirfds = NULL;
+    while (!list_empty(&lru_list)) {
+        struct cached_dirfd *entry = list_entry(lru_list.prev,
+            offsetof(struct cached_dirfd, lru_link));
+        list_del(&entry->lru_link);
+        close(entry->fd);
+        free(entry->name);
+        free(entry);
+    }
+    errno = saved_errno;
+}
+""".replace('\n+','\n')
    texts['safe.c']=safe
    stage=root/'build/patch-cleanup';stage.mkdir(exist_ok=True)
    records=[json.loads(p.read_text()) for p in (root/'build/patch-cc-records').glob('*.json')]
    outputs={};reports=[]
    for filename,text in texts.items():
        original=Path(pin['source'])/'src'/filename
        adapted=stage/filename;adapted.write_text(text);output=adapted.with_suffix('.o')
        matching=[r for r in records if r.get('file')==str(original)];assert len(matching)==1
        record=matching[0];args=record['arguments'].copy()
        args[args.index('-o')+1]=str(output);args[args.index(str(original))]=str(adapted)
        args+=['-I'+str(original.parent)]
        log=root/f'evidence/raw/patch-owned-{output.stem}-build.log'
        with log.open('w') as out:
            subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
        outputs[output.name]=output
        reports.append({'original_sha256':fingerprint(original),'adapted_source_sha256':fingerprint(adapted),
            'object_sha256':fingerprint(output),'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)})
    report={'scope':'Close owned patch input including pending fdopen ownership; release cached directory descriptors through GNU cache removal after normal/error exit. Preserve inherited stdin, errno and the original parsing/traversal flow.',
            'driver_sha256':fingerprint(Path(__file__)),'adaptations':reports}
    (root/'evidence/patch-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return outputs
