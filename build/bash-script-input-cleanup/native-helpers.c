/* shell.c -- GNU's idea of the POSIX shell specification. */

/* Copyright (C) 1987-2025 Free Software Foundation, Inc.

   This file is part of GNU Bash, the Bourne Again SHell.

   Bash is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   Bash is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with Bash.  If not, see <http://www.gnu.org/licenses/>.
*/

/*
  Birthdate:
  Sunday, January 10th, 1988.
  Initial author: Brian Fox
*/
#define INSTALL_DEBUG_MODE

#include "config.h"

#include "bashtypes.h"
#if !defined (_MINIX) && defined (HAVE_SYS_FILE_H)
#  include <sys/file.h>
#endif
#include "posixstat.h"
#include "posixtime.h"
#include "bashansi.h"
#include <stdio.h>
#include <signal.h>
#include <errno.h>
#include "filecntl.h"
#if defined (HAVE_PWD_H)
#  include <pwd.h>
#endif

#if defined (HAVE_UNISTD_H)
#  include <unistd.h>
#endif

#include "bashintl.h"

#define NEED_SH_SETLINEBUF_DECL		/* used in externs.h */

#include "shell.h"
extern void rboxc_bash_track_backup (int);
#include "parser.h"
#include "flags.h"
#include "trap.h"
#include "mailcheck.h"
#include "builtins.h"
#include "builtins/common.h"

#if defined (JOB_CONTROL)
#include "jobs.h"
#else
extern int running_in_background;
extern int initialize_job_control (int);
extern int get_tty_state (void);
#endif /* JOB_CONTROL */

#include "input.h"
#include "execute_cmd.h"
#include "findcmd.h"

#if defined (USING_BASH_MALLOC) && defined (DEBUG) && !defined (DISABLE_MALLOC_WRAPPERS)
#  include <malloc/shmalloc.h>
#elif defined (MALLOC_DEBUG) && defined (USING_BASH_MALLOC)
#  include <malloc/shmalloc.h>
#endif

#if defined (HISTORY)
#  include "bashhist.h"
#  include <readline/history.h>
#endif

#if defined (READLINE)
#  include <readline/readline.h>
#  include "bashline.h"
#endif

#include <tilde/tilde.h>
#include <glob/strmatch.h>

#if defined (__OPENNT)
#  include <opennt/opennt.h>
#endif

#if !defined (HAVE_GETPW_DECLS)
extern struct passwd *getpwuid (uid_t);
#endif /* !HAVE_GETPW_DECLS */

#if !defined (errno)
extern int errno;
#endif

#if defined (NO_MAIN_ENV_ARG)
extern char **environ;	/* used if no third argument to main() */
#endif

extern int gnu_error_format;

/* Non-zero means that this shell has already been run; i.e. you should
   call shell_reinitialize () if you need to start afresh. */
int shell_initialized = 0;
int bash_argv_initialized = 0;

COMMAND *global_command = (COMMAND *)NULL;

/* Information about the current user. */
struct user_info current_user =
{
  (uid_t)-1, (uid_t)-1, (uid_t)-1,
  (gid_t)-1, (gid_t)-1, (gid_t)-1,
  (char *)NULL, (char *)NULL, (char *)NULL
};

/* The current host's name. */
char *current_host_name = (char *)NULL;

/* Non-zero means that this shell is a login shell.
   Specifically:
   0 = not login shell.
   1 = login shell from getty (or equivalent fake out)
  -1 = login shell from "--login" (or -l) flag.
  -2 = both from getty, and from flag.
 */
int login_shell = 0;

/* Non-zero if this shell is being run by `su'. */
int su_shell = 0;

/* Non-zero means that at this moment, the shell is interactive.  In
   general, this means that the shell is at this moment reading input
   from the keyboard. */
int interactive = 0;

/* Non-zero means that the shell was started as an interactive shell. */
int interactive_shell = 0;

/* Non-zero means to send a SIGHUP to all jobs when an interactive login
   shell exits. */
int hup_on_exit = 0;

/* Non-zero means to list status of running and stopped jobs at shell exit */
int check_jobs_at_exit = 0;

/* Non-zero means to change to a directory name supplied as a command name */
int autocd = 0;

/* Tells what state the shell was in when it started:
	0 = non-interactive shell script
	1 = interactive
	2 = -c command
	3 = wordexp evaluation
   This is a superset of the information provided by interactive_shell.
*/
int startup_state = 0;
int reading_shell_script = 0;

int ssh_reading_startup_files = 0;

/* Special debugging helper. */
int debugging_login_shell = 0;

/* The environment that the shell passes to other commands. */
char **shell_environment;

/* Non-zero when we are parsing a command, managed by parse_command/parse_comsub */
int parsing_command = 0;

/* Non-zero when we are executing a top-level command. */
int executing = 0;

/* The number of commands executed so far. */
int current_command_number = 1;

/* Non-zero is the recursion depth for commands. */
int indirection_level = 0;

/* The name of this shell, as taken from argv[0]. */
char *shell_name = (char *)NULL;

/* time in seconds when the shell was started */
time_t shell_start_time;
struct timeval shellstart;

/* Are we running in an emacs shell window? */
int running_under_emacs;

/* Do we have /dev/fd? */
#ifdef HAVE_DEV_FD
int have_devfd = HAVE_DEV_FD;
#else
int have_devfd = 0;
#endif

/* The name of the .(shell)rc file, DEFAULT_BASHRC is sourced by default */
 char *bashrc_file;

/* Non-zero means to act more like the Bourne shell on startup. */
 int act_like_sh;

/* Non-zero if we have already expanded and sourced $ENV. */
 int sourced_env;

/* Is this shell running setuid? */
 int running_setuid;

/* Values for the long-winded argument names. */
 int debugging;			/* Do debugging things. */
 int no_rc;			/* Don't execute ~/.bashrc */
 int no_profile;			/* Don't execute .profile */
 int do_version;			/* Display interesting version info. */
 int make_login_shell;		/* Make this shell be a `-bash' shell. */
 int want_initial_help;		/* --help option */

int debugging_mode = 0;		/* In debugging mode with --debugger */
#if defined (READLINE)
int no_line_editing = 0;	/* non-zero -> don't do fancy line editing. */
#else
int no_line_editing = 1;	/* can't have line editing without readline */
#endif
#if defined (TRANSLATABLE_STRINGS)
int dump_translatable_strings;	/* Dump strings in $"...", don't execute. */
int dump_po_strings;		/* Dump strings in $"..." in po format */
#endif
int wordexp_only = 0;		/* Do word expansion only */
int protected_mode = 0;		/* No command substitution with --wordexp */

int pretty_print_mode = 0;	/* pretty-print a shell script */

#if defined (STRICT_POSIX)
int posixly_correct = 1;	/* Non-zero means posix.2 superset. */
#else
int posixly_correct = 0;	/* Non-zero means posix.2 superset. */
#endif

/* Some long-winded argument names.  These are obviously new. */
#define Int 1
#define Charp 2
 const struct {
  const char *name;
  int type;
  int *int_value;
  char **char_value;
} long_args[] = {
  { "debug", Int, &debugging, (char **)0x0 },
#if defined (DEBUGGER)
  { "debugger", Int, &debugging_mode, (char **)0x0 },
#endif
#if defined (TRANSLATABLE_STRINGS)
  { "dump-po-strings", Int, &dump_po_strings, (char **)0x0 },
  { "dump-strings", Int, &dump_translatable_strings, (char **)0x0 },
#endif
  { "help", Int, &want_initial_help, (char **)0x0 },
  { "init-file", Charp, (int *)0x0, &bashrc_file },
  { "login", Int, &make_login_shell, (char **)0x0 },
  { "noediting", Int, &no_line_editing, (char **)0x0 },
  { "noprofile", Int, &no_profile, (char **)0x0 },
  { "norc", Int, &no_rc, (char **)0x0 },
  { "posix", Int, &posixly_correct, (char **)0x0 },
  { "pretty-print", Int, &pretty_print_mode, (char **)0x0 },
#if defined (WORDEXP_OPTION)
  { "protected", Int, &protected_mode, (char **)0x0 },
#endif
  { "rcfile", Charp, (int *)0x0, &bashrc_file },
#if defined (RESTRICTED_SHELL)
  { "restricted", Int, &restricted, (char **)0x0 },
#endif
  { "verbose", Int, &verbose_flag, (char **)0x0 },
  { "version", Int, &do_version, (char **)0x0 },
#if defined (WORDEXP_OPTION)
  { "wordexp", Int, &wordexp_only, (char **)0x0 },
#endif
  { (char *)0x0, Int, (int *)0x0, (char **)0x0 }
};

/* These are extern so execute_simple_command can set them, and then
   longjmp back to main to execute a shell script, instead of calling
   main () again and resulting in indefinite, possibly fatal, stack
   growth. */
procenv_t subshell_top_level;
int subshell_argc;
char **subshell_argv;
char **subshell_envp;

char *exec_argv0;

/* The file descriptor from which the shell is reading input. */
int default_buffered_input = -1;

/* The following two variables are not static so they can show up in $-. */
int read_from_stdin;		/* -s flag supplied */
int want_pending_command;	/* -c flag supplied */

