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
