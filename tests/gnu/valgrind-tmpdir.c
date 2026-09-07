/* Test-only adapter: restore the client's TMPDIR after Valgrind starts. */
/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _GNU_SOURCE 1
#include <stdlib.h>
#include <unistd.h>
#include <valgrind/valgrind.h>

__attribute__((constructor)) static void restore_tmpdir(void)
{
  if (!RUNNING_ON_VALGRIND)
    return;
  const char *present = getenv("RBOXC_VALGRIND_TMPDIR_PRESENT");
  if (!present)
    return;
  const char *value = getenv("RBOXC_VALGRIND_TMPDIR_VALUE");
  int status;
  if (*present == '1' && value)
    status = setenv("TMPDIR", value, 1);
  else if (*present == '0')
    status = unsetenv("TMPDIR");
  else
    _exit(125);
  if (status != 0 || unsetenv("RBOXC_VALGRIND_TMPDIR_PRESENT") != 0
      || unsetenv("RBOXC_VALGRIND_TMPDIR_VALUE") != 0)
    _exit(125);
}