/* This variable is not static so it can be bound to $BASH_EXECUTION_STRING */
char *command_execution_string;	/* argument to -c option */
char *shell_script_filename; 	/* shell script */

int malloc_trace_at_exit = 0;

 int shell_reinitialized = 0;

 FILE *default_input;

 STRING_INT_ALIST *shopt_alist;
 int shopt_ind = 0, shopt_len = 0;

 int parse_long_options (char **, int, int);
 int parse_shell_options (char **, int, int);
 int bind_args (char **, int, int, int);

 void start_debugger (void);

 void add_shopt_to_alist (char *, int);
 void run_shopt_alist (void);

 void execute_env_file (char *);
 void execute_profile_file (void);
 void execute_bashrc_file (void);
 void run_startup_files (void);
 int open_shell_script (char *);
 void set_bash_input (void);
 int run_one_command (char *);
#if defined (WORDEXP_OPTION)
static int run_wordexp (char *);
#endif

 int uidget (void);

 void set_option_defaults (void);
 void reset_option_defaults (void);

 void init_interactive (void);
 void init_noninteractive (void);
 void init_interactive_script (void);

 void set_shell_name (char *);
 void shell_initialize (void);
 void shell_reinitialize (void);

 void show_shell_usage (FILE *, int);

#ifdef __CYGWIN__
static void
_cygwin32_check_tmp (void)
{
  struct stat sb;

  if (stat ("/tmp", &sb) < 0)
    internal_warning (_("could not find /tmp, please create!"));
  else
    {
      if (S_ISDIR (sb.st_mode) == 0)
	internal_warning (_("/tmp must be a valid directory name"));
    }
}
#endif /* __CYGWIN__ */


/* Every GNU call runs under live C jump checkpoints. Rust computation keeps
   asynchronous signals blocked; the logical GNU mask is restored for calls. */
struct rboxc_bash_gate {
  sigset_t allowed, top_mask, sub_mask;
  int top, sub, top_save, sub_save, pending, target;
};
void rboxc_gate_begin (struct rboxc_bash_gate *g) {
  int saved = errno; sigset_t all; sigfillset (&all);
  sigprocmask (SIG_BLOCK, &all, &g->allowed); errno = saved;
}
void rboxc_gate_block (struct rboxc_bash_gate *g) {
  int saved = errno; sigset_t all; sigfillset (&all);
  sigprocmask (SIG_BLOCK, &all, &g->allowed); errno = saved;
}
void rboxc_gate_caught (struct rboxc_bash_gate *g, int sub) {
  int saved = errno;
  if (sub ? g->sub_save : g->top_save)
    sigprocmask (SIG_SETMASK, sub ? &g->sub_mask : &g->top_mask, 0);
  rboxc_gate_block (g); errno = saved;
}
int rboxc_gate_checkpoint (struct rboxc_bash_gate *g, int id, int sub, int save) {
  if (g->target == id) { int code = g->pending; g->target = g->pending = 0; return code; }
  if (sub) { g->sub = id; g->sub_save = save; g->sub_mask = g->allowed; }
  else { g->top = id; g->top_save = save; g->top_mask = g->allowed; }
  return 0;
}

struct rboxc_gate_result_0 { int jumped; int target; int code; };
struct rboxc_gate_result_0 rboxc_gate_call_0 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_1 { int jumped; int target; int code; };
struct rboxc_gate_result_1 rboxc_gate_call_1 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_2 { int jumped; int target; int code; };
struct rboxc_gate_result_2 rboxc_gate_call_2 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_3 { int jumped; int target; int code; unsigned int value; };
struct rboxc_gate_result_3 rboxc_gate_call_3 (struct rboxc_bash_gate *g, unsigned int a0);
struct rboxc_gate_result_4 { int jumped; int target; int code; };
struct rboxc_gate_result_4 rboxc_gate_call_4 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_5 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_5 rboxc_gate_call_5 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_6 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_6 rboxc_gate_call_6 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_7 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_7 rboxc_gate_call_7 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_8 { int jumped; int target; int code; };
struct rboxc_gate_result_8 rboxc_gate_call_8 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_9 { int jumped; int target; int code; };
struct rboxc_gate_result_9 rboxc_gate_call_9 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_10 { int jumped; int target; int code; };
struct rboxc_gate_result_10 rboxc_gate_call_10 (struct rboxc_bash_gate *g, char * a0);
struct rboxc_gate_result_11 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_11 rboxc_gate_call_11 (struct rboxc_bash_gate *g, struct timeval * a0, void * a1);
struct rboxc_gate_result_12 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_12 rboxc_gate_call_12 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2);
struct rboxc_gate_result_13 { int jumped; int target; int code; };
struct rboxc_gate_result_13 rboxc_gate_call_13 (struct rboxc_bash_gate *g, FILE * a0, int a1);
struct rboxc_gate_result_14 { int jumped; int target; int code; };
struct rboxc_gate_result_14 rboxc_gate_call_14 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_15 { int jumped; int target; int code; };
struct rboxc_gate_result_15 rboxc_gate_call_15 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_16 { int jumped; int target; int code; };
struct rboxc_gate_result_16 rboxc_gate_call_16 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_17 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_17 rboxc_gate_call_17 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2);
struct rboxc_gate_result_18 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_18 rboxc_gate_call_18 (struct rboxc_bash_gate *g, char * a0, int a1);
struct rboxc_gate_result_19 { int jumped; int target; int code; };
struct rboxc_gate_result_19 rboxc_gate_call_19 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_20 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_20 rboxc_gate_call_20 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2);
struct rboxc_gate_result_21 { int jumped; int target; int code; };
struct rboxc_gate_result_21 rboxc_gate_call_21 (struct rboxc_bash_gate *g, const char * a0, char * a1);
struct rboxc_gate_result_22 { int jumped; int target; int code; };
struct rboxc_gate_result_22 rboxc_gate_call_22 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_23 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_23 rboxc_gate_call_23 (struct rboxc_bash_gate *g, FILE * a0);
struct rboxc_gate_result_24 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_24 rboxc_gate_call_24 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_25 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_25 rboxc_gate_call_25 (struct rboxc_bash_gate *g, FILE * a0);
struct rboxc_gate_result_26 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_26 rboxc_gate_call_26 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_27 { int jumped; int target; int code; };
struct rboxc_gate_result_27 rboxc_gate_call_27 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_28 { int jumped; int target; int code; };
struct rboxc_gate_result_28 rboxc_gate_call_28 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_29 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_29 rboxc_gate_call_29 (struct rboxc_bash_gate *g, int a0, int a1, int a2);
struct rboxc_gate_result_30 { int jumped; int target; int code; SHELL_VAR * value; };
struct rboxc_gate_result_30 rboxc_gate_call_30 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2);
struct rboxc_gate_result_31 { int jumped; int target; int code; };
struct rboxc_gate_result_31 rboxc_gate_call_31 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_32 { int jumped; int target; int code; };
struct rboxc_gate_result_32 rboxc_gate_call_32 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_33 { int jumped; int target; int code; };
struct rboxc_gate_result_33 rboxc_gate_call_33 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_34 { int jumped; int target; int code; };
struct rboxc_gate_result_34 rboxc_gate_call_34 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_35 { int jumped; int target; int code; };
struct rboxc_gate_result_35 rboxc_gate_call_35 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_36 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_36 rboxc_gate_call_36 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_37 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_37 rboxc_gate_call_37 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_38 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_38 rboxc_gate_call_38 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_39 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_39 rboxc_gate_call_39 (struct rboxc_bash_gate *g, const char * a0, const char * a1);
struct rboxc_gate_result_40 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_40 rboxc_gate_call_40 (struct rboxc_bash_gate *g, const char * a0, const char * a1);
struct rboxc_gate_result_41 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_41 rboxc_gate_call_41 (struct rboxc_bash_gate *g, const char * a0, const char * a1);
struct rboxc_gate_result_42 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_42 rboxc_gate_call_42 (struct rboxc_bash_gate *g, const char * a0, const char * a1);
struct rboxc_gate_result_43 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_43 rboxc_gate_call_43 (struct rboxc_bash_gate *g, const char * a0, const char * a1);
struct rboxc_gate_result_44 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_44 rboxc_gate_call_44 (struct rboxc_bash_gate *g, const char * a0, const char * a1, size_t a2);
struct rboxc_gate_result_45 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_45 rboxc_gate_call_45 (struct rboxc_bash_gate *g, const char * a0, const char * a1, size_t a2);
struct rboxc_gate_result_46 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_46 rboxc_gate_call_46 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_47 { int jumped; int target; int code; };
struct rboxc_gate_result_47 rboxc_gate_call_47 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_48 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_48 rboxc_gate_call_48 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_49 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_49 rboxc_gate_call_49 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_50 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_50 rboxc_gate_call_50 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_51 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_51 rboxc_gate_call_51 (struct rboxc_bash_gate *g, int a0, int a1);
struct rboxc_gate_result_52 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_52 rboxc_gate_call_52 (struct rboxc_bash_gate *g, char * a0);
struct rboxc_gate_result_53 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_53 rboxc_gate_call_53 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2, int a3);
struct rboxc_gate_result_54 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_54 rboxc_gate_call_54 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2, int a3);
struct rboxc_gate_result_55 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_55 rboxc_gate_call_55 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2, int a3);
struct rboxc_gate_result_56 { int jumped; int target; int code; unsigned long value; };
struct rboxc_gate_result_56 rboxc_gate_call_56 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_57 { int jumped; int target; int code; void * value; };
struct rboxc_gate_result_57 rboxc_gate_call_57 (struct rboxc_bash_gate *g, unsigned long a0);
struct rboxc_gate_result_58 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_58 rboxc_gate_call_58 (struct rboxc_bash_gate *g, char * a0, const char * a1);
struct rboxc_gate_result_59 { int jumped; int target; int code; unsigned long value; };
struct rboxc_gate_result_59 rboxc_gate_call_59 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_60 { int jumped; int target; int code; void * value; };
struct rboxc_gate_result_60 rboxc_gate_call_60 (struct rboxc_bash_gate *g, unsigned long a0);
struct rboxc_gate_result_61 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_61 rboxc_gate_call_61 (struct rboxc_bash_gate *g, char * a0, const char * a1);
struct rboxc_gate_result_62 { int jumped; int target; int code; };
struct rboxc_gate_result_62 rboxc_gate_call_62 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_63 { int jumped; int target; int code; };
struct rboxc_gate_result_63 rboxc_gate_call_63 (struct rboxc_bash_gate *g, void * a0);
struct rboxc_gate_result_64 { int jumped; int target; int code; SHELL_VAR * value; };
struct rboxc_gate_result_64 rboxc_gate_call_64 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2);
struct rboxc_gate_result_65 { int jumped; int target; int code; };
struct rboxc_gate_result_65 rboxc_gate_call_65 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_66 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_66 rboxc_gate_call_66 (struct rboxc_bash_gate *g, char * a0);
struct rboxc_gate_result_67 { int jumped; int target; int code; };
struct rboxc_gate_result_67 rboxc_gate_call_67 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_68 { int jumped; int target; int code; };
struct rboxc_gate_result_68 rboxc_gate_call_68 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_69 { int jumped; int target; int code; };
struct rboxc_gate_result_69 rboxc_gate_call_69 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_70 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_70 rboxc_gate_call_70 (struct rboxc_bash_gate *g, char * a0);
struct rboxc_gate_result_71 { int jumped; int target; int code; };
struct rboxc_gate_result_71 rboxc_gate_call_71 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_72 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_72 rboxc_gate_call_72 (struct rboxc_bash_gate *g, char * a0);
struct rboxc_gate_result_73 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_73 rboxc_gate_call_73 (struct rboxc_bash_gate *g, FILE * a0);
struct rboxc_gate_result_74 { int jumped; int target; int code; };
struct rboxc_gate_result_74 rboxc_gate_call_74 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_75 { int jumped; int target; int code; };
struct rboxc_gate_result_75 rboxc_gate_call_75 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_76 { int jumped; int target; int code; };
struct rboxc_gate_result_76 rboxc_gate_call_76 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_77 { int jumped; int target; int code; };
struct rboxc_gate_result_77 rboxc_gate_call_77 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_78 { int jumped; int target; int code; };
struct rboxc_gate_result_78 rboxc_gate_call_78 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_79 { int jumped; int target; int code; };
struct rboxc_gate_result_79 rboxc_gate_call_79 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_80 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_80 rboxc_gate_call_80 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_81 { int jumped; int target; int code; char * value; };
struct rboxc_gate_result_81 rboxc_gate_call_81 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2);
struct rboxc_gate_result_82 { int jumped; int target; int code; };
struct rboxc_gate_result_82 rboxc_gate_call_82 (struct rboxc_bash_gate *g, const char * a0);
struct rboxc_gate_result_83 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_83 rboxc_gate_call_83 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_84 { int jumped; int target; int code; };
struct rboxc_gate_result_84 rboxc_gate_call_84 (struct rboxc_bash_gate *g, int a0);
struct rboxc_gate_result_85 { int jumped; int target; int code; int value; };
struct rboxc_gate_result_85 rboxc_gate_call_85 (struct rboxc_bash_gate *g);
struct rboxc_gate_result_86 { int jumped; int target; int code; };
struct rboxc_gate_result_86 rboxc_gate_call_86 (struct rboxc_bash_gate *g, int a0);
#if defined (NO_MAIN_ENV_ARG)
/* systems without third argument to main() */
int
main (int argc, char **argv)
#else /* !NO_MAIN_ENV_ARG */
int
main (int argc, char **argv, char **env)
#endif /* !NO_MAIN_ENV_ARG */
;


 int
