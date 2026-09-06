/* od -- dump files in octal and other formats
   Copyright (C) 1992-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
#include <config.h>
#include "system.h"
#include "ftoastr.h"
#include "xprintf.h"
#if FLOAT16_SUPPORTED
  /* Available since clang 6 (2018), and gcc 7 (2017).  */
  typedef _Float16 float16;
#else
# define FLOAT16_SUPPORTED 0
  /* This is just a place-holder to avoid a few '#if' directives.
     In this case, the type isn't actually used.  */
  typedef float float16;
#endif

#if BF16_SUPPORTED
  /* Available since clang 11 (2020), and gcc 13 (2023). */
  typedef __bf16 bfloat16;
#else
# define BF16_SUPPORTED 0
  /* This is just a place-holder to avoid a few '#if' directives.
     In this case, the type isn't actually used.  */
  typedef float bfloat16;
#endif

static idx_t
pad_at (idx_t fields, idx_t i, idx_t pad)
{
  /* This implementation assumes that (FIELDS - 1)^2 does not overflow
     intmax_t, an assumption checked by pad_at_overflow.  */
  intmax_t m = pad % fields;
  return pad / fields * i + m * i / fields;
}
#define PRINT_FIELDS(N, T, FMT_STRING_DECL, ACTION)                     \
void                                                             \
N (idx_t fields, idx_t blank, void const *block,			\
   FMT_STRING_DECL, int width, idx_t pad, bool input_swap)				\
{                                                                       \
  T const *p = block;                                                   \
  idx_t pad_remaining = pad;						\
  for (idx_t i = fields; blank < i; i--)				\
    {                                                                   \
      idx_t next_pad = pad_at (fields, i - 1, pad);			\
      int adjusted_width = pad_remaining - next_pad + width;            \
      T x;                                                              \
      if (input_swap && sizeof (T) > 1)                                 \
        {                                                               \
          union {                                                       \
            T x;                                                        \
            char b[sizeof (T)];                                         \
          } u;                                                          \
          for (idx_t j = 0; j < sizeof (T); j++)			\
            u.b[j] = ((char const *) p)[sizeof (T) - 1 - j];            \
          x = u.x;                                                      \
        }                                                               \
      else                                                              \
        x = *p;                                                         \
      p++;                                                              \
      ACTION;                                                           \
      pad_remaining = next_pad;                                         \
    }                                                                   \
}

#define PRINT_TYPE(N, T)                                                \
  PRINT_FIELDS (N, T, char const *fmt_string,                           \
                xprintf (fmt_string, adjusted_width, x))

#define PRINT_FLOATTYPE(N, T, FTOASTR, BUFSIZE)                         \
  PRINT_FIELDS (N, T, MAYBE_UNUSED char const *fmt_string,              \
                char buf[BUFSIZE];                                      \
                FTOASTR (buf, sizeof buf, 0, 0, x);                     \
                xprintf ("%*s", adjusted_width, buf))

PRINT_FLOATTYPE (rboxc_print_bfloat, bfloat16, ftoastr, FLT_BUFSIZE_BOUND)
PRINT_FLOATTYPE (rboxc_print_halffloat, float16, ftoastr, FLT_BUFSIZE_BOUND)
PRINT_FLOATTYPE (rboxc_print_float, float, ftoastr, FLT_BUFSIZE_BOUND)
PRINT_FLOATTYPE (rboxc_print_double, double, dtoastr, DBL_BUFSIZE_BOUND)
PRINT_FLOATTYPE (rboxc_print_long_double, long double, ldtoastr, LDBL_BUFSIZE_BOUND)
