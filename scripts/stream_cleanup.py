"""Pinned GNU standard-stream finalization adaptations for the glibc profile."""
import hashlib
import subprocess


def prepare(root, source, build, target):
    records = []
    objects = []
    for name in ('error', 'closeout'):
        original = (source/f'lib/{name}.c').read_text()
        text = original
        if name == 'error':
            anchor = 'return 0 <= fcntl (fd, F_GETFL);'
            assert text.count(anchor) == 1
            text = text.replace(anchor, 'return 0 <= fcntl (fd, F_GETFD);')
        else:
            anchor = '#include "close-stream.h"'
            assert text.count(anchor) == 1
            text = text.replace(anchor, anchor+'\n#include "fpending.h"\n#include <fcntl.h>')
            anchor = 'void\nclose_stdout (void)\n'
            helper = '''/* stdout and stderr are static FILE objects on this glibc profile.
   With no pending data or stream error, an already-closed descriptor has
   nothing left to flush or close. Preserve close_stream's EBADF exception. */
static int
rboxc_close_standard_stream (FILE *stream)
{
  int saved_errno = errno;
  if (!__fpending (stream) && !ferror (stream)
      && fcntl (fileno (stream), F_GETFD) < 0 && errno == EBADF)
    return 0;
  errno = saved_errno;
  return close_stream (stream);
}

'''
            assert text.count(anchor) == 1
            text = text.replace(anchor, helper+anchor)
            for stream in ('stdout', 'stderr'):
                anchor = f'close_stream ({stream})'
                assert text.count(anchor) == 1
                text = text.replace(anchor, f'rboxc_close_standard_stream ({stream})')
        generated = root/f'src/bridges/{name}.c'
        generated.write_text(text)
        obj = target/f'{name}.o'
        subprocess.run(['gcc', '-O2', '-I'+str(build/'lib'), '-I'+str(source/'lib'),
                        '-c', generated, '-o', obj], check=True)
        objects.append(str(obj))
        records.append({'source': f'lib/{name}.c',
                        'original_sha256': hashlib.sha256(original.encode()).hexdigest(),
                        'adapted_sha256': hashlib.sha256(text.encode()).hexdigest()})
    return objects, records
