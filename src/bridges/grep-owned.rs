// SPDX-License-Identifier: GPL-3.0-or-later
// Release matcher and entry resources after their last use, including
// staged compiler objects when a diagnostic exits before compilation returns.
extern "C" {
    fn GEAfree_all();
    fn Pfree_all();
    fn GEAfree(compiled: *mut ::core::ffi::c_void);
    fn Ffree(compiled: *mut ::core::ffi::c_void);
}
static mut RBOXC_GREP_MATCHER_FREE: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)> = None;
static mut RBOXC_GREP_COLORS: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
static mut RBOXC_GREP_INPUT: ::core::ffi::c_int = -1;
unsafe fn rboxc_grep_close_input(descriptor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if RBOXC_GREP_INPUT == descriptor {
        RBOXC_GREP_INPUT = -1;
    }
    close(descriptor)
}
unsafe extern "C" fn rboxc_grep_release_owned() {
    let saved_errno = *libc::__errno_location();
    if let Some(release) = RBOXC_GREP_MATCHER_FREE {
        if !compiled_pattern.is_null() {
            release(compiled_pattern);
            compiled_pattern = ::core::ptr::null_mut();
        }
    }
    GEAfree_all();
    Pfree_all();
    compiled_pattern = ::core::ptr::null_mut();
    libc::free(RBOXC_GREP_COLORS.cast());
    RBOXC_GREP_COLORS = ::core::ptr::null_mut();
    if RBOXC_GREP_INPUT >= 0 && RBOXC_GREP_INPUT != STDIN_FILENO {
        rboxc_grep_close_input(RBOXC_GREP_INPUT);
    }
    while !RBOXC_GREP_TREES.is_null() {
        rboxc_grep_close_tree((*RBOXC_GREP_TREES).tree);
    }
    rboxc_grep_release_alias();
    libc::free(buffer.cast());
    buffer = ::core::ptr::null_mut();
    *libc::__errno_location() = saved_errno;
}

struct RboxcGrepTree {
    tree: *mut FTS,
    next: *mut RboxcGrepTree,
}
static mut RBOXC_GREP_TREES: *mut RboxcGrepTree = ::core::ptr::null_mut();
unsafe fn rboxc_grep_track_tree(tree: *mut FTS) {
    let record = libc::malloc(::core::mem::size_of::<RboxcGrepTree>()).cast::<RboxcGrepTree>();
    if record.is_null() {
        rpl_fts_close(tree);
        xalloc_die();
        return;
    }
    record.write(RboxcGrepTree { tree, next: RBOXC_GREP_TREES });
    RBOXC_GREP_TREES = record;
}
unsafe fn rboxc_grep_close_tree(tree: *mut FTS) -> ::core::ffi::c_int {
    let mut link = &raw mut RBOXC_GREP_TREES;
    while !(*link).is_null() {
        let record = *link;
        if (*record).tree == tree {
            *link = (*record).next;
            libc::free(record.cast());
            break;
        }
        link = &raw mut (*record).next;
    }
    rpl_fts_close(tree)
}

// PCRE2 JIT can load spare input-buffer bytes within the allocation. Give
// those bytes defined values while retaining GNU's logical input bounds.
unsafe fn rboxc_grep_allocate_buffer(size: idx_t) -> *mut ::core::ffi::c_void {
    let pointer = ximalloc(size);
    libc::memset(pointer, 0, size as usize);
    pointer
}