parse_long_options (char **argv, int arg_start, int arg_end)
{
  int arg_index, longarg, i;
  char *arg_string;

  arg_index = arg_start;
  while ((arg_index != arg_end) && (arg_string = argv[arg_index]) &&
	 (*arg_string == '-'))
    {
      longarg = 0;

      /* Make --login equivalent to -login. */
      if (arg_string[1] == '-' && arg_string[2])
	{
	  longarg = 1;
	  arg_string++;
	}

      for (i = 0; long_args[i].name; i++)
	{
	  if (STREQ (arg_string + 1, long_args[i].name))
	    {
	      if (long_args[i].type == Int)
		*long_args[i].int_value = 1;
	      else if (argv[++arg_index] == 0)
		{
		  report_error (_("%s: option requires an argument"), long_args[i].name);
		  exit (EX_BADUSAGE);
		}
	      else
		*long_args[i].char_value = argv[arg_index];

	      break;
	    }
	}
      if (long_args[i].name == 0)
	{
	  if (longarg)
	    {
	      report_error (_("%s: invalid option"), argv[arg_index]);
	      show_shell_usage (stderr, 0);
	      exit (EX_BADUSAGE);
	    }
	  break;		/* No such argument.  Maybe flag arg. */
	}

      arg_index++;
    }

  return (arg_index);
}

 int
parse_shell_options (char **argv, int arg_start, int arg_end)
{
  int arg_index;
  int arg_character, on_or_off, next_arg, i;
  char *o_option, *arg_string;

  arg_index = arg_start;
  while (arg_index != arg_end && (arg_string = argv[arg_index]) &&
	 (*arg_string == '-' || *arg_string == '+'))
    {
      /* There are flag arguments, so parse them. */
      next_arg = arg_index + 1;

      /* A single `-' signals the end of options.  From the 4.3 BSD sh.
	 An option `--' means the same thing; this is the standard
	 getopt(3) meaning. */
      if (arg_string[0] == '-' &&
	   (arg_string[1] == '\0' ||
	     (arg_string[1] == '-' && arg_string[2] == '\0')))
	return (next_arg);

      i = 1;
      on_or_off = arg_string[0];
      while (arg_character = arg_string[i++])
	{
	  switch (arg_character)
	    {
	    case 'c':
	      want_pending_command = 1;
	      break;

	    case 'l':
	      make_login_shell = 1;
	      break;

	    case 's':
	      read_from_stdin = 1;
	      break;

	    case 'o':
	      o_option = argv[next_arg];
	      if (o_option == 0)
		{
		  set_option_defaults ();
		  list_minus_o_opts (-1, (on_or_off == '-') ? 0 : 1);
		  reset_option_defaults ();
		  break;
		}
	      if (set_minus_o_option (on_or_off, o_option) != EXECUTION_SUCCESS)
		exit (EX_BADUSAGE);
	      next_arg++;
	      break;

	    case 'O':
	      /* Since some of these can be overridden by the normal
		 interactive/non-interactive shell initialization or
		 initializing posix mode, we save the options and process
		 them after initialization. */
	      o_option = argv[next_arg];
	      if (o_option == 0)
		{
		  shopt_listopt (o_option, (on_or_off == '-') ? 0 : 1);
		  break;
		}
	      add_shopt_to_alist (o_option, on_or_off);
	      next_arg++;
	      break;

	    case 'D':
#if defined (TRANSLATABLE_STRINGS)
	      dump_translatable_strings = 1;
#endif
	      break;

	    default:
	      if (change_flag (arg_character, on_or_off) == FLAG_ERROR)
		{
		  report_error (_("%c%c: invalid option"), on_or_off, arg_character);
		  show_shell_usage (stderr, 0);
		  exit (EX_BADUSAGE);
		}
	    }
	}
      /* Can't do just a simple increment anymore -- what about
	 "bash -abouo emacs ignoreeof -hP"? */
      arg_index = next_arg;
    }

  return (arg_index);
}

/* Exit the shell with status S. */
void
exit_shell (int s)
{
  fflush (stdout);		/* XXX */
  fflush (stderr);

  /* Clean up the terminal if we are in a state where it's been modified. */
#if defined (READLINE)
  if (bash_readline_initialized && RL_ISSTATE (RL_STATE_TERMPREPPED) && rl_deprep_term_function)
    (*rl_deprep_term_function) ();
#endif
  if (read_tty_modified ())
    read_tty_cleanup ();

  /* Do trap[0] if defined.  Allow it to override the exit status
     passed to us. */
  if (signal_is_trapped (0))
    s = run_exit_trap ();

#if defined (PROCESS_SUBSTITUTION)
  unlink_all_fifos ();
#endif /* PROCESS_SUBSTITUTION */

#if defined (HISTORY)
  if (remember_on_history)
    maybe_save_shell_history ();
#endif /* HISTORY */

#if defined (COPROCESS_SUPPORT)
  coproc_flush ();
#endif

#if defined (JOB_CONTROL)
  /* If the user has run `shopt -s huponexit', hangup all jobs when we exit
     an interactive login shell.  ksh does this unconditionally. */
  if (interactive_shell && login_shell && hup_on_exit)
    hangup_all_jobs ();

  /* If this shell is interactive, or job control is active, terminate all
     stopped jobs and restore the original terminal process group.  Don't do
     this if we're in a subshell and calling exit_shell after, for example,
     a failed word expansion.  We want to do this even if the shell is not
     interactive because we set the terminal's process group when job control
     is enabled regardless of the interactive status. */
  if (subshell_environment == 0)
    end_job_control ();
#endif /* JOB_CONTROL */

  /* Always return the exit status of the last command to our parent. */
  sh_exit (s);
}

