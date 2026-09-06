/* system-dependent definitions for coreutils
   Copyright (C) 1989-2026 Free Software Foundation, Inc.

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
void
rboxc_oprintf (char const *program, char const *message, ...)
{
  va_list args;
  char *buf;
  int buflen = -1;

#if defined MANUAL_URL || defined BOLD_MAN_REFS
  va_start (args, message);
  buflen = vasprintf (&buf, message, args);
  va_end (args);
#endif

  if (buflen < 0)
    {
      vprintf (message, args);
      return;
    }

  oputs_ (program, buf);
  free (buf);
}
