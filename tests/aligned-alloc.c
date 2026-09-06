/* Copyright (C) 2026 Rbox contributors.
   SPDX-License-Identifier: GPL-3.0-or-later */
#include <assert.h>
#include <errno.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
void *__wrap_aligned_alloc(size_t alignment, size_t size);

int main(void)
{
    size_t alignments[] = {8, 16, 64, 4096};
    size_t sizes[] = {0, 1, 7, 4095, 4096, 4097};
    for (size_t a = 0; a < sizeof alignments / sizeof *alignments; ++a)
        for (size_t s = 0; s < sizeof sizes / sizeof *sizes; ++s) {
            void *p = aligned_alloc(alignments[a], sizes[s]);
            assert(p && (uintptr_t)p % alignments[a] == 0);
            volatile unsigned char *bytes = p;
            for (size_t i = 0; i < sizes[s]; ++i)
                bytes[i] = 0x5a;
            free(p);
        }
    /* Overflow is rejected before attempting a backing allocation. */
    volatile size_t too_large = SIZE_MAX;
    errno = 0;
    assert(__wrap_aligned_alloc(4096, too_large) == NULL && errno == ENOMEM);
    return 0;
}
