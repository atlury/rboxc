"""Close nohup's replacement input when setup or command execution fails."""


def cleanup_nohup(text, replace_once):
    declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_nohup('
    helper = '''// A successful exec transfers the redirected input to the command.
// If nohup exits instead, it still owns the /dev/null descriptor.
static mut RBOXC_NOHUP_INPUT_OWNED: bool = false;
unsafe extern "C" fn rboxc_close_nohup_input() {
    if RBOXC_NOHUP_INPUT_OWNED {
        RBOXC_NOHUP_INPUT_OWNED = false;
        let saved_errno = *::libc::__errno_location();
        ::libc::close(STDIN_FILENO);
        *::libc::__errno_location() = saved_errno;
    }
}
'''
    text = replace_once(text, declaration, helper + declaration)
    anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
    text = replace_once(text, anchor, anchor + '\n    atexit(Some(rboxc_close_nohup_input));')
    # This follows the fatal check for fd_reopen, before any later setup
    # diagnostic can exit. Inherited nonterminal input is never owned.
    anchor = '        if !redirecting_stdout && !redirecting_stderr {'
    return replace_once(text, anchor,
                        '        RBOXC_NOHUP_INPUT_OWNED = true;\n' + anchor)
