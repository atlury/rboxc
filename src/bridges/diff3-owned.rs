// SPDX-License-Identifier: GPL-3.0-or-later
// GNU diff3 rewires its block lists and keeps their storage until process exit.
// Keep allocation ownership separate from those algorithmic links.
struct RboxcDiff3Allocation {
    pointer: *mut ::core::ffi::c_void,
    next: *mut RboxcDiff3Allocation,
}
static mut RBOXC_DIFF3_ALLOCATIONS: *mut RboxcDiff3Allocation = ::core::ptr::null_mut();
static mut RBOXC_DIFF3_PIPE: [::core::ffi::c_int; 2] = [-1; 2];
static mut RBOXC_DIFF3_MERGE_INPUT: *mut FILE = ::core::ptr::null_mut();
extern "C" {
    fn atexit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;
    fn xalloc_die() -> !;
}
unsafe fn rboxc_diff3_track(pointer: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let record = libc::malloc(::core::mem::size_of::<RboxcDiff3Allocation>())
        .cast::<RboxcDiff3Allocation>();
    if record.is_null() {
        libc::free(pointer);
        xalloc_die();
    }
    record.write(RboxcDiff3Allocation { pointer, next: RBOXC_DIFF3_ALLOCATIONS });
    RBOXC_DIFF3_ALLOCATIONS = record;
    pointer
}
unsafe fn rboxc_diff3_xmalloc(size: size_t) -> *mut ::core::ffi::c_void {
    rboxc_diff3_track(xmalloc(size))
}
unsafe fn rboxc_diff3_ximalloc(size: idx_t) -> *mut ::core::ffi::c_void {
    rboxc_diff3_track(ximalloc(size))
}
unsafe fn rboxc_diff3_xinmalloc(count: idx_t, size: idx_t) -> *mut ::core::ffi::c_void {
    rboxc_diff3_track(xinmalloc(count, size))
}
unsafe fn rboxc_diff3_xicalloc(count: idx_t, size: idx_t) -> *mut ::core::ffi::c_void {
    rboxc_diff3_track(xicalloc(count, size))
}
unsafe fn rboxc_diff3_xpalloc(pointer: *mut ::core::ffi::c_void, count: *mut idx_t,
    minimum: idx_t, maximum: ptrdiff_t, size: idx_t) -> *mut ::core::ffi::c_void {
    let mut record = RBOXC_DIFF3_ALLOCATIONS;
    while !record.is_null() && (*record).pointer != pointer {
        record = (*record).next;
    }
    let replacement = xpalloc(pointer, count, minimum, maximum, size);
    if record.is_null() {
        rboxc_diff3_track(replacement)
    } else {
        (*record).pointer = replacement;
        replacement
    }
}
unsafe fn rboxc_diff3_xfreopen(filename: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char, stream: *mut FILE) {
    xfreopen(filename, mode, stream);
    RBOXC_DIFF3_MERGE_INPUT = stream;
}
unsafe fn rboxc_diff3_exec_failed() {
    let saved_errno = *__errno_location();
    close(STDOUT_FILENO);
    rboxc_diff3_release_owned();
    *__errno_location() = saved_errno;
}
unsafe fn rboxc_diff3_close(descriptor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    for index in 0..2 {
        if RBOXC_DIFF3_PIPE[index] == descriptor {
            RBOXC_DIFF3_PIPE[index] = -1;
        }
    }
    close(descriptor)
}
unsafe extern "C" fn rboxc_diff3_release_owned() {
    let saved_errno = *__errno_location();
    if !RBOXC_DIFF3_MERGE_INPUT.is_null() {
        let stream = RBOXC_DIFF3_MERGE_INPUT;
        RBOXC_DIFF3_MERGE_INPUT = ::core::ptr::null_mut();
        fclose(stream);
    }
    while !RBOXC_DIFF3_ALLOCATIONS.is_null() {
        let record = RBOXC_DIFF3_ALLOCATIONS;
        RBOXC_DIFF3_ALLOCATIONS = (*record).next;
        libc::free((*record).pointer);
        libc::free(record.cast());
    }
    for index in 0..2 {
        if RBOXC_DIFF3_PIPE[index] > 2 {
            rboxc_diff3_close(RBOXC_DIFF3_PIPE[index]);
        }
    }
    *__errno_location() = saved_errno;
}
