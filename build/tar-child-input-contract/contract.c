#include <stdbool.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <string.h>
static void xalloc_die(void) { _exit(99); }
static void xclose(int fd) { if (close(fd)) _exit(98); }
#define _(text) text
#define FATAL_ERROR(args) do { (void)e; _exit(97); } while (0)
static pid_t rboxc_input_owner;
static struct stat rboxc_input_identity;
static bool rboxc_input_active;
static bool rboxc_input_cleanup_registered;

static void
rboxc_release_child_input (void)
{
  int saved_errno = errno;
  struct stat current;
  if (rboxc_input_active && rboxc_input_owner == getpid ()
      && fstat (STDIN_FILENO, &current) == 0
      && current.st_dev == rboxc_input_identity.st_dev
      && current.st_ino == rboxc_input_identity.st_ino
      && current.st_mode == rboxc_input_identity.st_mode
      && current.st_rdev == rboxc_input_identity.st_rdev)
    close (STDIN_FILENO);
  rboxc_input_active = false;
  errno = saved_errno;
}

static void
rboxc_record_child_input (void)
{
  int saved_errno = errno;
  rboxc_input_active = fstat (STDIN_FILENO, &rboxc_input_identity) == 0;
  rboxc_input_owner = getpid ();
  if (!rboxc_input_cleanup_registered)
    {
      if (atexit (rboxc_release_child_input))
        xalloc_die ();
      rboxc_input_cleanup_registered = true;
    }
  errno = saved_errno;
}

static void
xdup2 (int from, int into)
{
  if (from != into)
    {
      if (dup2 (from, into) < 0)
	{
	  int e = errno;
	  FATAL_ERROR ((0, e, _("Cannot dup2")));
	}
      xclose (from);
      if (into == STDIN_FILENO)
        rboxc_record_child_input ();
    }
}


static void own_pipe(void) {
  int descriptors[2];
  if (pipe(descriptors)) _exit(90);
  xdup2(descriptors[0], 0);
  xclose(descriptors[1]);
}
static void require_open(void) { if (fcntl(0, F_GETFD) < 0) _exit(91); }
static void require_closed(void) {
  errno = 0;
  if (fcntl(0, F_GETFD) != -1 || errno != EBADF) _exit(92);
}
int main(int argc, char **argv) {
  if (argc != 2) return 89;
  if (!strcmp(argv[1], "failed-stat")) {
    xclose(0);
    rboxc_record_child_input();
    if (open("/dev/null", O_RDONLY) != 0) return 88;
    rboxc_release_child_input();
    require_open();
    xclose(0);
    return 0;
  }
  own_pipe();
  if (!strcmp(argv[1], "inherited")) {
    pid_t child = fork();
    if (child < 0) return 87;
    if (!child) {
      rboxc_release_child_input();
      require_open();
      xclose(0);
      return 0;
    }
    int status;
    if (waitpid(child, &status, 0) != child || status != 0) return 86;
    require_open();
  } else if (!strcmp(argv[1], "reused")) {
    xclose(0);
    if (open("/dev/null", O_RDONLY) != 0) return 85;
    rboxc_release_child_input();
    require_open();
    xclose(0);
    return 0;
  } else if (!strcmp(argv[1], "replaced")) {
    own_pipe();
  } else if (strcmp(argv[1], "owned")) return 84;
  rboxc_release_child_input();
  require_closed();
  return 0;
}
