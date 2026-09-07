"""Transfer GNU stdbuf's allocated buffers to glibc's stream ownership."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import subprocess


def prepare(root, source, build, destination):
    original = (source/'src/libstdbuf.c').read_text()
    text = original
    anchor = '#include <stdlib.h>'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor+'''

#if defined __GLIBC__
/* GNU libc's exported buffer-management routine. Its final argument
   transfers malloc-backed storage to the FILE rather than its caller. */
extern void _IO_setb (FILE *, char *, char *, int);
#endif''')
    anchor = '      free (buf);\n    }\n}'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '''      free (buf);
    }
#if defined __GLIBC__
  else if (buf && stream->_IO_buf_base == buf
           && stream->_IO_buf_end == buf + size)
    {
      /* setvbuf records caller ownership. This buffer came from malloc,
         so let glibc release it on close, replacement, or final cleanup.
         Keep the buffer installed until then so pending output is valid. */
      _IO_setb (stream, buf, buf + size, 1);
    }
#endif
}''')
    generated = root/'src/bridges/libstdbuf.c'
    generated.write_text(text)
    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary = destination.with_suffix('.so.tmp')
    subprocess.run(['gcc', '-shared', '-fPIC', '-O2', '-Wall', '-Wextra', '-Werror',
                    '-Wl,-z,defs', '-I'+str(build/'lib'), '-I'+str(source/'lib'),
                    str(generated), '-o', str(temporary)], check=True)
    temporary.replace(destination)
    return {'source': 'src/libstdbuf.c',
            'original_sha256': hashlib.sha256(original.encode()).hexdigest(),
            'adapted_sha256': hashlib.sha256(text.encode()).hexdigest(),
            'scope': 'GNU libc owns successfully installed malloc-backed stdio buffers; other libc behavior unchanged'}