/* A wrapper for exit that (optionally) can do other things, like malloc
   statistics tracing. */
void
sh_exit (int s)
{
#if defined (MALLOC_DEBUG) && defined (USING_BASH_MALLOC)
  if (malloc_trace_at_exit && (subshell_environment & (SUBSHELL_COMSUB|SUBSHELL_PROCSUB)) == 0)
    trace_malloc_stats (get_name_for_error (), NULL);
  /* mlocation_write_table (); */
#endif

  exit (s);
}

/* Exit a subshell, which includes calling the exit trap.  We don't want to
   do any more cleanup, since a subshell is created as an exact copy of its
   parent. */
void
subshell_exit (int s)
{
  fflush (stdout);
  fflush (stderr);

  /* Do trap[0] if defined.  Allow it to override the exit status
     passed to us. */
  last_command_exit_value = s;
  if (signal_is_trapped (0))
    s = run_exit_trap ();

  sh_exit (s);
}

void
set_exit_status (int s)
{
  set_pipestatus_from_exit (last_command_exit_value = s);
}

/* Source the bash startup files.  If POSIXLY_CORRECT is non-zero, we obey
   the Posix.2 startup file rules:  $ENV is expanded, and if the file it
   names exists, that file is sourced.  The Posix.2 rules are in effect
   for interactive shells only. (section 4.56.5.3) */

/* Execute ~/.bashrc for most shells.  Never execute it if
   ACT_LIKE_SH is set, or if NO_RC is set.

   If the executable file "/usr/gnu/src/bash/foo" contains:

   #!/usr/gnu/bin/bash
   echo hello

   then:

	 COMMAND	    EXECUTE BASHRC
	 --------------------------------
	 bash -c foo		NO
	 bash foo		NO
	 foo			NO
	 rsh machine ls		YES (for rsh, which calls `bash -c')
	 rsh machine foo	YES (for shell started by rsh) NO (for foo!)
	 echo ls | bash		NO
	 login			NO
	 bash			YES
*/

 void

execute_env_file (char *env_file)
{
  char *fn;

  if (env_file && *env_file)
    {
      fn = expand_string_unsplit_to_string (env_file, Q_DOUBLE_QUOTES);
      if (fn && *fn)
	maybe_execute_file (fn, 1);
      FREE (fn);
    }
}

/* Execute /etc/profile and one of the personal login shell initialization files. */
 void
execute_profile_file (void)
{
  maybe_execute_file (SYS_PROFILE, 1);

  if (act_like_sh)	/* sh */
    maybe_execute_file ("~/.profile", 1);
  else if ((maybe_execute_file ("~/.bash_profile", 1) == 0) &&
	   (maybe_execute_file ("~/.bash_login", 1) == 0))	/* bash */
    maybe_execute_file ("~/.profile", 1);
}

 void
execute_bashrc_file (void)
{
#ifdef SYS_BASHRC
#  if defined (__OPENNT)
  maybe_execute_file (_prefixInstallPath(SYS_BASHRC, NULL, 0), 1);
#  else
  maybe_execute_file (SYS_BASHRC, 1);
#  endif
#endif

  if (bashrc_file)
    maybe_execute_file (bashrc_file, 1);
  else
    maybe_execute_file (DEFAULT_BASHRC, 1);
}

 void
