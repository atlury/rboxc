/* Test-only exec launcher; preserve shebang arguments and the tested PATH. */
/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _GNU_SOURCE 1
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <valgrind/valgrind.h>

int main(int argc, char **argv)
{
  char self[PATH_MAX], executable[PATH_MAX], real[PATH_MAX], log[PATH_MAX];
  ssize_t length = readlink("/proc/self/exe", self, sizeof self - 1);
  if (length < 0 || (size_t)length >= sizeof self - 1 || argc < 1)
    return 125;
  self[length] = '\0';
  memcpy(executable, self, (size_t)length + 1);
  if (RUNNING_ON_VALGRIND && argc >= 3 && strcmp(argv[1], "--rboxc-dispatch") == 0)
    {
      argc -= 2;
      argv += 2;
    }
  char *slash = strrchr(self, '/');
  if (!slash)
    return 125;
  *slash = '\0';
  char *name = strrchr(argv[0], '/');
  name = name ? name + 1 : argv[0];
  int n = snprintf(real, sizeof real, "%s/../real/%s", self, name);
  int m = snprintf(log, sizeof log, "--log-file=%s/../memory/%%p.log", self);
  if (n < 0 || (size_t)n >= sizeof real || m < 0 || (size_t)m >= sizeof log)
    return 125;
  if (RUNNING_ON_VALGRIND)
    {
      execv(real, argv);
      perror("test launcher exec");
      return 126;
    }
  char **args = calloc((size_t)argc + 10, sizeof *args);
  if (!args)
    return 125;
  char *options[] = {"/usr/bin/valgrind", "--leak-check=full",
                    "--show-leak-kinds=all", "--track-fds=yes",
                    "--trace-children=yes", log, executable, "--rboxc-dispatch"};
  for (size_t i = 0; i < sizeof options / sizeof *options; i++)
    args[i] = options[i];
  for (int i = 0; i < argc; i++)
    args[i + 8] = argv[i];
  execv(args[0], args);
  int saved_errno = errno;
  free(args);
  errno = saved_errno;
  perror("test launcher valgrind");
  return 126;
}
