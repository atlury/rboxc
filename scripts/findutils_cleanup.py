"""Preserve GNU Findutils behavior while releasing owned files and directories."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import shutil
import subprocess
from findutils_helpers import fingerprint


def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['findutils']
    original=Path(pin['source'])/'lib/fdleak.c'
    assert fingerprint(original)==pin['helper_source_sha256']['fdleak']
    text=original.read_text()
    before='          pf[i].fd = fd_min + i;'
    assert text.count(before)==1
    text=text.replace(before,'''          const int saved_errno = errno;
          const int flags = fcntl (fd_min + i, F_GETFD);
          const int probe_errno = errno;
          errno = saved_errno;
          pf[i].fd = (flags < 0 && probe_errno == EBADF) ? -1 : fd_min + i;''')
    before='              if (pf[j].revents != POLLNVAL)'
    assert text.count(before)==1
    text=text.replace(before,'              if (pf[j].fd >= 0 && pf[j].revents != POLLNVAL)')
    stage=root/'build/findutils-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/'fdleak.c';adapted.write_text(text)
    records=[json.loads(p.read_text()) for p in (root/'build/findutils-cc-records').glob('*.json')]
    records=[r for r in records if Path(r['file'])==original];assert len(records)==1
    record=records[0];arguments=record['arguments'].copy();output=stage/'fdleak.o'
    arguments[arguments.index('-o')+1]=str(output)
    arguments[arguments.index(str(original))]=str(adapted)
    arguments+=['-I'+str(original.parent)]
    log=root/'evidence/raw/findutils-fd-probe-build.log'
    with log.open('w') as out:subprocess.run(arguments,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    archive=stage/'libfind.a';shutil.copy2(root/'build/gnu-findutils/lib/libfind.a',archive)
    members=subprocess.check_output(['ar','t',archive],text=True).splitlines()
    assert members.count('fdleak.o')==1
    subprocess.run(['ar','r',archive,output],check=True);subprocess.run(['ranlib',archive],check=True)
    report={'scope':'Preflight each descriptor with F_GETFD, preserving errno, and mark only known EBADF descriptors negative before GNU poll. The existing polling and callback policy remains in place for valid handles, including O_PATH behavior.',
        'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(original),
        'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),'archive_sha256':fingerprint(archive),
        'compiler_arguments':arguments,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    find_archive=stage/'libfindtools.a'
    shutil.copy2(root/'build/gnu-findutils/find/libfindtools.a',find_archive)
    fixes=[]
    for name in ('parser','exec','util'):
        original=Path(pin['source'])/'find'/(name+'.c')
        assert fingerprint(original)==pin['helper_source_sha256'][name]
        text=original.read_text()
        if name=='exec':
            before='''      if (local)
        free_cwd (execp->wd_for_exec);'''
            assert text.count(before)==1
            text=text.replace(before,'''      if (local)
        {
          free_cwd (execp->wd_for_exec);
          free (execp->wd_for_exec);
          execp->wd_for_exec = NULL;
        }''')
            before='''  if (execp->wd_for_exec->desc < 0)
    return false;'''
            assert text.count(before)==1
            text=text.replace(before,'''  if (execp->wd_for_exec->desc < 0)
    {
      const int saved_errno = errno;
      free (execp->wd_for_exec);
      execp->wd_for_exec = NULL;
      errno = saved_errno;
      return false;
    }''')
        elif name=='parser':
            anchor='static bool\nparse_samefile (const struct parser_table* entry, char **argv, int *arg_ptr)'
            assert text.count(anchor)==1
            text=text.replace(anchor,'''/* Keep reference files open throughout traversal, then release them. */
struct rboxc_samefile_fd { int fd; struct rboxc_samefile_fd *next; };
static struct rboxc_samefile_fd *rboxc_samefile_fds;
static void
rboxc_release_samefile_fds (void)
{
  const int saved_errno = errno;
  while (rboxc_samefile_fds)
    {
      struct rboxc_samefile_fd *p = rboxc_samefile_fds;
      rboxc_samefile_fds = p->next;
      if (p->fd >= 0) close (p->fd);
      free (p);
    }
  errno = saved_errno;
}

'''+anchor)
            start=text.index(anchor);end=text.index('\nstatic bool\nparse_true',start)
            function=text[start:end]
            before='      fd = open_cloexec (filename, openflags);'
            assert function.count(before)==1
            function=function.replace(before,'''      if (!rboxc_samefile_fds && atexit (rboxc_release_samefile_fds))
        error (EXIT_FAILURE, errno, _("The atexit library function failed"));
      struct rboxc_samefile_fd *owned = xmalloc (sizeof *owned);
      owned->fd = -1;
      owned->next = rboxc_samefile_fds;
      rboxc_samefile_fds = owned;
      fd = open_cloexec (filename, openflags);
      owned->fd = fd;''')
            assert function.count('close (fd);')==2
            function=function.replace('close (fd);','close (fd);\n                      owned->fd = -1;')
            text=text[:start]+function+text[end:]
        else:
            start=text.index('void\ncomplete_pending_execs (struct predicate *p)')
            end=text.index('\nvoid\nrecord_initial_cwd',start)
            body=text[start:end]
            anchor='          bc_do_exec (&execp->ctl, &execp->state);'
            assert body.count(anchor)==1
            body=body.replace(anchor,anchor+'''
          /* Exit completion can consume an execdir batch before the
             directory-completion walk sees it.  Release its owned cwd now. */
          if (execp->wd_for_exec && execp->wd_for_exec != initial_wd)
            {
              free_cwd (execp->wd_for_exec);
              free (execp->wd_for_exec);
              execp->wd_for_exec = NULL;
            }''')
            text=text[:start]+body+text[end:]
            anchor='  sharefile_destroy (state.shared_files);'
            assert text.count(anchor)==1
            text=text.replace(anchor,'''  sharefile_handle owned_files = state.shared_files;
  state.shared_files = NULL;
  sharefile_destroy (owned_files);
  free (owned_files);''')
        adapted=stage/(name+'.c');adapted.write_text(text)
        records=[json.loads(p.read_text()) for p in (root/'build/findutils-cc-records').glob('*.json')]
        records=[r for r in records if Path(r['file'])==original];assert len(records)==1
        record=records[0];arguments=record['arguments'].copy();output=stage/(name+'.o')
        arguments[arguments.index('-o')+1]=str(output)
        arguments[arguments.index(str(original))]=str(adapted)
        arguments+=['-I'+str(original.parent),'-g']
        log=root/('evidence/raw/findutils-'+name+'-cleanup-build.log')
        with log.open('w') as out:subprocess.run(arguments,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
        assert subprocess.check_output(['ar','t',find_archive],text=True).splitlines().count(name+'.o')==1
        subprocess.run(['ar','r',find_archive,output],check=True)
        fixes.append({'name':name,'original_sha256':fingerprint(original),'adapted_source_sha256':fingerprint(adapted),
                      'object_sha256':fingerprint(output),'compiler_arguments':arguments,
                      'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)})
    subprocess.run(['ranlib',find_archive],check=True)
    report['find_cleanup']={'scope':'Release samefile reference descriptors at exit, including parse failures; free execdir working-directory storage after each single execution, on open failure, and when the final pending batch is completed.',
                            'archive_sha256':fingerprint(find_archive),'objects':fixes}
    (root/'evidence/findutils-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {'libfind.a':archive,'libfindtools.a':find_archive}