run_startup_files (void)
{
#if defined (JOB_CONTROL)
  int old_job_control;
#endif
  int sourced_login, run_by_ssh;

  /* TAG:bash-5.3 andrew.gregory.8@gmail.com 2/21/2022 */
  /* get the rshd/sshd case out of the way first. */
  if (interactive_shell == 0 && no_rc == 0 && login_shell == 0 &&
      act_like_sh == 0 && command_execution_string && shell_level < 2)
    {
#ifdef SSH_SOURCE_BASHRC
      run_by_ssh = (find_variable ("SSH_CLIENT") != (SHELL_VAR *)0) ||
		   (find_variable ("SSH2_CLIENT") != (SHELL_VAR *)0);
#else
      run_by_ssh = 0;
#endif

      ssh_reading_startup_files = 0;
      /* If we were run by sshd or we think we were run by rshd, execute
	 ~/.bashrc if we are a top-level shell. */
#if 1	/* TAG:bash-5.3 */
      if ((run_by_ssh || isnetconn (fileno (stdin))) && shell_level < 2)
#else
      if (isnetconn (fileno (stdin) && shell_level < 2)
#endif
	{
	  ssh_reading_startup_files = 1;
	  execute_bashrc_file ();
	  ssh_reading_startup_files = 0;
	  return;
	}
    }

#if defined (JOB_CONTROL)
  /* Startup files should be run without job control enabled. */
  old_job_control = interactive_shell ? set_job_control (0) : 0;
#endif

  sourced_login = 0;

  /* A shell begun with the --login (or -l) flag that is not in posix mode
     runs the login shell startup files, no matter whether or not it is
     interactive.  If NON_INTERACTIVE_LOGIN_SHELLS is defined, run the
     startup files if argv[0][0] == '-' as well. */
#if defined (NON_INTERACTIVE_LOGIN_SHELLS)
  if (login_shell && posixly_correct == 0)
#else
  if (login_shell < 0 && posixly_correct == 0)
#endif
    {
      /* We don't execute .bashrc for login shells. */
      no_rc++;

      /* Execute /etc/profile and one of the personal login shell
	 initialization files. */
      if (no_profile == 0)
	execute_profile_file ();

      sourced_login = 1;
    }

  /* A non-interactive shell not named `sh' and not in posix mode reads and
     executes commands from $BASH_ENV.  If `su' starts a shell with `-c cmd'
     and `-su' as the name of the shell, we want to read the startup files.
     No other non-interactive shells read any startup files. */
  if (interactive_shell == 0 && !(su_shell && login_shell))
    {
      if (posixly_correct == 0 && act_like_sh == 0 && privileged_mode == 0 &&
	    sourced_env++ == 0)
	execute_env_file (get_string_value ("BASH_ENV"));
      return;
    }

  /* Interactive shell or `-su' shell. */
  if (posixly_correct == 0)		  /* bash, sh */
    {
      if (login_shell && sourced_login++ == 0)
	{
	  /* We don't execute .bashrc for login shells. */
	  no_rc++;

	  if (no_profile == 0)
	    execute_profile_file ();
	}

      /* bash */
      if (act_like_sh == 0 && no_rc == 0)
	execute_bashrc_file ();
      /* sh */
      else if (act_like_sh && privileged_mode == 0 && sourced_env++ == 0)
	execute_env_file (get_string_value ("ENV"));
    }
  else		/* bash --posix, sh --posix */
    {
      /* bash and sh */
      if (interactive_shell && privileged_mode == 0 && sourced_env++ == 0)
	execute_env_file (get_string_value ("ENV"));
    }

#if defined (JOB_CONTROL)
  set_job_control (old_job_control);
#endif
}

#if defined (RESTRICTED_SHELL)
/* Return 1 if the shell should be a restricted one based on NAME or the
   value of `restricted'.  Don't actually do anything, just return a
   boolean value. */
int
shell_is_restricted (char *name)
{
  char *temp;

  if (restricted)
    return 1;
  temp = base_pathname (name);
  if (*temp == '-')
    temp++;
  return (STREQ (temp, RESTRICTED_SHELL_NAME));
}

/* Perhaps make this shell a `restricted' one, based on NAME.  If the
   basename of NAME is "rbash", then this shell is restricted.  The
   name of the restricted shell is a configurable option, see config.h.
   In a restricted shell, PATH, SHELL, ENV, and BASH_ENV are read-only
   and non-unsettable.
   Do this also if `restricted' is already set to 1; maybe the shell was
   started with -r. */
int
maybe_make_restricted (char *name)
{
  char *temp;

  temp = base_pathname (name);
  if (*temp == '-')
    temp++;
  if (restricted || (STREQ (temp, RESTRICTED_SHELL_NAME)))
    {
#if defined (RBASH_STATIC_PATH_VALUE)
      bind_variable ("PATH", RBASH_STATIC_PATH_VALUE, 0);
      stupidly_hack_special_variables ("PATH");		/* clear hash table */
#endif
      set_var_read_only ("PATH");
      set_var_read_only ("SHELL");
      set_var_read_only ("ENV");
      set_var_read_only ("BASH_ENV");
      set_var_read_only ("HISTFILE");
      restricted = 1;
    }
  return (restricted);
}
#endif /* RESTRICTED_SHELL */

/* Fetch the current set of uids and gids and return 1 if we're running
   setuid or setgid. */
 int
uidget (void)
{
  uid_t u;

  u = current_user.uid;

#if HAVE_SETRESUID
  (void) getresuid (&current_user.uid, &current_user.euid, &current_user.saveuid);
#else
  current_user.uid = getuid ();
  current_user.euid = current_user.saveuid = geteuid ();
#endif

#if HAVE_SETRESGID
  (void) getresgid (&current_user.gid, &current_user.egid, &current_user.savegid);
#else
  current_user.gid = getgid ();
  current_user.egid = current_user.savegid = getegid ();
#endif

  if (current_user.uid != u)
    {
      FREE (current_user.user_name);
      FREE (current_user.shell);
      FREE (current_user.home_dir);
      current_user.user_name = current_user.shell = current_user.home_dir = NULL;
    }

  /* See whether or not we are running setuid or setgid. */
  return (current_user.uid != current_user.euid) ||
	   (current_user.gid != current_user.egid);
}

void
disable_priv_mode (void)
{
  int e, r;

  r = 0;
#if HAVE_SETRESUID
  if (current_user.euid != current_user.uid || current_user.saveuid != current_user.uid)
    r = setresuid (current_user.uid, current_user.uid, current_user.uid) ;
#else
  if (current_user.euid != current_user.uid)
    r = setuid (current_user.uid);
#endif
  if (r < 0)
    {
      e = errno;
      sys_error (_("cannot set uid to %d: effective uid %d"), current_user.uid, current_user.euid);
#if defined (EXIT_ON_SETUID_FAILURE)
      if (e == EAGAIN)
	exit (e);
#endif
    }

  r = 0;
#if HAVE_SETRESGID
  if (current_user.egid != current_user.gid || current_user.savegid != current_user.gid)
    r = setresgid (current_user.gid, current_user.gid, current_user.gid);
#else
  if (current_user.egid != current_user.gid)
    r = setgid (current_user.gid);
#endif
  if (r < 0)
    sys_error (_("cannot set gid to %d: effective gid %d"), current_user.gid, current_user.egid);

  current_user.euid = current_user.uid;
  current_user.egid = current_user.gid;

  current_user.saveuid = current_user.uid;
  current_user.savegid = current_user.gid;
}

#if defined (WORDEXP_OPTION)
static int
run_wordexp (char *words)
{
  int code, nw, nb;
  WORD_LIST *wl, *tl, *result;

  code = setjmp_nosigs (top_level);

  if (code != NOT_JUMPED)
    {
      switch (code)
	{
	  /* Some kind of throw to top_level has occurred. */
	case FORCE_EOF:
	  return last_command_exit_value = 127;
	case ERREXIT:
	case EXITPROG:
	case EXITBLTIN:
	  return last_command_exit_value;
	case DISCARD:
	  return last_command_exit_value = 1;
	default:
	  command_error ("run_wordexp", CMDERR_BADJUMP, code, 0);
	}
    }

  /* Run it through the parser to get a list of words and expand them */
  if (words && *words)
    {
      with_input_from_string (words, "--wordexp");
      if (parse_command () != 0)
	return (126);
      if (global_command == 0)
	{
	  printf ("0\n0\n");
	  return (0);
	}
      if (global_command->type != cm_simple)
	return (126);
      wl = global_command->value.Simple->words;
      if (protected_mode)
	for (tl = wl; tl; tl = tl->next)
	  tl->word->flags |= W_NOCOMSUB|W_NOPROCSUB;
      result = wl ? expand_words_no_vars (wl) : (WORD_LIST *)0;
    }
  else
    result = (WORD_LIST *)0;

  last_command_exit_value = 0;

  if (result == 0)
    {
      printf ("0\n0\n");
      return (0);
    }

  /* Count up the number of words and bytes, and print them.  Don't count
     the trailing NUL byte. */
  for (nw = nb = 0, wl = result; wl; wl = wl->next)
    {
      nw++;
      nb += strlen (wl->word->word);
    }
  printf ("%u\n%u\n", nw, nb);
  /* Print each word on a separate line.  This will have to be changed when
     the interface to glibc is completed. */
  for (wl = result; wl; wl = wl->next)
    printf ("%s\n", wl->word->word);

  return (0);
}
#endif

#if defined (ONESHOT)
/* Run one command, given as the argument to the -c option.  Tell
   parse_and_execute not to fork for a simple command. */
 int
run_one_command (char *command)
{
  int code;

  code = setjmp_nosigs (top_level);

  if (code != NOT_JUMPED)
    {
#if defined (PROCESS_SUBSTITUTION)
      unlink_fifo_list ();
#endif /* PROCESS_SUBSTITUTION */
      switch (code)
	{
	  /* Some kind of throw to top_level has occurred. */
	case FORCE_EOF:
	  return last_command_exit_value = 127;
	case ERREXIT:
	case EXITPROG:
	case EXITBLTIN:
	  return last_command_exit_value;
	case DISCARD:
	  return last_command_exit_value = 1;
	default:
	  command_error ("run_one_command", CMDERR_BADJUMP, code, 0);
	}
    }
   return (parse_and_execute (savestring (command), "-c", SEVAL_NOHIST|SEVAL_RESETLINE));
}
#endif /* ONESHOT */

 int
bind_args (char **argv, int arg_start, int arg_end, int start_index)
{
  register int i;
  WORD_LIST *args, *tl;

  for (i = arg_start, args = tl = (WORD_LIST *)NULL; i < arg_end; i++)
    {
      if (args == 0)
	args = tl = make_word_list (make_word (argv[i]), args);
      else
	{
	  tl->next = make_word_list (make_word (argv[i]), (WORD_LIST *)NULL);
	  tl = tl->next;
	}
    }

  if (args)
    {
      if (start_index == 0)	/* bind to $0...$n for sh -c command */
	{
	  /* Posix.2 4.56.3 says that the first argument after sh -c command
	     becomes $0, and the rest of the arguments become $1...$n */
	  shell_name = savestring (args->word->word);
	  FREE (dollar_vars[0]);
	  dollar_vars[0] = savestring (args->word->word);
	  remember_args (args->next, 1);
	  if (debugging_mode)
	    {
	      push_args (args->next);	/* BASH_ARGV and BASH_ARGC */
	      bash_argv_initialized = 1;
	    }
	}
      else			/* bind to $1...$n for shell script */
        {
	  remember_args (args, 1);
	  /* We do this unconditionally so something like -O extdebug doesn't
	     do it first.  We're setting the definitive positional params
	     here. */
	  if (debugging_mode)
	    {
	      push_args (args);		/* BASH_ARGV and BASH_ARGC */
	      bash_argv_initialized = 1;
	    }
        }

      dispose_words (args);
    }

  return (i);
}

void
unbind_args (void)
{
  remember_args ((WORD_LIST *)NULL, 1);
  pop_args ();				/* Reset BASH_ARGV and BASH_ARGC */
}

 void
start_debugger (void)
{
#if defined (DEBUGGER) && defined (DEBUGGER_START_FILE)
  int old_errexit;
  int r;

  old_errexit = exit_immediately_on_error;
  exit_immediately_on_error = 0;

  r = force_execute_file (DEBUGGER_START_FILE, 1);
  if (r < 0)
    {
      internal_warning (_("cannot start debugger; debugging mode disabled"));
      debugging_mode = 0;
    }
  error_trace_mode = function_trace_mode = debugging_mode;

  set_shellopts ();
  set_bashopts ();

  exit_immediately_on_error += old_errexit;
#endif
}

static int rboxc_script_cleanup_registered;
static struct stat rboxc_script_identity;
static void rboxc_script_cleanup (void)
{
  int saved = errno;
  struct stat current;
  if (default_buffered_input > 0 && fstat(default_buffered_input, &current) == 0 &&
      current.st_dev == rboxc_script_identity.st_dev && current.st_ino == rboxc_script_identity.st_ino &&
      current.st_rdev == rboxc_script_identity.st_rdev)
    unset_bash_input (0);
  errno = saved;
}

 int
open_shell_script (char *script_name)
{
  int fd, e, fd_is_tty;
  char *filename, *path_filename, *t;
  char sample[80];
  int sample_len;
  struct stat sb;
#if defined (ARRAY_VARS)
  SHELL_VAR *funcname_v, *bash_source_v, *bash_lineno_v;
  ARRAY *funcname_a, *bash_source_a, *bash_lineno_a;
#endif

  filename = savestring (script_name);

  fd = open (filename, O_RDONLY);
  if ((fd < 0) && (errno == ENOENT) && (absolute_program (filename) == 0))
    {
      e = errno;
      /* If it's not in the current directory, try looking through PATH
	 for it. */
      path_filename = find_path_file (script_name);
      if (path_filename)
	{
	  free (filename);
	  filename = path_filename;
	  fd = open (filename, O_RDONLY);
	}
      else
	errno = e;
    }

  if (fd < 0)
    {
      e = errno;
      file_error (filename);
#if defined (JOB_CONTROL)
      end_job_control ();	/* just in case we were run as bash -i script */
#endif
      sh_exit ((e == ENOENT) ? EX_NOTFOUND : EX_NOINPUT);
    }

  /* Directory, binary-file and read diagnostics can exit before the
     normal buffered-input owner is installed. */
  rboxc_bash_track_backup (fd);

  free (dollar_vars[0]);
  dollar_vars[0] = exec_argv0 ? savestring (exec_argv0) : savestring (script_name);
  if (exec_argv0)
    {
      free (exec_argv0);
      exec_argv0 = NULL;
    }

  if (file_isdir (filename))
    {
#if defined (EISDIR)
      errno = EISDIR;
#else
      errno = EINVAL;
#endif
      file_error (filename);
#if defined (JOB_CONTROL)
      end_job_control ();	/* just in case we were run as bash -i script */
#endif
      sh_exit (EX_NOINPUT);
    }

#if defined (ARRAY_VARS)
  GET_ARRAY_FROM_VAR ("FUNCNAME", funcname_v, funcname_a);
  GET_ARRAY_FROM_VAR ("BASH_SOURCE", bash_source_v, bash_source_a);
  GET_ARRAY_FROM_VAR ("BASH_LINENO", bash_lineno_v, bash_lineno_a);

  push_source (bash_source_a, filename);
  if (bash_lineno_a)
    {
      t = itos (executing_line_number ());
      array_push (bash_lineno_a, t);
      free (t);
    }
  array_push (funcname_a, "main");
#endif

#ifdef HAVE_DEV_FD
  fd_is_tty = isatty (fd);
#else
  fd_is_tty = 0;
#endif

  /* Only do this with non-tty file descriptors we can seek on. */
  if (fd_is_tty == 0 && (lseek (fd, 0L, 1) != -1))
    {
      /* Check to see if the `file' in `bash file' is a binary file
	 according to the same tests done by execute_simple_command (),
	 and report an error and exit if it is. */
      sample_len = read (fd, sample, sizeof (sample));
      if (sample_len < 0)
	{
	  e = errno;
	  if ((fstat (fd, &sb) == 0) && S_ISDIR (sb.st_mode))
	    {
#if defined (EISDIR)
	      errno = EISDIR;
	      file_error (filename);
#else	      
	      internal_error (_("%s: Is a directory"), filename);
#endif
	    }
	  else
	    {
	      errno = e;
	      file_error (filename);
	    }
#if defined (JOB_CONTROL)
	  end_job_control ();	/* just in case we were run as bash -i script */
#endif
	  exit (EX_NOEXEC);
	}
      else if (sample_len > 0 && (check_binary_file (sample, sample_len)))
	{
	  internal_error ("%s: %s", filename, _("cannot execute binary file"));
#if defined (JOB_CONTROL)
	  end_job_control ();	/* just in case we were run as bash -i script */
#endif
	  exit (EX_BINARY_FILE);
	}
      /* Now rewind the file back to the beginning. */
      lseek (fd, 0L, 0);
    }

  /* Open the script.  But try to move the file descriptor to a randomly
     large one, in the hopes that any descriptors used by the script will
     not match with ours. */
  fd = move_to_high_fd (fd, 1, -1);

  default_buffered_input = fd;
  SET_CLOSE_ON_EXEC (default_buffered_input);

  /* Just about the only way for this code to be executed is if something
     like `bash -i /dev/stdin' is executed. */
  if (interactive_shell && fd_is_tty)
    {
      dup2 (fd, 0);
      close (fd);
      fd = 0;
      default_buffered_input = 0;
    }
  else if (forced_interactive && fd_is_tty == 0)
    /* But if a script is called with something like `bash -i scriptname',
       we need to do a non-interactive setup here, since we didn't do it
       before. */
    init_interactive_script ();

  free (filename);

  if (default_buffered_input > 0 && fstat(default_buffered_input, &rboxc_script_identity) == 0 && !rboxc_script_cleanup_registered) {
    if (atexit(rboxc_script_cleanup) != 0) _exit(2);
    rboxc_script_cleanup_registered = 1;
  }
  reading_shell_script = 1;
  return (fd);
}

/* Initialize the input routines for the parser. */
 void
set_bash_input (void)
{
  /* Make sure the fd from which we are reading input is not in
     no-delay mode. */
  if (interactive == 0)
    sh_unset_nodelay_mode (default_buffered_input);
  else
    sh_unset_nodelay_mode (fileno (stdin));

  /* with_input_from_stdin really means `with_input_from_readline' */
  if (interactive && no_line_editing == 0)
    with_input_from_stdin ();
  else if (interactive == 0)
    {
      errno = 0;
      with_input_from_buffered_stream (default_buffered_input, dollar_vars[0]);
      if (get_buffered_stream (default_buffered_input) == NULL)
	{
	  last_command_exit_value = EX_NOINPUT;
	  if (errno != 0)
	    sys_error ("%s", _("error creating buffered stream"));
	  else
	    report_error ("%s", _("error creating buffered stream"));
	}
    }
  else
    with_input_from_stream (default_input, dollar_vars[0]);
}

/* Close the current shell script input source and forget about it.  This is
   extern so execute_cmd.c:initialize_subshell() can call it.  If CHECK_ZERO
   is non-zero, we close default_buffered_input even if it's the standard
   input (fd 0). */
void
unset_bash_input (int check_zero)
{
  if ((check_zero && default_buffered_input >= 0) ||
      (check_zero == 0 && default_buffered_input > 0))
    {
      close_buffered_fd (default_buffered_input);
      default_buffered_input = bash_input.location.buffered_fd = -1;
      bash_input.type = st_none;		/* XXX */
    }
}
      

#if !defined (PROGRAM)
#  define PROGRAM "bash"
#endif

 void
set_shell_name (char *argv0)
{
  /* Here's a hack.  If the name of this shell is "sh", then don't do
     any startup files; just try to be more like /bin/sh. */
  shell_name = argv0 ? base_pathname (argv0) : PROGRAM;

  if (argv0 && *argv0 == '-')
    {
      if (*shell_name == '-')
	shell_name++;
      login_shell = 1;
    }

  if (shell_name[0] == 's' && shell_name[1] == 'h' && shell_name[2] == '\0')
    act_like_sh++;
  if (shell_name[0] == 's' && shell_name[1] == 'u' && shell_name[2] == '\0')
    su_shell++;

  shell_name = argv0 ? argv0 : PROGRAM;
  FREE (dollar_vars[0]);
  dollar_vars[0] = savestring (shell_name);

  /* A program may start an interactive shell with
	  "execl ("/bin/bash", "-", NULL)".
     If so, default the name of this shell to our name. */
  if (!shell_name || !*shell_name || (shell_name[0] == '-' && !shell_name[1]))
    shell_name = PROGRAM;
}

/* Some options are initialized to -1 so we have a way to determine whether
   they were set on the command line. This is an issue when listing the option
   values at invocation (`bash -o'), so we set the defaults here and reset
   them after the call to list_minus_o_options (). */
/* XXX - could also do this for histexp_flag, jobs_m_flag */
 void
set_option_defaults (void)
{
#if defined (HISTORY)
  enable_history_list = 0;
#endif
}

 void
reset_option_defaults (void)
{
#if defined (HISTORY)
  enable_history_list = -1;
#endif
}

 void
init_interactive (void)
{
  expand_aliases = expaliases_flag = 1;
  interactive_shell = startup_state = interactive = 1;
#if defined (HISTORY)
  if (enable_history_list == -1)
    enable_history_list = 1;				/* set default  */
  remember_on_history = enable_history_list;
#  if defined (BANG_HISTORY)
  histexp_flag = history_expansion;			/* XXX */
#  endif
#endif
}

 void
init_noninteractive (void)
{
#if defined (HISTORY)
  if (enable_history_list == -1)			/* set default */
    enable_history_list = 0;
  bash_history_reinit (0);
#endif /* HISTORY */
  interactive_shell = startup_state = interactive = 0;
  expand_aliases = expaliases_flag = posixly_correct;	/* XXX - was 0 not posixly_correct */
  no_line_editing = 1;
#if defined (JOB_CONTROL)
  /* Even if the shell is not interactive, enable job control if the -i or
     -m option is supplied at startup. */
  set_job_control (forced_interactive||jobs_m_flag);
#endif /* JOB_CONTROL */
}

 void
init_interactive_script (void)
{
#if defined (HISTORY)
  if (enable_history_list == -1)
    enable_history_list = 1;
#endif
  init_noninteractive ();
  expand_aliases = expaliases_flag = interactive_shell = startup_state = 1;
#if defined (HISTORY)
  remember_on_history = enable_history_list;	/* XXX */
#endif
}

void
get_current_user_info (void)
{
  struct passwd *entry;

  /* Don't fetch this more than once. */
  if (current_user.user_name == 0)
    {
#if defined (__TANDEM)
      entry = getpwnam (getlogin ());
#else
      entry = getpwuid (current_user.uid);
#endif
      if (entry)
	{
	  current_user.user_name = savestring (entry->pw_name);
	  current_user.shell = (entry->pw_shell && entry->pw_shell[0])
				? savestring (entry->pw_shell)
				: savestring ("/bin/sh");
	  current_user.home_dir = savestring (entry->pw_dir);
	}
      else
	{
	  current_user.user_name = _("I have no name!");
	  current_user.user_name = savestring (current_user.user_name);
	  current_user.shell = savestring ("/bin/sh");
	  current_user.home_dir = savestring ("/");
	}
#if defined (HAVE_GETPWENT)
      endpwent ();
#endif
    }
}

/* Do whatever is necessary to initialize the shell.
   Put new initializations in here. */
 void
shell_initialize (void)
{
  char hostname[256];
  int should_be_restricted;

  /* Line buffer output for stderr and stdout. */
  if (shell_initialized == 0)
    {
      sh_setlinebuf (stderr);
      sh_setlinebuf (stdout);
    }

  /* Sort the array of shell builtins so that the binary search in
     find_shell_builtin () works correctly. */
  initialize_shell_builtins ();

  /* Initialize the trap signal handlers before installing our own
     signal handlers.  traps.c:restore_original_signals () is responsible
     for restoring the original default signal handlers.  That function
     is called when we make a new child. */
  initialize_traps ();
  initialize_signals (0);

  /* It's highly unlikely that this will change. */
  if (current_host_name == 0)
    {
      /* Initialize current_host_name. */
      if (gethostname (hostname, 255) < 0)
	current_host_name = "??host??";
      else
	current_host_name = savestring (hostname);
    }

  /* Initialize the stuff in current_user that comes from the password
     file.  We don't need to do this right away if the shell is not
     interactive. */
  if (interactive_shell)
    get_current_user_info ();

  /* Initialize our interface to the tilde expander. */
  tilde_initialize ();

#if defined (RESTRICTED_SHELL)
  should_be_restricted = shell_is_restricted (shell_name);
#endif

  /* Initialize internal and environment variables.  Don't import shell
     functions from the environment if we are running in privileged or
     restricted mode or if the shell is running setuid. */
#if defined (RESTRICTED_SHELL)
  initialize_shell_variables (shell_environment, privileged_mode||restricted||should_be_restricted||running_setuid);
#else
  initialize_shell_variables (shell_environment, privileged_mode||running_setuid);
#endif

  /* Initialize the data structures for storing and running jobs. */
  initialize_job_control (jobs_m_flag);

  /* Initialize input streams to null. */
  initialize_bash_input ();

  initialize_flags ();

  /* Initialize the shell options.  Don't import the shell options
     from the environment variables $SHELLOPTS or $BASHOPTS if we are
     running in privileged or restricted mode or if the shell is running
     setuid. */
#if defined (RESTRICTED_SHELL)
  initialize_shell_options (privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_bashopts (privileged_mode||restricted||should_be_restricted||running_setuid);
#else
  initialize_shell_options (privileged_mode||running_setuid);
  initialize_bashopts (privileged_mode||running_setuid);
#endif
}

/* Function called by main () when it appears that the shell has already
   had some initialization performed.  This is supposed to reset the world
   back to a pristine state, as if we had been exec'ed. */
 void
shell_reinitialize (void)
{
  /* The default shell prompts. */
  primary_prompt = PPROMPT;
  secondary_prompt = SPROMPT;

  /* Things that get 1. */
  current_command_number = 1;

  /* We have decided that the ~/.bashrc file should not be executed
     for the invocation of each shell script.  If the variable $ENV
     (or $BASH_ENV) is set, its value is used as the name of a file
     to source. */
  no_rc = no_profile = 1;

  /* Things that get 0. */
  login_shell = make_login_shell = su_shell = executing = 0;
  debugging = debugging_mode = 0;
  do_version = line_number = last_command_exit_value = 0;
  forced_interactive = interactive_shell = interactive = 0;
  subshell_environment = running_in_background = 0;
  expand_aliases = expaliases_flag = 0;
  bash_argv_initialized = 0;

  /* 20240120 */
  startup_state = reading_shell_script = 0;
  /* XXX - inherit posixly_correct? */

  /* The shell has never done this. Should it? */
#if 0
  reset_shell_flags ();
  reset_shell_options ();
  reset_shopt_options ();
#endif

  /* XXX - should we set jobs_m_flag to 0 here? */

#if defined (HISTORY)
  bash_history_reinit (enable_history_list = 0);
#endif /* HISTORY */

#if defined (RESTRICTED_SHELL)
  restricted = 0;
#endif /* RESTRICTED_SHELL */

  /* Ensure that the default startup file is used.  (Except that we don't
     execute this file for reinitialized shells). */
  bashrc_file = NULL;

  /* Delete all variables and functions.  They will be reinitialized when
     the environment is parsed. */
  delete_all_contexts (shell_variables);
  delete_all_variables (shell_functions);

  reinit_special_variables ();

#if defined (READLINE)
  bashline_reinitialize ();
#endif

  shell_reinitialized = 1;
}

 void
show_shell_usage (FILE *fp, int extra)
{
  int i;
  char *set_opts, *s, *t;

  if (extra)
    fprintf (fp, _("GNU bash, version %s-(%s)\n"), shell_version_string (), MACHTYPE);
  fprintf (fp, _("Usage:\t%s [GNU long option] [option] ...\n\t%s [GNU long option] [option] script-file ...\n"),
	     shell_name, shell_name);
  fputs (_("GNU long options:\n"), fp);
  for (i = 0; long_args[i].name; i++)
    fprintf (fp, "\t--%s\n", long_args[i].name);

  fputs (_("Shell options:\n"), fp);
  fputs (_("\t-ilrsD or -c command or -O shopt_option\t\t(invocation only)\n"), fp);

  for (i = 0, set_opts = 0; shell_builtins[i].name; i++)
    if (STREQ (shell_builtins[i].name, "set"))
      {
	set_opts = savestring (shell_builtins[i].short_doc);
	break;
      }

  if (set_opts)
    {
      s = strchr (set_opts, '[');
      if (s == 0)
	s = set_opts;
      while (*++s == '-')
	;
      t = strchr (s, ']');
      if (t)
	*t = '\0';
      fprintf (fp, _("\t-%s or -o option\n"), s);
      free (set_opts);
    }

  if (extra)
    {
      fprintf (fp, _("Type `%s -c \"help set\"' for more information about shell options.\n"), shell_name);
      fprintf (fp, _("Type `%s -c help' for more information about shell builtin commands.\n"), shell_name);
      fprintf (fp, _("Use the `bashbug' command to report bugs.\n"));
      fprintf (fp, "\n");
      fprintf (fp, _("bash home page: <http://www.gnu.org/software/bash>\n"));
      fprintf (fp, _("General help using GNU software: <http://www.gnu.org/gethelp/>\n"));
    }
}

 void
add_shopt_to_alist (char *opt, int on_or_off)
{
  if (shopt_ind >= shopt_len)
    {
      shopt_len += 8;
      shopt_alist = (STRING_INT_ALIST *)xrealloc (shopt_alist, shopt_len * sizeof (shopt_alist[0]));
    }
  shopt_alist[shopt_ind].word = opt;
  shopt_alist[shopt_ind].token = on_or_off;
  shopt_ind++;
}

 void
run_shopt_alist (void)
{
  register int i;

  for (i = 0; i < shopt_ind; i++)
    if (shopt_setopt (shopt_alist[i].word, (shopt_alist[i].token == '-')) != EXECUTION_SUCCESS)
      exit (EX_BADUSAGE);
  free (shopt_alist);
  shopt_alist = 0;
  shopt_ind = shopt_len = 0;
}

struct rboxc_gate_result_0 rboxc_gate_call_0 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_0 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_0 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_0 result = {0};
    exit (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_1 rboxc_gate_call_1 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_1 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_1 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_1 result = {0};
    xtrace_init ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_2 rboxc_gate_call_2 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_2 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_2 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_2 result = {0};
    check_dev_tty ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_3 rboxc_gate_call_3 (struct rboxc_bash_gate *g, unsigned int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_3 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_3 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_3 result = {0};
    result.value = sleep (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_4 rboxc_gate_call_4 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_4 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_4 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_4 result = {0};
    set_default_locale ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_5 rboxc_gate_call_5 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_5 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_5 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_5 result = {0};
    result.value = uidget ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_6 rboxc_gate_call_6 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_6 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_6 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_6 result = {0};
    result.value = getenv (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_7 rboxc_gate_call_7 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_7 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_7 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_7 result = {0};
    result.value = getenv (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_8 rboxc_gate_call_8 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_8 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_8 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_8 result = {0};
    shell_reinitialize ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_9 rboxc_gate_call_9 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_9 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_9 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_9 result = {0};
    exit (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_10 rboxc_gate_call_10 (struct rboxc_bash_gate *g, char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_10 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_10 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_10 result = {0};
    set_shell_name (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_11 rboxc_gate_call_11 (struct rboxc_bash_gate *g, struct timeval * a0, void * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_11 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_11 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_11 result = {0};
    result.value = gettimeofday (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_12 rboxc_gate_call_12 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_12 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_12 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_12 result = {0};
    result.value = parse_long_options (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_13 rboxc_gate_call_13 (struct rboxc_bash_gate *g, FILE * a0, int a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_13 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_13 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_13 result = {0};
    show_shell_usage (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_14 rboxc_gate_call_14 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_14 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_14 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_14 result = {0};
    exit (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_15 rboxc_gate_call_15 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_15 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_15 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_15 result = {0};
    show_shell_version (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_16 rboxc_gate_call_16 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_16 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_16 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_16 result = {0};
    exit (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_17 rboxc_gate_call_17 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_17 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_17 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_17 result = {0};
    result.value = parse_shell_options (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_18 rboxc_gate_call_18 (struct rboxc_bash_gate *g, char * a0, int a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_18 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_18 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_18 result = {0};
    result.value = set_login_shell (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_19 rboxc_gate_call_19 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_19 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_19 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_19 result = {0};
    disable_priv_mode ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_20 rboxc_gate_call_20 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_20 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_20 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_20 result = {0};
    result.value = dcgettext (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_21 rboxc_gate_call_21 (struct rboxc_bash_gate *g, const char * a0, char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_21 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_21 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_21 result = {0};
    report_error (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_22 rboxc_gate_call_22 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_22 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_22 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_22 result = {0};
    exit (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_23 rboxc_gate_call_23 (struct rboxc_bash_gate *g, FILE * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_23 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_23 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_23 result = {0};
    result.value = fileno (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_24 rboxc_gate_call_24 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_24 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_24 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_24 result = {0};
    result.value = isatty (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_25 rboxc_gate_call_25 (struct rboxc_bash_gate *g, FILE * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_25 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_25 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_25 result = {0};
    result.value = fileno (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_26 rboxc_gate_call_26 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_26 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_26 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_26 result = {0};
    result.value = isatty (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_27 rboxc_gate_call_27 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_27 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_27 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_27 result = {0};
    init_interactive ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_28 rboxc_gate_call_28 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_28 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_28 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_28 result = {0};
    init_noninteractive ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_29 rboxc_gate_call_29 (struct rboxc_bash_gate *g, int a0, int a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_29 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_29 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_29 result = {0};
    result.value = fcntl (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_30 rboxc_gate_call_30 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_30 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_30 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_30 result = {0};
    result.value = bind_variable (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_31 rboxc_gate_call_31 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_31 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_31 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_31 result = {0};
    sv_strict_posix (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_32 rboxc_gate_call_32 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_32 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_32 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_32 result = {0};
    run_shopt_alist ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_33 rboxc_gate_call_33 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_33 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_33 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_33 result = {0};
    shell_initialize ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_34 rboxc_gate_call_34 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_34 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_34 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_34 result = {0};
    set_default_lang ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_35 rboxc_gate_call_35 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_35 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_35 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_35 result = {0};
    set_default_locale_vars ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_36 rboxc_gate_call_36 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_36 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_36 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_36 result = {0};
    result.value = get_string_value (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_37 rboxc_gate_call_37 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_37 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_37 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_37 result = {0};
    result.value = get_string_value (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_38 rboxc_gate_call_38 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_38 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_38 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_38 result = {0};
    result.value = get_string_value (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_39 rboxc_gate_call_39 (struct rboxc_bash_gate *g, const char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_39 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_39 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_39 result = {0};
    result.value = strstr (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_40 rboxc_gate_call_40 (struct rboxc_bash_gate *g, const char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_40 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_40 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_40 result = {0};
    result.value = strstr (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_41 rboxc_gate_call_41 (struct rboxc_bash_gate *g, const char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_41 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_41 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_41 result = {0};
    result.value = STREQ (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_42 rboxc_gate_call_42 (struct rboxc_bash_gate *g, const char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_42 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_42 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_42 result = {0};
    result.value = STREQ (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_43 rboxc_gate_call_43 (struct rboxc_bash_gate *g, const char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_43 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_43 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_43 result = {0};
    result.value = STREQ (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_44 rboxc_gate_call_44 (struct rboxc_bash_gate *g, const char * a0, const char * a1, size_t a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_44 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_44 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_44 result = {0};
    result.value = STREQN (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_45 rboxc_gate_call_45 (struct rboxc_bash_gate *g, const char * a0, const char * a1, size_t a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_45 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_45 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_45 result = {0};
    result.value = STREQN (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_46 rboxc_gate_call_46 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_46 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_46 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_46 result = {0};
    result.value = compat_init ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_47 rboxc_gate_call_47 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_47 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_47 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_47 result = {0};
    exit_shell (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_48 rboxc_gate_call_48 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_48 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_48 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_48 result = {0};
    result.value = set_job_control (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_49 rboxc_gate_call_49 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_49 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_49 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_49 result = {0};
    result.value = unbind_variable (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_50 rboxc_gate_call_50 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_50 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_50 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_50 result = {0};
    result.value = unbind_variable (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_51 rboxc_gate_call_51 (struct rboxc_bash_gate *g, int a0, int a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_51 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_51 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_51 result = {0};
    result.value = change_flag (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_52 rboxc_gate_call_52 (struct rboxc_bash_gate *g, char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_52 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_52 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_52 result = {0};
    result.value = shell_is_restricted (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_53 rboxc_gate_call_53 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2, int a3) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_53 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_53 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_53 result = {0};
    result.value = bind_args (a0, a1, a2, a3);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_54 rboxc_gate_call_54 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2, int a3) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_54 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_54 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_54 result = {0};
    result.value = bind_args (a0, a1, a2, a3);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_55 rboxc_gate_call_55 (struct rboxc_bash_gate *g, char ** a0, int a1, int a2, int a3) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_55 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_55 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_55 result = {0};
    result.value = bind_args (a0, a1, a2, a3);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_56 rboxc_gate_call_56 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_56 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_56 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_56 result = {0};
    result.value = strlen (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_57 rboxc_gate_call_57 (struct rboxc_bash_gate *g, unsigned long a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_57 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_57 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_57 result = {0};
    result.value = xmalloc (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_58 rboxc_gate_call_58 (struct rboxc_bash_gate *g, char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_58 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_58 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_58 result = {0};
    result.value = strcpy (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_59 rboxc_gate_call_59 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_59 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_59 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_59 result = {0};
    result.value = strlen (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_60 rboxc_gate_call_60 (struct rboxc_bash_gate *g, unsigned long a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_60 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_60 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_60 result = {0};
    result.value = xmalloc (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_61 rboxc_gate_call_61 (struct rboxc_bash_gate *g, char * a0, const char * a1) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_61 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_61 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_61 result = {0};
    result.value = strcpy (a0, a1);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_62 rboxc_gate_call_62 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_62 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_62 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_62 result = {0};
    run_startup_files ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_63 rboxc_gate_call_63 (struct rboxc_bash_gate *g, void * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_63 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_63 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_63 result = {0};
    free (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_64 rboxc_gate_call_64 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_64 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_64 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_64 result = {0};
    result.value = bind_variable (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_65 rboxc_gate_call_65 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_65 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_65 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_65 result = {0};
    sv_strict_posix (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_66 rboxc_gate_call_66 (struct rboxc_bash_gate *g, char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_66 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_66 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_66 result = {0};
    result.value = maybe_make_restricted (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_67 rboxc_gate_call_67 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_67 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_67 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_67 result = {0};
    cmd_init ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_68 rboxc_gate_call_68 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_68 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_68 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_68 result = {0};
    uwp_init ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_69 rboxc_gate_call_69 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_69 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_69 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_69 result = {0};
    start_debugger ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_70 rboxc_gate_call_70 (struct rboxc_bash_gate *g, char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_70 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_70 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_70 result = {0};
    result.value = run_one_command (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_71 rboxc_gate_call_71 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_71 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_71 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_71 result = {0};
    exit_shell (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_72 rboxc_gate_call_72 (struct rboxc_bash_gate *g, char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_72 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_72 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_72 result = {0};
    result.value = open_shell_script (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_73 rboxc_gate_call_73 (struct rboxc_bash_gate *g, FILE * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_73 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_73 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_73 result = {0};
    result.value = fileno (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_74 rboxc_gate_call_74 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_74 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_74 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_74 result = {0};
    set_bash_input ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_75 rboxc_gate_call_75 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_75 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_75 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_75 result = {0};
    start_debugger ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_76 rboxc_gate_call_76 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_76 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_76 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_76 result = {0};
    reset_mail_timer ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_77 rboxc_gate_call_77 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_77 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_77 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_77 result = {0};
    init_mail_dates ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_78 rboxc_gate_call_78 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_78 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_78 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_78 result = {0};
    bash_initialize_history ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_79 rboxc_gate_call_79 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_79 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_79 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_79 result = {0};
    load_history ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_80 rboxc_gate_call_80 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_80 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_80 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_80 result = {0};
    result.value = get_tty_state ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_81 rboxc_gate_call_81 (struct rboxc_bash_gate *g, const char * a0, const char * a1, int a2) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_81 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_81 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_81 result = {0};
    result.value = dcgettext (a0, a1, a2);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_82 rboxc_gate_call_82 (struct rboxc_bash_gate *g, const char * a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_82 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_82 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_82 result = {0};
    internal_warning (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_83 rboxc_gate_call_83 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_83 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_83 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_83 result = {0};
    result.value = pretty_print_loop ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_84 rboxc_gate_call_84 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_84 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_84 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_84 result = {0};
    exit_shell (a0);
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_85 rboxc_gate_call_85 (struct rboxc_bash_gate *g) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_85 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_85 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_85 result = {0};
    result.value = reader_loop ();
    rboxc_gate_block (g);
    return result;
  }

struct rboxc_gate_result_86 rboxc_gate_call_86 (struct rboxc_bash_gate *g, int a0) {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct rboxc_gate_result_86 result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct rboxc_gate_result_86 result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct rboxc_gate_result_86 result = {0};
    exit_shell (a0);
    rboxc_gate_block (g);
    return result;
  }
