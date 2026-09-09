"""Release Screen daemon resources after its existing terminal restoration."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

RUST_OWNERSHIP = r'''
extern "C" {
    #[link_name = "atexit"]
    fn rboxc_screen_atexit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;
    #[link_name = "cur_term"]
    static mut RBOXC_SCREEN_CUR_TERM: *mut ::core::ffi::c_void;
    #[link_name = "del_curterm"]
    fn rboxc_screen_del_curterm(term: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    // This behavior-first Linux profile uses the recorded ncurses ABI.
    #[link_name = "_nc_free_tparm"]
    fn rboxc_screen_free_tparm(term: *mut ::core::ffi::c_void);
}
static mut RBOXC_SCREEN_STREAM_OWNER: ::core::ffi::c_int = -1;
static mut RBOXC_SCREEN_STREAMS: [*mut FILE; 3] = [::core::ptr::null_mut(); 3];
static mut RBOXC_SCREEN_STREAM_CLEANUP_REGISTERED: bool = false;
unsafe extern "C" fn rboxc_screen_release_streams() {
    if RBOXC_SCREEN_STREAM_OWNER != getpid() { return; }
    let saved_errno = *libc::__errno_location();
    // Screen uses the termcap API in this command process. Release its current
    // terminfo object only after GNU has restored and flushed the terminal.
    if !RBOXC_SCREEN_CUR_TERM.is_null() {
        // Other cached terminal descriptions can keep ncurses' shared parameter
        // cache alive after del_curterm. No terminal operations follow this exit.
        rboxc_screen_free_tparm(RBOXC_SCREEN_CUR_TERM);
        rboxc_screen_del_curterm(RBOXC_SCREEN_CUR_TERM);
    }
    for slot in 0..3 {
        let stream = RBOXC_SCREEN_STREAMS[slot];
        RBOXC_SCREEN_STREAMS[slot] = ::core::ptr::null_mut();
        if !stream.is_null() { libc::fclose(stream.cast()); }
    }
    *libc::__errno_location() = saved_errno;
}
unsafe fn rboxc_screen_owned_freopen(path: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char, stream: *mut FILE) -> *mut FILE {
    let slot = if stream == stdin { 0 } else if stream == stdout { 1 } else { 2 };
    if RBOXC_SCREEN_STREAM_OWNER != getpid() {
        RBOXC_SCREEN_STREAMS = [::core::ptr::null_mut(); 3];
    }
    // freopen closes the old association even when opening the new file fails.
    RBOXC_SCREEN_STREAMS[slot] = ::core::ptr::null_mut();
    let result = freopen(path, mode, stream);
    if !result.is_null() {
        RBOXC_SCREEN_STREAM_OWNER = getpid();
        RBOXC_SCREEN_STREAMS[slot] = result;
        if !RBOXC_SCREEN_STREAM_CLEANUP_REGISTERED {
            if rboxc_screen_atexit(rboxc_screen_release_streams) != 0 {
                rboxc_screen_release_streams();
                libc::_exit(2);
            }
            RBOXC_SCREEN_STREAM_CLEANUP_REGISTERED = true;
        }
    }
    result
}
'''


def rust_adapter(text):
    for anchor in ['    if freopen(', '        || freopen(']:
        assert text.count(anchor) == (1 if anchor.startswith('    if') else 2)
        text = text.replace(anchor, anchor.replace('freopen(', 'rboxc_screen_owned_freopen('))
    return text + RUST_OWNERSHIP


def socket_adapter(text):
    anchor='int MakeServerSocket(void)\n{'
    assert text.count(anchor)==1
    text=text.replace(anchor, '''static int rboxc_owned_server = -1;
static pid_t rboxc_server_owner = -1;
static bool rboxc_server_cleanup_registered;
static void rboxc_release_server(void)
{
    int saved_errno = errno;
    if (rboxc_server_owner == getpid()) {
        int owned = rboxc_owned_server;
        rboxc_owned_server = -1;
        if (owned >= 0) {
            if (ServerSocket == owned) ServerSocket = -1;
            close(owned);
        }
    }
    errno = saved_errno;
}

'''+anchor)
    anchor='\t\tPanic(errno, "socket");\n\ta.sun_family = AF_UNIX;'
    # MakeClientSocket retains its existing independent ownership.
    assert text.count(anchor)==2
    text=text.replace(anchor, '''\t\tPanic(errno, "socket");
    rboxc_owned_server = s;
    rboxc_server_owner = getpid();
    if (!rboxc_server_cleanup_registered) {
        if (atexit(rboxc_release_server) != 0) {
            rboxc_release_server();
            Panic(0, "Cannot register socket cleanup");
        }
        rboxc_server_cleanup_registered = true;
    }
\ta.sun_family = AF_UNIX;''',1)
    anchor='int RecoverSocket(void)\n{\n\tclose(ServerSocket);'
    assert text.count(anchor)==1
    text=text.replace(anchor,'int RecoverSocket(void)\n{\n\trboxc_release_server();\n\tServerSocket = -1;')
    return text


def prepare(root):
    source=Path('/opt/src/screen-5.0.2/socket.c')
    pin=json.loads((root/'inventory/sources.json').read_text())['screen']
    assert fingerprint(source)==pin['source_and_header_sha256']['socket.c']
    stage=root/'build/screen-daemon-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/'socket.c';adapted.write_text(socket_adapter(source.read_text()))
    records=[json.loads(p.read_text()) for p in (root/'build/screen-cc-records').glob('*.json')]
    records=[r for r in records if r.get('file')==str(source)];assert len(records)==1
    record=records[0];args=record['arguments'].copy();obj=stage/'socket.o'
    args[args.index(str(source))]=str(adapted);args[args.index('-o')+1]=str(obj)
    args+=['-iquote',str(source.parent)]
    log=stage/'build.log'
    with log.open('w') as stream:subprocess.run(args,cwd=record['directory'],stdout=stream,stderr=subprocess.STDOUT,check=True)
    report={'scope':'Track server-socket ownership by process from successful open, including bind-error paths. '
        'Release through an exit callback; invalidate before close and before socket recovery; preserve errno '
        'and borrowed handles in forked children. GNU terminal restoration and socket unlink ordering remain.',
        'source_sha256':fingerprint(source),'adapted_sha256':fingerprint(adapted),'object_sha256':fingerprint(obj),
        'driver_sha256':fingerprint(Path(__file__)),'arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log),
        'terminal_cleanup_reference':'https://invisible-island.net/ncurses/man/curs_termcap.3x.html#h3-Releasing-Memory'}
    (root/'evidence/screen-daemon-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {'socket.o':obj}
