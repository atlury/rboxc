/* seq - print sequence of numbers to standard output.
   Copyright (C) 1994-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Ulrich Drepper.  */

#define usage _usage_seq
#include <config.h>
#include <getopt.h>
#include <stdio.h>
#include <sys/types.h>

#include "system.h"
#include "c-ctype.h"
#include "cl-strtod.h"
#include "full-write.h"
#include "quote.h"
#include "xstrtod.h"
#include "xvasprintf.h"

/* Roll our own isfinite/isnan rather than using <math.h>, so that we don't
   have to worry about linking -lm just for isfinite.  */
#ifndef isfinite
# define isfinite(x) ((x) * 0 == 0)
#endif
#ifndef isnan
# define isnan(x) ((x) != (x))
#endif

/* Limit below which seq_fast has more throughput.
   Determined with: seq 0 200 inf | pv > /dev/null  */
#define SEQ_FAST_STEP_LIMIT 200  /* Keep in sync with texinfo description.  */
#define SEQ_FAST_STEP_LIMIT_DIGITS 3

/* The official name of this program (e.g., no 'g' prefix).  */
#define PROGRAM_NAME "seq"

#define AUTHORS proper_name ("Ulrich Drepper")

#define locale_ok rboxc_seq_locale_ok
#define equal_width rboxc_seq_equal_width
#define separator rboxc_seq_separator
#define terminator rboxc_seq_terminator

/* True if the locale settings were honored.  */
extern bool locale_ok;

/* If true print all number with equal width.  */
extern bool equal_width;

/* The string used to separate two numbers.  */
extern char const *separator;

/* The string output after all numbers have been output.
   Usually "\n" or "\0".  */
extern char const terminator[];

static struct option const long_options[] =
{
  { "equal-width", no_argument, NULL, 'w'},
  { "format", required_argument, NULL, 'f'},
  { "separator", required_argument, NULL, 's'},
  {GETOPT_HELP_OPTION_DECL},
  {GETOPT_VERSION_OPTION_DECL},
  { NULL, 0, NULL, 0}
};

struct operand
{
  /* Its value, converted to 'long double'.  */
  long double value;

  /* Its print width, if it were printed out in a form similar to its
     input form.  An input like "-.1" is treated like "-0.1", and an
     input like "1." is treated like "1", but otherwise widths are
     left alone.  */
  size_t width;

  /* Number of digits after the decimal point, or INT_MAX if the
     number can't easily be expressed as a fixed-point number.  */
  int precision;
};
typedef struct operand operand;

/* Description of what a number-generating format will generate.  */
struct layout
{
  /* Number of bytes before and after the number.  */
  size_t prefix_len;
  size_t suffix_len;
};


