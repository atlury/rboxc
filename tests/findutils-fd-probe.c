/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Exercise GNU's private callback policy with ordinary live and closed fds. */
#include RBOXC_FDLEAK_SOURCE
#include <stdint.h>

struct observed { uint64_t mask; int stop; };
static int collect (int fd, void *data)
{
  struct observed *p = data;
  assert (fd >= 64 && fd < 70);
  p->mask |= UINT64_C(1) << (fd - 64);
  return p->stop;
}
int main (void)
{
  int ordinary = open ("/dev/null", O_RDONLY);
  int path = open (".", O_PATH);
  int pipes[2];
  assert (ordinary >= 0 && path >= 0 && pipe (pipes) == 0);
  assert (dup2 (ordinary, 64) == 64);
  assert (dup2 (path, 66) == 66);
  assert (dup2 (pipes[0], 67) == 67);
  assert (dup2 (pipes[1], 68) == 68);
  close (ordinary); close (path); close (pipes[0]); close (pipes[1]);
  assert (fcntl (65, F_GETFD) == -1 && errno == EBADF);
  assert (fcntl (69, F_GETFD) == -1 && errno == EBADF);
  struct observed full = { 0, 0 }, stop = { 0, 17 };
  errno = EDOM;
  int rv = visit_open_fds (64, 70, collect, &full);
  int full_errno = errno;
  errno = EDOM;
  int short_rv = visit_open_fds (64, 70, collect, &stop);
  int short_errno = errno;
  close (64); close (66); close (67); close (68);
  printf ("all=%llu result=%d errno=%d stop=%llu result=%d errno=%d\n",
          (unsigned long long) full.mask, rv, full_errno,
          (unsigned long long) stop.mask, short_rv, short_errno);
  return !(full.mask == 25 && rv == 0 && full_errno == EDOM
           && stop.mask == 1 && short_rv == 17 && short_errno == EDOM);
}
