/* Restore the original sort test's seven-descriptor limit before main. */
/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _GNU_SOURCE 1
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/resource.h>
#include <unistd.h>
#include <valgrind/valgrind.h>
__attribute__((constructor)) static void restore_sort_limit(void)
{
  if (!RUNNING_ON_VALGRIND || !getenv("RBOXC_SORT_FD_LIMIT")) return;
  int saved_errno = errno;
  const char *dir = getenv("RBOXC_SORT_FD_JOURNAL");
  if (!dir || getenv("RBOXC_SORT_FD_LIMIT")[0] != '7') _exit(125);
  struct rlimit limit = {7, 7}, actual;
  if (setrlimit(RLIMIT_NOFILE, &limit) || getrlimit(RLIMIT_NOFILE, &actual)) {
    dprintf(2, "sort fd profile: set/get limit failed: %d\n", errno); _exit(125);
  }
  if (actual.rlim_cur != 7 || actual.rlim_max != 7) {
    dprintf(2, "sort fd profile: observed soft=%lu hard=%lu\n", (unsigned long)actual.rlim_cur, (unsigned long)actual.rlim_max); _exit(125);
  }
  char path[4096], record[128];
  int n = snprintf(path, sizeof path, "%s/sort-fd-%ld.json", dir, (long)getpid());
  int len = snprintf(record, sizeof record, "{\"pid\":%ld,\"soft\":7,\"hard\":7,\"before_main\":true}\n", (long)getpid());
  if (n < 0 || (size_t)n >= sizeof path || len < 0 || (size_t)len >= sizeof record) _exit(125);
  int fd = open(path, O_WRONLY|O_CREAT|O_EXCL|O_CLOEXEC, 0600);
  if (fd < 0 || write(fd, record, (size_t)len) != len || close(fd)) {
    dprintf(2, "sort fd profile: journal failed: %d\n", errno); _exit(125);
  }
  if (unsetenv("RBOXC_SORT_FD_LIMIT") || unsetenv("RBOXC_SORT_FD_JOURNAL")) _exit(125);
  errno = saved_errno;
}
