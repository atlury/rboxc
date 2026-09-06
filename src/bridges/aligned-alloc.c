/* Copyright (C) 2026 Rbox contributors.
   SPDX-License-Identifier: GPL-3.0-or-later */
#include <errno.h>
#include <stdint.h>
#include <stdlib.h>

void *__real_aligned_alloc(size_t alignment, size_t size);

/* GNU uses glibc's arbitrary-size extension. Round the backing allocation
   to an alignment multiple so the same requests also satisfy Memcheck's
   aligned_alloc contract. Callers retain their original logical size. */
void *__wrap_aligned_alloc(size_t alignment, size_t size)
{
    /* GNU alignalloc requires a unique nonnull result for a zero size. */
    if (!size)
        size = 1;
    if (alignment && size % alignment) {
        size_t padding = alignment - size % alignment;
        if (SIZE_MAX - size < padding) {
            errno = ENOMEM;
            return NULL;
        }
        size += padding;
    }
    return __real_aligned_alloc(alignment, size);
}
