/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Own adapter arguments across GNU exit paths, without Rust destructors. */
#include <stdlib.h>
#include <string.h>
#include <limits.h>
static char **arguments;
static char *command_text;
static void release_arguments(void) {
  free(arguments); arguments = NULL;
  free(command_text); command_text = NULL;
}
char **rboxc_shell_arguments(int argc, char **argv, const char *script) {
  if (argc < 1 || argc > INT_MAX - 5 || arguments) return NULL;
  arguments = calloc((size_t)argc + 6, sizeof(char *));
  command_text = strdup(script);
  if (!arguments || !command_text) { release_arguments(); return NULL; }
  if (atexit(release_arguments)) { release_arguments(); return NULL; }
  arguments[0] = "bash";
  arguments[1] = "--noprofile";
  arguments[2] = "--norc";
  arguments[3] = "-c";
  arguments[4] = command_text;
  arguments[5] = argv[0];
  for (int i = 1; i < argc; ++i) arguments[i + 5] = argv[i];
  return arguments;
}

#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>
/* Private command paths let GNU updatedb use ordinary exec/pipeline semantics. */
static char private_directory[] = "/tmp/rboxc-updatedb-XXXXXX";
static pid_t directory_owner;
static const char *tools[] = {"bash", "sh", "find", "frcode", "sort", "sed",
  "rm", "cut", "id", "mktemp", "mkdir", "date", "chmod", "mv", NULL};
static void release_directory(void) {
  if (!directory_owner || getpid() != directory_owner) return;
  char path[PATH_MAX];
  for (size_t i = 0; tools[i]; ++i) {
    snprintf(path, sizeof path, "%s/%s", private_directory, tools[i]);
    unlink(path);
  }
  rmdir(private_directory);
  directory_owner = 0;
}
int rboxc_updatedb_environment(void) {
  char executable[PATH_MAX], path[PATH_MAX];
  ssize_t n = readlink("/proc/self/exe", executable, sizeof executable - 1);
  if (n < 0 || (size_t)n >= sizeof executable - 1) return -1;
  executable[n] = 0;
  if (!mkdtemp(private_directory)) return -1;
  directory_owner = getpid();
  if (atexit(release_directory)) { release_directory(); return -1; }
  for (size_t i = 0; tools[i]; ++i) {
    snprintf(path, sizeof path, "%s/%s", private_directory, tools[i]);
    if (symlink(executable, path)) return -1;
  }
  const char *old_path = getenv("PATH");
  if (!old_path) old_path = "/usr/bin:/bin";
  size_t size = strlen(private_directory) + strlen(old_path) + 2;
  char *new_path = malloc(size);
  if (!new_path) return -1;
  snprintf(new_path, size, "%s:%s", private_directory, old_path);
  int failed = setenv("PATH", new_path, 1);
  free(new_path);
  if (failed) return -1;
  for (size_t i = 0; i < 2; ++i) {
    const char *key = i ? "LIBEXECDIR" : "BINDIR";
    const char *value = getenv(key);
    if ((!value || !*value) && setenv(key, private_directory, 1)) return -1;
  }
  snprintf(path, sizeof path, "%s/sh", private_directory);
  if (!getenv("SHELL") && setenv("SHELL", path, 1)) return -1;
  return 0;
}

#include <errno.h>
#include <sys/stat.h>
/* Only Bash helper references bind to this wrapper. A successful replacement
   becomes Bash-owned; inherited standards and later unrelated files are kept. */
static unsigned char bash_owned_standard[3];
static struct stat bash_standard_identity[3];
static int bash_standard_cleanup_registered;
static void release_bash_standard(void) {
  int saved = errno;
  FILE *streams[3] = {stdin, stdout, stderr};
  for (int fd = 0; fd < 3; ++fd) {
    struct stat current;
    if (bash_owned_standard[fd] && fstat(fd, &current) == 0 &&
        current.st_dev == bash_standard_identity[fd].st_dev &&
        current.st_ino == bash_standard_identity[fd].st_ino &&
        current.st_rdev == bash_standard_identity[fd].st_rdev) {
      if (fileno(streams[fd]) == fd) fclose(streams[fd]);
      else close(fd);
    }
    bash_owned_standard[fd] = 0;
  }
  errno = saved;
}
static void track_bash_standard(int fd) {
  int saved = errno;
  if (fd >= 0 && fd < 3 && fstat(fd, &bash_standard_identity[fd]) == 0) {
    bash_owned_standard[fd] = 1;
    if (!bash_standard_cleanup_registered) {
      if (atexit(release_bash_standard)) _exit(2);
      bash_standard_cleanup_registered = 1;
    }
  }
  errno = saved;
}
int rboxc_bash_owned_dup2(int from, int to) {
  int result = dup2(from, to), saved = errno;
  if (result >= 0 && from != to) track_bash_standard(to);
  errno = saved;
  return result;
}
int rboxc_bash_owned_pipe(int fds[2]) {
  int result = pipe(fds), saved = errno;
  /* A pipe can allocate a closed standard slot without calling dup2. Its
     lifetime and visibility stay unchanged until the shell exits. */
  if (result == 0) {
    track_bash_standard(fds[0]);
    track_bash_standard(fds[1]);
  }
  errno = saved;
  return result;
}

/* Redirection backup descriptors survive fork but belong to the shell that
   created them. Forget normal closes and finalize backups abandoned by a
   command-substitution child. Never restore its parent's redirections. */
struct bash_backup {
  int fd;
  struct stat identity;
  struct bash_backup *next;
};
static struct bash_backup *bash_backups;
static int bash_backup_cleanup_registered;
static void release_bash_backups(void) {
  int saved = errno;
  while (bash_backups) {
    struct bash_backup *item = bash_backups;
    struct stat current;
    bash_backups = item->next;
    if (fstat(item->fd, &current) == 0 &&
        current.st_dev == item->identity.st_dev &&
        current.st_ino == item->identity.st_ino &&
        current.st_rdev == item->identity.st_rdev)
      close(item->fd);
    free(item);
  }
  errno = saved;
}
static void forget_bash_backup(int fd) {
  struct bash_backup **slot = &bash_backups;
  while (*slot) {
    struct bash_backup *item = *slot;
    if (item->fd == fd) { *slot = item->next; free(item); }
    else slot = &item->next;
  }
}
void rboxc_bash_track_backup(int fd) {
  int saved = errno;
  struct stat identity;
  if (fstat(fd, &identity) == 0) {
    forget_bash_backup(fd);
    struct bash_backup *item = malloc(sizeof *item);
    if (!item) _exit(2);
    item->fd = fd; item->identity = identity; item->next = bash_backups;
    bash_backups = item;
    if (!bash_backup_cleanup_registered) {
      if (atexit(release_bash_backups)) _exit(2);
      bash_backup_cleanup_registered = 1;
    }
  }
  errno = saved;
}
int rboxc_bash_owned_close(int fd) {
  int result, saved = errno;
  /* A trace stream can already have closed this descriptor through fclose.
     Preserve close's EBADF result without issuing a duplicate close syscall. */
  if (fcntl(fd, F_GETFD) < 0 && errno == EBADF)
    result = -1;
  else {
    errno = saved;
    result = close(fd);
  }
  saved = errno;
  /* Linux releases the descriptor before reporting late close errors.
     EBADF also means this ownership record is no longer valid. */
  forget_bash_backup(fd);
  errno = saved;
  return result;
}
