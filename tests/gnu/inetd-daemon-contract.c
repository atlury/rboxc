/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _GNU_SOURCE
#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <unistd.h>

extern int waitdaemon (int, int, int);

int main (int argc, char **argv)
{
  assert (argc == 5);
  int nochdir = atoi (argv[2]), noclose = atoi (argv[3]);
  int sparse = atoi (argv[4]);
  struct rlimit limit;
  assert (getrlimit (RLIMIT_NOFILE, &limit) == 0);
  limit.rlim_cur = 64;
  assert (setrlimit (RLIMIT_NOFILE, &limit) == 0);
  char before[4096], after[4096];
  assert (getcwd (before, sizeof before));
  /* The invoking fixture passes this descriptor through exec. */
  assert (fcntl (9, F_GETFD) >= 0);
  if (sparse)
    {
      assert (close (STDIN_FILENO) == 0);
      assert (close (STDOUT_FILENO) == 0);
    }
  pid_t parent = getpid ();
  errno = 0;
  int result = waitdaemon (nochdir, noclose, 0);
  int returned_errno = errno;
  assert (result == parent);
  assert (getcwd (after, sizeof after));
  assert (strcmp (after, nochdir ? before : "/") == 0);
  assert (getsid (0) > 0 && getsid (0) != getpid ());
  struct sigaction disposition;
  assert (sigaction (SIGHUP, NULL, &disposition) == 0);
  assert (disposition.sa_handler == SIG_IGN);
  if (noclose)
    {
      assert (fcntl (9, F_GETFD) >= 0);
      for (int fd = 0; fd < 2; fd++)
        assert ((fcntl (fd, F_GETFD) >= 0) == !sparse);
    }
  else
    {
      assert (fcntl (9, F_GETFD) < 0 && errno == EBADF);
      struct stat expected, actual;
      assert (stat ("/dev/null", &expected) == 0);
      for (int fd = 0; fd < 3; fd++)
        {
          assert (fstat (fd, &actual) == 0);
          assert (actual.st_rdev == expected.st_rdev && S_ISCHR (actual.st_mode));
        }
    }
  /* Close the probe's inherited and daemon-reopened standard descriptors. */
  for (int fd = 0; fd < 10; fd++)
    if (fcntl (fd, F_GETFD) >= 0)
      assert (close (fd) == 0);
  FILE *report = fopen (argv[1], "w");
  assert (report);
  assert (fprintf (report, "{\"pass\":true,\"nochdir\":%d,\"noclose\":%d,\"sparse\":%d,\"errno\":%d}\n",
                   nochdir, noclose, sparse, returned_errno) > 0);
  assert (fclose (report) == 0);
  return 0;
}
