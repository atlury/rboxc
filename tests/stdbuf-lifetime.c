/* Exercise preload-buffer lifetime through ordinary stdio operations. */
/* SPDX-License-Identifier: GPL-3.0-or-later */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char replacement[3][64];

static void late_output(void)
{
  fputs("late output\n", stdout);
  fputs("late error\n", stderr);
}

int main(int argc, char **argv)
{
  if (argc != 2)
    return 2;
  if (strcmp(argv[1], "empty") == 0)
    return 0;
  if (strcmp(argv[1], "replace") == 0)
    {
      if (setvbuf(stdin, replacement[0], _IOFBF, sizeof replacement[0])
          || setvbuf(stdout, replacement[1], _IOFBF, sizeof replacement[1])
          || setvbuf(stderr, replacement[2], _IOFBF, sizeof replacement[2]))
        return 3;
    }
  if (strcmp(argv[1], "reopen") == 0)
    {
      if (!freopen("input", "r", stdin)
          || !freopen("output", "w", stdout)
          || !freopen("error", "w", stderr))
        return 4;
    }
  char input[32];
  if (!fgets(input, sizeof input, stdin) || strcmp(input, "input\n") != 0)
    return 5;
  if (fputs("output\n", stdout) == EOF || fputs("error\n", stderr) == EOF)
    return 6;
  if (strcmp(argv[1], "atexit") == 0 && atexit(late_output))
    return 7;
  if (strcmp(argv[1], "close") == 0 || strcmp(argv[1], "reopen") == 0)
    {
      if (fclose(stdin) || fclose(stdout) || fclose(stderr))
        return 8;
    }
  return 0;
}
