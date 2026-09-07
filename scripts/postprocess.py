"""Version-specific C2Rust output repairs; keep transformations reproducible."""
from pathlib import Path
import re
from cleanup import cleanup

def normalize(name, text):
    # Pinned nightly's VaList owns the cursor and directly implements the
    # platform C ABI. GNU va_start/va_copy sites remain explicit clones;
    # passing a list transfers it rather than making a new cursor copy.
    text = text.replace('::core::ffi::VaListImpl', '::core::ffi::VaList')
    text = text.replace('.as_va_list()', '')
    # Keep GNU's reusable formatting helper in C. Its va_list lifetime
    # and error branch cannot be represented by moving a Rust VaList.
    match = re.search(r'#\[inline\]\nunsafe extern "C" fn oprintf_\(', text)
    if match:
        begin = text.index('{', match.start())
        depth = 1
        end = begin + 1
        while depth:
            depth += (text[end] == '{') - (text[end] == '}')
            end += 1
        declaration = ('extern "C" {\n    #[link_name = "rboxc_oprintf"]\n'
                       '    fn oprintf_(program: *const ::core::ffi::c_char, message: *const ::core::ffi::c_char, ...);\n}\n')
        text = text[:match.start()] + declaration + text[end:]
    text = text.replace('::core::ptr::from_exposed_addr_mut', '::core::ptr::with_exposed_provenance_mut')
    text = text.replace('.expose_addr()', '.expose_provenance()')
    if name == 'fmt':
        text = text.replace('r#final', 'final_word').replace('.set_final(', '.set_final_word(')
    if name == 'du':
        text = text.replace('static mut posix_prefix:', 'static mut GNU_POSIX_PREFIX:')
        text = text.replace('&raw const posix_prefix', '&raw const GNU_POSIX_PREFIX')
    if name == 'factor':
        # All remaining table primes have been tried after the last block.
        # Its lookahead has no next prime to inspect: GNU's seven sentinels
        # cover the eight divblock calls, but not every i + 8 lookahead when
        # the preceding double-limb loop leaves an unaligned start index.
        anchor = '        let mut p_0: int_least32_t = primes_ptab[(i + 8 as idx_t) as usize] as int_least32_t;'
        replacement = ('        if i + 8 >= C2Rust_Unnamed_3::PRIMES_PTAB_ENTRIES.0 as idx_t {\n'
                       '            break;\n        }\n'+anchor)
        if replacement not in text:
            assert text.count(anchor) == 1, 'GNU factor table lookahead anchor changed'
            text = text.replace(anchor, replacement, 1)
    if name == 'cp':
        # GNU leaves this directory descriptor for process exit. Release it
        # after the final operand and metadata restoration. Negative values
        # are GNU sentinel descriptors and must not be closed. Preserve errno
        # and the copy result; this descriptor carries no buffered output.
        anchor = '    return ok;\n}\nunsafe extern "C" fn cp_option_init'
        replacement = ('    if target_dirfd >= 0 {\n'
                       '        let saved_errno = *::libc::__errno_location();\n'
                       '        ::libc::close(target_dirfd);\n'
                       '        *::libc::__errno_location() = saved_errno;\n'
                       '    }\n' + anchor)
        if replacement not in text:
            assert text.count(anchor) == 1, 'GNU cp do_copy cleanup anchor changed'
            text = text.replace(anchor, replacement, 1)
    return cleanup(name, text)

if __name__ == '__main__':
    for path in (Path(__file__).resolve().parents[1]/'src/generated').glob('applet_*.rs'):
        text = path.read_text()
        updated = normalize(path.stem.removeprefix('applet_'), text)
        if updated != text:
            path.write_text(updated)