static operand
scan_arg (char const *arg)
{
  operand ret;

  if (! xstrtold (arg, NULL, &ret.value, cl_strtold))
    {
      error (0, 0, _("invalid floating point argument: %s"), quote (arg));
      usage (EXIT_FAILURE);
    }

  if (isnan (ret.value))
    {
      error (0, 0, _("invalid %s argument: %s"), quote_n (0, "not-a-number"),
             quote_n (1, arg));
      usage (EXIT_FAILURE);
    }

  /* We don't output spaces or '+' so don't include in width */
  while (isspace (to_uchar (*arg)) || *arg == '+')
    arg++;

  /* Default to auto width and precision.  */
  ret.width = 0;
  ret.precision = INT_MAX;

  /* Use no precision (and possibly fast generation) for integers.  */
  char const *decimal_point = strchr (arg, '.');
  if (! decimal_point && ! strchr (arg, 'p') /* not a hex float */)
    ret.precision = 0;

  /* auto set width and precision for decimal inputs.  */
  if (! arg[strcspn (arg, "xX")] && isfinite (ret.value))
    {
      size_t fraction_len = 0;
      ret.width = strlen (arg);

      if (decimal_point)
        {
          fraction_len = strcspn (decimal_point + 1, "eE");
          if (fraction_len <= INT_MAX)
            ret.precision = fraction_len;
          ret.width += (fraction_len == 0                       /* #.  -> #   */
                        ? -1
                        : (decimal_point == arg                 /* .#  -> 0.# */
                           || !c_isdigit (decimal_point[-1]))); /* -.# -> 0.# */
        }
      char const *e = strchr (arg, 'e');
      if (! e)
        e = strchr (arg, 'E');
      if (e)
        {
          long exponent = MAX (strtol (e + 1, NULL, 10), -LONG_MAX);
          ret.precision += exponent < 0 ? -exponent
                                        : - MIN (ret.precision, exponent);
          /* Don't account for e.... in the width since this is not output.  */
          ret.width -= strlen (arg) - (e - arg);
          /* Adjust the width as per the exponent.  */
          if (exponent < 0)
            {
              if (decimal_point)
                {
                  if (e == decimal_point + 1) /* undo #. -> # above  */
                    ret.width++;
                }
              else
                ret.width++;
              exponent = -exponent;
            }
          else
            {
              if (decimal_point && ret.precision == 0 && fraction_len)
                ret.width--; /* discount space for '.'  */
              exponent -= MIN (fraction_len, exponent);
            }
          ret.width += exponent;
        }
    }

  return ret;
}
static void
print_numbers (char const *fmt, struct layout layout,
               long double first, long double step, long double last)
{
  bool out_of_range = (step < 0 ? first < last : last < first);

  if (! out_of_range)
    {
      long double x = first;

      for (long double i = 1; ; i++)
        {
          long double x0 = x;
          if (printf (fmt, x) < 0)
            write_error ();
          if (out_of_range)
            break;

          /* Mathematically equivalent to 'x += step;', and typically
             less subject to rounding error.  */
          x = first + i * step;

          out_of_range = (step < 0 ? x < last : last < x);

          if (out_of_range)
            {
              /* If the number just past LAST prints as a value equal
                 to LAST, and prints differently from the previous
                 number, then print the number.  This avoids problems
                 with rounding.  For example, with the x86 it causes
                 "seq 0 0.000001 0.000003" to print 0.000003 instead
                 of stopping at 0.000002.  */

              bool print_extra_number = false;
              if (locale_ok)
                setlocale (LC_NUMERIC, "C");
              char *x_str;
              int x_strlen = asprintf (&x_str, fmt, x);
              if (locale_ok)
                setlocale (LC_NUMERIC, "");
              if (x_strlen < 0)
                xalloc_die ();
              x_str[x_strlen - layout.suffix_len] = '\0';

              long double x_val;
              if (xstrtold (x_str + layout.prefix_len, NULL,
                            &x_val, cl_strtold)
                  && x_val == last)
                {
                  char *x0_str = NULL;
                  int x0_strlen = asprintf (&x0_str, fmt, x0);
                  if (x0_strlen < 0)
                    xalloc_die ();
                  x0_str[x0_strlen - layout.suffix_len] = '\0';
                  print_extra_number = !streq (x0_str, x_str);
                  free (x0_str);
                }

              free (x_str);
              if (! print_extra_number)
                break;
            }

          if (fputs (separator, stdout) == EOF)
            write_error ();
        }

      if (fputs (terminator, stdout) == EOF)
        write_error ();
    }
}
static char const *
get_default_format (operand first, operand step, operand last)
{
  static char format_buf[sizeof "%0.Lf" + 2 * INT_STRLEN_BOUND (int)];

  int prec = MAX (first.precision, step.precision);

  if (prec != INT_MAX && last.precision != INT_MAX)
    {
      if (equal_width)
        {
          /* increase first_width by any increased precision in step */
          size_t first_width = first.width + (prec - first.precision);
          /* adjust last_width to use precision from first/step */
          size_t last_width = last.width + (prec - last.precision);
          if (last.precision && prec == 0)
            last_width--;  /* don't include space for '.' */
          if (last.precision == 0 && prec)
            last_width++;  /* include space for '.' */
          if (first.precision == 0 && prec)
            first_width++;  /* include space for '.' */
          size_t width = MAX (first_width, last_width);
          if (width <= INT_MAX)
            {
              int w = width;
              sprintf (format_buf, "%%0%d.%dLf", w, prec);
              return format_buf;
            }
        }
      else
        {
          sprintf (format_buf, "%%.%dLf", prec);
          return format_buf;
        }
    }

  return "%Lg";
}
_Static_assert(sizeof(long double) == 16 && _Alignof(long double) == 16,
               "pinned x86-64 GNU long double storage");
_Static_assert(sizeof(operand) == 32 && offsetof(operand, width) == 16
               && offsetof(operand, precision) == 24, "operand layout");
void rboxc_seq_one(operand *out)
{ *out = (operand){1, 1, 0}; }
void rboxc_seq_scan(char const *arg, operand *out)
{ *out = scan_arg(arg); }
bool rboxc_seq_parse_step(char const *arg, void *value)
{ return xstrtold(arg, NULL, value, cl_strtold); }
bool rboxc_seq_fast_step(void const *value)
{ long double v = *(long double const *)value; return 0 < v && v <= SEQ_FAST_STEP_LIMIT; }
bool rboxc_seq_zero(void const *value)
{ return *(long double const *)value == 0; }
bool rboxc_seq_nonnegative(void const *value)
{ return 0 <= *(long double const *)value; }
bool rboxc_seq_finite(void const *value)
{ return isfinite(*(long double const *)value); }
uintmax_t rboxc_seq_uintmax(void const *value)
{ return *(long double const *)value; }
char *rboxc_seq_integer_string(void const *value)
{ return xasprintf("%0.Lf", *(long double const *)value); }
char const *rboxc_seq_default(operand const *first, operand const *step, operand const *last)
{ return get_default_format(*first, *step, *last); }
void rboxc_seq_print(char const *fmt, struct layout const *layout,
                     void const *first, void const *step, void const *last)
{ print_numbers(fmt, *layout, *(long double const *)first,
                *(long double const *)step, *(long double const *)last); }
