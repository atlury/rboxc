"""Finalize GNU Patch's owned patch input stream on normal and error exit."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def output_ownership(text):
    """Keep ownership across GNU fatal exits, which bypass Rust stack drops."""
    anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor + '''
    if atexit(Some(rboxc_release_pending_output)) != 0 {
        libc::_exit(2);
    }''')
    anchor = '        if outfd < 0 as ::core::ffi::c_int {'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '        RBOXC_PENDING_OUTPUT_FD = outfd;\n' + anchor)
    anchor = '                if outstate.ofp.is_null() {'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '                RBOXC_PENDING_OUTPUT_STREAM = outstate.ofp;\n' + anchor)
    # Fclose and close consume the resource even on a late write/close error.
    # Forget ownership before calling them, so fatal exit cannot close it twice.
    anchor = '''        if outfile.is_null() {
            if !outstate.ofp.is_null() {
                Fclose(outstate.ofp);'''
    assert text.count(anchor) == 1
    text = text.replace(anchor, '''        RBOXC_PENDING_OUTPUT_FD = -1;
        RBOXC_PENDING_OUTPUT_STREAM = ::core::ptr::null_mut();
''' + anchor)
    text += '''
static mut RBOXC_PENDING_OUTPUT_FD: ::core::ffi::c_int = -1;
static mut RBOXC_PENDING_OUTPUT_STREAM: *mut FILE = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_release_pending_output() {
    let saved_errno = *libc::__errno_location();
    let stream = RBOXC_PENDING_OUTPUT_STREAM;
    let fd = RBOXC_PENDING_OUTPUT_FD;
    RBOXC_PENDING_OUTPUT_STREAM = ::core::ptr::null_mut();
    RBOXC_PENDING_OUTPUT_FD = -1;
    if !stream.is_null() {
        libc::fclose(stream.cast());
    } else if fd >= 0 {
        libc::close(fd);
    }
    *libc::__errno_location() = saved_errno;
}
'''
    return text

def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['patch']
    source=Path(pin['source'])/'src/pch.c'
    assert fingerprint(source)==pin['source_and_header_sha256']['src/pch.c']
    text=source.read_text()
    anchor='/* Open the patch file at the beginning of time. */'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''static int rboxc_pending_patch_fd = -1;
static bool rboxc_patch_cleanup_registered;
static bool rboxc_owned_restored_stdin;
static struct stat rboxc_restored_stdin_identity;
static int
rboxc_restore_patch_stdin(int saved_fd)
{
    int result = dup2(saved_fd, STDIN_FILENO);
    if (result >= 0) {
        int saved_errno = errno;
        rboxc_owned_restored_stdin =
            fstat(STDIN_FILENO, &rboxc_restored_stdin_identity) == 0;
        errno = saved_errno;
    }
    return result;
}
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
    if (rboxc_owned_restored_stdin) {
        struct stat current;
        rboxc_owned_restored_stdin = false;
        if (fstat(STDIN_FILENO, &current) == 0
            && current.st_dev == rboxc_restored_stdin_identity.st_dev
            && current.st_ino == rboxc_restored_stdin_identity.st_ino
            && current.st_mode == rboxc_restored_stdin_identity.st_mode
            && current.st_rdev == rboxc_restored_stdin_identity.st_rdev)
            close(STDIN_FILENO);
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
    anchor='dup2 (stdin_dup, STDIN_FILENO)'
    assert text.count(anchor)==1
    text=text.replace(anchor,'rboxc_restore_patch_stdin (stdin_dup)')
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
    source_merge=Path(pin['source'])/'src/merge.c'
    assert fingerprint(source_merge)==pin['source_and_header_sha256']['src/merge.c']
    donor=Path(json.loads((root/'inventory/sources.json').read_text())['diffutils']['source'])/'src/analyze.c'
    assert fingerprint(donor)=='e6018af7c95c26d4c5538bfbe209a285f5bc91fd206772a097e28243558cccc2'
    merge=source_merge.read_text()
    anchor='  ctxt.heuristic = true;'
    assert merge.count(anchor)==1
    # Match GNU Diffutils' bounded approximate-square-root policy for the
    # shared diffseq context. Patch leaves this required field uninitialized.
    merge=merge.replace(anchor,anchor+'''
  ptrdiff_t too_expensive = 1;
  for (idx_t remaining = diags; remaining; remaining >>= 2)
    too_expensive *= 2;
  ctxt.too_expensive = too_expensive < 4096 ? 4096 : too_expensive;
''')
    texts['merge.c']=merge
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
    report={'scope':'Close owned patch input including pending fdopen ownership; release cached directory descriptors through GNU cache removal after normal/error exit. Finalize stdin only when GNU replaced it through its Ed-script restore dup2 and it still has the recorded identity. Preserve untouched inherited stdin, errno and the original parsing/traversal flow. Initialize the previously indeterminate diffseq too_expensive field using GNU Diffutils bounded approximate-square-root policy.',
            'driver_sha256':fingerprint(Path(__file__)),
            'merge_threshold_reference':{'path':str(donor),'sha256':fingerprint(donor),'provider':'GNU Diffutils 3.12'},
            'adaptations':reports}
    (root/'evidence/patch-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return outputs
