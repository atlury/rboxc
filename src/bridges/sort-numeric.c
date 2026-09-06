/* sort - sort lines of text (with all kinds of options).
   Copyright (C) 1988-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.

   Written December 1988 by Mike Haertel.
   The author may be reached (Email) at the address mike@gnu.ai.mit.edu,
   or (US mail) as Mike Haertel c/o Free Software Foundation.

   Ørn E. Hansen added NLS support in 1997.  */
#include <config.h>
#include "system.h"
#include <math.h>
static int
nan_compare (long double a, long double b)
{
  char buf[2][sizeof "-nan""()" + CHAR_BIT * sizeof a];
  snprintf (buf[0], sizeof buf[0], "%Lf", a);
  snprintf (buf[1], sizeof buf[1], "%Lf", b);
  return strcmp (buf[0], buf[1]);
}
int
rboxc_general_numcompare (char const *sa, char const *sb)
{
  /* FIXME: maybe add option to try expensive FP conversion
     only if A and B can't be compared more cheaply/accurately.  */

  char *ea;
  char *eb;
  long double a = strtold (sa, &ea);
  long double b = strtold (sb, &eb);

  /* Put conversion errors at the start of the collating sequence.  */
  if (sa == ea)
    return sb == eb ? 0 : -1;
  if (sb == eb)
    return 1;

  /* Sort numbers in the usual way, where -0 == +0.  Put NaNs after
     conversion errors but before numbers; sort them by internal
     bit-pattern, for lack of a more portable alternative.  */
  return (a < b ? -1
          : a > b ? 1
          : a == b ? 0
          : b == b ? -1
          : a == a ? 1
          : nan_compare (a, b));
}
void rboxc_strtold_end(char const *s, char **end) { (void) strtold(s, end); }
