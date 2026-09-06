/* printf - format and print data
   Copyright (C) 1990-2026 Free Software Foundation, Inc.

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
#include <wchar.h>
#include "cl-strtod.h"
#include "xprintf.h"
static char const *const cfcc_msg =
 N_("warning: %s: character(s) following character constant have been ignored");
#define STRTOX(TYPE, FUNC_NAME, LIB_FUNC_EXPR)				 \
static TYPE								 \
FUNC_NAME (char const *s, bool posixly_correct, void (*verify_numeric)(char const *, char const *))						 \
{									 \
  char *end;								 \
  TYPE val;								 \
                                                                         \
  if ((*s == '\"' || *s == '\'') && *(s + 1))				 \
    {									 \
      unsigned char ch = *++s;						 \
      val = ch;								 \
                                                                         \
      if (MB_CUR_MAX > 1 && *(s + 1))					 \
        {								 \
          mbstate_t mbstate; mbszero (&mbstate);			 \
          wchar_t wc;							 \
          size_t slen = strlen (s);					 \
          ssize_t bytes;						 \
          /* Use mbrtowc not mbrtoc32, as per POSIX.  */		 \
          bytes = mbrtowc (&wc, s, slen, &mbstate);			 \
          if (0 < bytes)						 \
            {								 \
              val = wc;							 \
              s += bytes - 1;						 \
            }								 \
        }								 \
                                                                         \
      /* If POSIXLY_CORRECT is not set, then give a warning that there	 \
         are characters following the character constant and that GNU	 \
         printf is ignoring those characters.  If POSIXLY_CORRECT *is*	 \
         set, then don't give the warning.  */				 \
      if (*++s != 0 && !posixly_correct)				 \
        error (0, 0, _(cfcc_msg), s);					 \
    }									 \
  else									 \
    {									 \
      errno = 0;							 \
      val = (LIB_FUNC_EXPR);						 \
      verify_numeric (s, end);						 \
    }									 \
  return val;								 \
}									 \


STRTOX (long double, vstrtold, cl_strtold (s, &end))
void rboxc_printf_float(char const *p, char const *argument, bool have_field_width, int field_width, bool have_precision, int precision, bool posixly_correct, void (*verify_numeric)(char const *, char const *))
{
        long double arg = argument ? vstrtold (argument, posixly_correct, verify_numeric) : 0;
        if (!have_field_width)
          {
            if (!have_precision)
              xprintf (p, arg);
            else
              xprintf (p, precision, arg);
          }
        else
          {
            if (!have_precision)
              xprintf (p, field_width, arg);
            else
              xprintf (p, field_width, precision, arg);
          }
      }
