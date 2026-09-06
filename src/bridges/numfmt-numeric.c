/* Reformat numbers like 11505426432 to the more human-readable 11G
   Copyright (C) 2012-2026 Free Software Foundation, Inc.

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

#include <config.h>
#include <float.h>
#include <getopt.h>
#include <stdio.h>
#include <sys/types.h>
#include <langinfo.h>

#include "argmatch.h"
#include "c-ctype.h"
#include "mbswidth.h"
#include "mcel.h"
#include "quote.h"
#include "skipchars.h"
#include "system.h"
#include "xstrtol.h"

#include "set-fields.h"

#if HAVE_FPSETPREC
# include <ieeefp.h>
#endif

#define scale_from rboxc_numfmt_scale_from
#define scale_to rboxc_numfmt_scale_to
#define round_style rboxc_numfmt_round_style
#define inval_style rboxc_numfmt_inval_style
#define suffix rboxc_numfmt_suffix
#define unit_separator rboxc_numfmt_unit_separator
#define from_unit_size rboxc_numfmt_from_unit_size
#define to_unit_size rboxc_numfmt_to_unit_size
#define grouping rboxc_numfmt_grouping
#define padding_buffer rboxc_numfmt_padding_buffer
#define padding_buffer_size rboxc_numfmt_padding_buffer_size
#define padding_width rboxc_numfmt_padding_width
#define zero_padding_width rboxc_numfmt_zero_padding_width
#define user_precision rboxc_numfmt_user_precision
#define format_str rboxc_numfmt_format_str
#define format_str_prefix rboxc_numfmt_format_str_prefix
#define format_str_suffix rboxc_numfmt_format_str_suffix
#define conv_exit_code rboxc_numfmt_conv_exit_code
#define auto_padding rboxc_numfmt_auto_padding
#define delimiter rboxc_numfmt_delimiter
#define line_delim rboxc_numfmt_line_delim
#define header rboxc_numfmt_header
#define debug rboxc_numfmt_debug
#define decimal_point rboxc_numfmt_decimal_point
#define decimal_point_length rboxc_numfmt_decimal_point_length
#define thousands_sep rboxc_numfmt_thousands_sep
#define thousands_sep_length rboxc_numfmt_thousands_sep_length
#define dev_debug rboxc_numfmt_dev_debug

/* The official name of this program (e.g., no 'g' prefix).  */
#define PROGRAM_NAME "numfmt"

#define AUTHORS proper_name ("Assaf Gordon")

/* Exit code when some numbers fail to convert.  */
enum { EXIT_CONVERSION_WARNINGS = 2 };

enum
{
  FROM_OPTION = CHAR_MAX + 1,
  FROM_UNIT_OPTION,
  TO_OPTION,
  TO_UNIT_OPTION,
  ROUND_OPTION,
  SUFFIX_OPTION,
  GROUPING_OPTION,
  PADDING_OPTION,
  FIELD_OPTION,
  DEBUG_OPTION,
  DEV_DEBUG_OPTION,
  HEADER_OPTION,
  FORMAT_OPTION,
  INVALID_OPTION,
  UNIT_SEPARATOR_OPTION
};

enum scale_type
{
  scale_none,                   /* the default: no scaling.  */
  scale_auto,                   /* --from only.  */
  scale_SI,
  scale_IEC,
  scale_IEC_I                   /* 'i' suffix is required.  */
};

static char const *const scale_from_args[] =
{
  "none", "auto", "si", "iec", "iec-i", NULL
};

static enum scale_type const scale_from_types[] =
{
  scale_none, scale_auto, scale_SI, scale_IEC, scale_IEC_I
};

static char const *const scale_to_args[] =
{
  "none", "si", "iec", "iec-i", NULL
};

static enum scale_type const scale_to_types[] =
{
  scale_none, scale_SI, scale_IEC, scale_IEC_I
};


enum round_type
{
  round_ceiling,
  round_floor,
  round_from_zero,
  round_to_zero,
  round_nearest,
};

static char const *const round_args[] =
{
  "up", "down", "from-zero", "towards-zero", "nearest", NULL
};

static enum round_type const round_types[] =
{
  round_ceiling, round_floor, round_from_zero, round_to_zero, round_nearest
};


enum inval_type
{
  inval_abort,
  inval_fail,
  inval_warn,
  inval_ignore
};

static char const *const inval_args[] =
{
  "abort", "fail", "warn", "ignore", NULL
};

static enum inval_type const inval_types[] =
{
  inval_abort, inval_fail, inval_warn, inval_ignore
};

static struct option const longopts[] =
{
  {"from", required_argument, NULL, FROM_OPTION},
  {"from-unit", required_argument, NULL, FROM_UNIT_OPTION},
  {"to", required_argument, NULL, TO_OPTION},
  {"to-unit", required_argument, NULL, TO_UNIT_OPTION},
  {"round", required_argument, NULL, ROUND_OPTION},
  {"padding", required_argument, NULL, PADDING_OPTION},
  {"suffix", required_argument, NULL, SUFFIX_OPTION},
  {"unit-separator", required_argument, NULL, UNIT_SEPARATOR_OPTION},
  {"grouping", no_argument, NULL, GROUPING_OPTION},
  {"delimiter", required_argument, NULL, 'd'},
  {"field", required_argument, NULL, FIELD_OPTION},
  {"debug", no_argument, NULL, DEBUG_OPTION},
  {"-debug", no_argument, NULL, DEV_DEBUG_OPTION},
  {"header", optional_argument, NULL, HEADER_OPTION},
  {"format", required_argument, NULL, FORMAT_OPTION},
  {"invalid", required_argument, NULL, INVALID_OPTION},
  {"zero-terminated", no_argument, NULL, 'z'},
  {GETOPT_HELP_OPTION_DECL},
  {GETOPT_VERSION_OPTION_DECL},
  {NULL, 0, NULL, 0}
};

/* Maximum number of digits we can safely handle
   without precision loss, if scaling is 'none'.  */
enum { MAX_UNSCALED_DIGITS = LDBL_DIG };

/* Maximum number of digits we can work with.
   This is equivalent to 999Q.
   NOTE: 'long double' can handle more than that, but there's
         no official suffix assigned beyond Quetta (1000^10).  */
enum { MAX_ACCEPTABLE_DIGITS = 33 };

extern enum scale_type scale_from;
extern enum scale_type scale_to;
extern enum round_type round_style;
extern enum inval_type inval_style;
extern char const *suffix;
extern char const *unit_separator;
extern uintmax_t from_unit_size;
extern uintmax_t to_unit_size;
extern int grouping;
extern char *padding_buffer;
extern idx_t padding_buffer_size;
extern intmax_t padding_width;
extern int zero_padding_width;
extern long int user_precision;
extern char const *format_str;
extern char *format_str_prefix;
extern char *format_str_suffix;

/* By default, any conversion error will terminate the program.  */
extern int conv_exit_code;


/* auto-pad each line based on skipped whitespace.  */
extern int auto_padding;

/* field delimiter - if NULL, blanks separate fields.  */
extern char const *delimiter;

/* line delimiter.  */
extern unsigned char line_delim;

/* if non-zero, the first 'header' lines from STDIN are skipped.  */
extern uintmax_t header;

/* Debug for users: print warnings to STDERR about possible
   error (similar to sort's debug).  */
extern bool debug;

/* will be set according to the current locale.  */
extern char const *decimal_point;
extern int decimal_point_length;
extern char const *thousands_sep;
extern int thousands_sep_length;

/* debugging for developers.  Enables devmsg().  */
extern bool dev_debug;

static bool
newline_or_blank (mcel_t g)
{
  return g.ch == '\n' || c32issep (g.ch);
}

static inline int
default_scale_base (enum scale_type scale)
{
  switch (scale)
    {
    case scale_IEC:
    case scale_IEC_I:
      return 1024;

    case scale_none:
    case scale_auto:
    case scale_SI:
    default:
      return 1000;
    }
}

static char const zero_and_valid_suffixes[] = "0KkMGTPEZYRQ";
static char const *valid_suffixes = 1 + zero_and_valid_suffixes;

static inline bool
valid_suffix (const char suf)
{
  return strchr (valid_suffixes, suf) != NULL;
}

static inline int
suffix_power (const char suf)
{
  switch (suf)
    {
    case 'k':                  /* kilo.  */
    case 'K':                  /* kilo or kibi.  */
      return 1;

    case 'M':                  /* mega or mebi.  */
      return 2;

    case 'G':                  /* giga or gibi.  */
      return 3;

    case 'T':                  /* tera or tebi.  */
      return 4;

    case 'P':                  /* peta or pebi.  */
      return 5;

    case 'E':                  /* exa or exbi.  */
      return 6;

    case 'Z':                  /* zetta or 2**70.  */
      return 7;

    case 'Y':                  /* yotta or 2**80.  */
      return 8;

    case 'R':                  /* ronna or 2**90.  */
      return 9;

    case 'Q':                  /* quetta or 2**100.  */
      return 10;

    default:                   /* should never happen. assert?  */
      return 0;
    }
}

static inline char const *
suffix_power_char (int power)
{
  switch (power)
    {
    case 0:
      return "";

    case 1:
      return "K";

    case 2:
      return "M";

    case 3:
      return "G";

    case 4:
      return "T";

    case 5:
      return "P";

    case 6:
      return "E";

    case 7:
      return "Z";

    case 8:
      return "Y";

    case 9:
      return "R";

    case 10:
      return "Q";

    default:
      return "(error)";
    }
}

/* Similar to 'powl(3)' but without requiring 'libm'.  */
static long double
powerld (long double base, int x)
{
  long double result = base;
  if (x == 0)
    return 1;                   /* note for test coverage: this is never
                                   reached, as 'powerld' won't be called if
                                   there's no suffix, hence, no "power".  */

  /* TODO: check for overflow, inf?  */
  while (--x)
    result *= base;
  return result;
}

/* Similar to 'fabs(3)' but without requiring 'libm'.  */
static inline long double
absld (long double val)
{
  return val < 0 ? -val : val;
}

/* Scale down 'val', returns 'updated val' and 'x', such that
     val*base^X = original val
     Similar to "frexpl(3)" but without requiring 'libm',
     allowing only integer scale, limited functionality and error checking.  */
static long double
expld (long double val, int base, int /*output */ *x)
{
  int power = 0;

  if (val >= -LDBL_MAX && val <= LDBL_MAX)
    {
      while (absld (val) >= base)
        {
          ++power;
          val /= base;
        }
    }
  if (x)
    *x = power;
  return val;
}

/* EXTREMELY limited 'ceil' - without 'libm'.
   Assumes values that fit in intmax_t.  */
static inline intmax_t
simple_round_ceiling (long double val)
{
  intmax_t intval = val;
  if (intval < val)
    intval++;
  return intval;
}

/* EXTREMELY limited 'floor' - without 'libm'.
   Assumes values that fit in intmax_t.  */
static inline intmax_t
simple_round_floor (long double val)
{
  return -simple_round_ceiling (-val);
}

/* EXTREMELY limited 'round away from zero'.
   Assumes values that fit in intmax_t.  */
static inline intmax_t
simple_round_from_zero (long double val)
{
  return val < 0 ? simple_round_floor (val) : simple_round_ceiling (val);
}

/* EXTREMELY limited 'round away to zero'.
   Assumes values that fit in intmax_t.  */
static inline intmax_t
simple_round_to_zero (long double val)
{
  return val;
}

/* EXTREMELY limited 'round' - without 'libm'.
   Assumes values that fit in intmax_t.  */
static inline intmax_t
simple_round_nearest (long double val)
{
  return val < 0 ? val - 0.5 : val + 0.5;
}

ATTRIBUTE_CONST
static inline long double
simple_round (long double val, enum round_type t)
{
  intmax_t rval;
  intmax_t intmax_mul = val / INTMAX_MAX;
  val -= (long double) INTMAX_MAX * intmax_mul;

  switch (t)
    {
    case round_ceiling:
      rval = simple_round_ceiling (val);
      break;

    case round_floor:
      rval = simple_round_floor (val);
      break;

    case round_from_zero:
      rval = simple_round_from_zero (val);
      break;

    case round_to_zero:
      rval = simple_round_to_zero (val);
      break;

    case round_nearest:
      rval = simple_round_nearest (val);
      break;

    default:
      /* to silence the compiler - this should never happen.  */
      return 0;
    }

  return (long double) INTMAX_MAX * intmax_mul + rval;
}

enum simple_strtod_error
{
  SSE_OK = 0,
  SSE_OK_PRECISION_LOSS,
  SSE_OVERFLOW,
  SSE_INVALID_NUMBER,

  /* the following are returned by 'simple_strtod_human'.  */
  SSE_VALID_BUT_FORBIDDEN_SUFFIX,
  SSE_INVALID_SUFFIX,
  SSE_MISSING_I_SUFFIX
};

/* Read an *integer* INPUT_STR,
   but return the integer value in a 'long double' VALUE
   hence, no UINTMAX_MAX limitation.
   NEGATIVE is updated, and is stored separately from the VALUE
   so that signbit() isn't required to determine the sign of -0..
   ENDPTR is required (unlike strtod) and is used to store a pointer
   to the character after the last character used in the conversion.

   Note locale'd grouping is not supported,
   nor is skipping of white-space supported.

   Returns:
      SSE_OK - valid number.
      SSE_OK_PRECISION_LOSS - if more than 18 digits were used.
      SSE_OVERFLOW          - if more than 33 digits (999Q) were used.
      SSE_INVALID_NUMBER    - if no digits were found.  */
static enum simple_strtod_error
simple_strtod_int (char const *input_str,
                   char **endptr, long double *value, bool *negative)
{
  enum simple_strtod_error e = SSE_OK;

  long double val = 0;
  int digits = 0;
  bool found_digit = false;

  if (*input_str == '-')
    {
      input_str++;
      *negative = true;
    }
  else
    *negative = false;

  *endptr = (char *) input_str;
  while (c_isdigit (**endptr))
    {
      int digit = (**endptr) - '0';

      found_digit = true;

      if (val || digit)
        digits++;

      if (digits > MAX_UNSCALED_DIGITS)
        e = SSE_OK_PRECISION_LOSS;

      if (digits > MAX_ACCEPTABLE_DIGITS)
        return SSE_OVERFLOW;

      val *= 10;
      val += digit;

      ++(*endptr);

      if (thousands_sep_length > 0
          && STREQ_LEN (*endptr, thousands_sep, thousands_sep_length)
          && c_isdigit ((*endptr)[thousands_sep_length]))
        (*endptr) += thousands_sep_length;
    }
  if (! found_digit
      && ! STREQ_LEN (*endptr, decimal_point, decimal_point_length))
    return SSE_INVALID_NUMBER;
  if (*negative)
    val = -val;

  if (value)
    *value = val;

  return e;
}

/* Read a floating-point INPUT_STR represented as "NNNN[.NNNNN]",
   and return the value in a 'long double' VALUE.
   ENDPTR is required (unlike strtod) and is used to store a pointer
   to the character after the last character used in the conversion.
   PRECISION is optional and used to indicate fractions are present.

   Note locale'd grouping is not supported,
   nor is skipping of white-space supported.

   Returns:
      SSE_OK - valid number.
      SSE_OK_PRECISION_LOSS - if more than 18 digits were used.
      SSE_OVERFLOW          - if more than 33 digits (999Q) were used.
      SSE_INVALID_NUMBER    - if no digits were found.  */
static enum simple_strtod_error
simple_strtod_float (char const *input_str,
                     char **endptr,
                     long double *value,
                     size_t *precision)
{
  bool negative;
  enum simple_strtod_error e = SSE_OK;

  if (precision)
    *precision = 0;

  /* TODO: accept locale'd grouped values for the integral part.  */
  e = simple_strtod_int (input_str, endptr, value, &negative);
  if (e != SSE_OK && e != SSE_OK_PRECISION_LOSS)
    return e;

  /* optional decimal point + fraction.  */
  if (STREQ_LEN (*endptr, decimal_point, decimal_point_length))
    {
      char *ptr2;
      long double val_frac = 0;
      bool neg_frac;

      (*endptr) += decimal_point_length;
      enum simple_strtod_error e2 =
        simple_strtod_int (*endptr, &ptr2, &val_frac, &neg_frac);
      if (e2 != SSE_OK && e2 != SSE_OK_PRECISION_LOSS)
        return e2;
      if (e2 == SSE_OK_PRECISION_LOSS)
        e = e2;                       /* propagate warning.  */
      if (neg_frac)
        return SSE_INVALID_NUMBER;

      /* number of digits in the fractions.  */
      size_t exponent = ptr2 - *endptr;

      val_frac = ((long double) val_frac) / powerld (10, exponent);

      /* TODO: detect loss of precision (only really 18 digits
         of precision across all digits (before and after '.')).  */
      if (value)
        {
          if (negative)
            *value -= val_frac;
          else
            *value += val_frac;
        }

      if (precision)
        *precision = exponent;

      *endptr = ptr2;
    }
  return e;
}

/* Read a 'human' INPUT_STR represented as "NNNN[.NNNNN] + suffix",
   and return the value in a 'long double' VALUE,
   with the precision of the input returned in PRECISION.
   ENDPTR is required (unlike strtod) and is used to store a pointer
   to the character after the last character used in the conversion.
   ALLOWED_SCALING determines the scaling supported.

   TODO:
     support locale'd grouping
     accept scientific and hex floats (probably use strtold directly)

   Returns:
      SSE_OK - valid number.
      SSE_OK_PRECISION_LOSS - if more than LDBL_DIG digits were used.
      SSE_OVERFLOW          - if more than 33 digits (999Q) were used.
      SSE_INVALID_NUMBER    - if no digits were found.
      SSE_VALID_BUT_FORBIDDEN_SUFFIX
      SSE_INVALID_SUFFIX
      SSE_MISSING_I_SUFFIX  */
static enum simple_strtod_error
simple_strtod_human (char const *input_str,
                     char **endptr, long double *value, size_t *precision,
                     enum scale_type allowed_scaling)
{
  int power = 0;
  /* 'scale_auto' is checked below.  */
  int scale_base = default_scale_base (allowed_scaling);

  devmsg ("simple_strtod_human:\n  input string: %s\n"
          "  locale decimal-point: %s\n"
          "  MAX_UNSCALED_DIGITS: %d\n",
          quote_n (0, input_str),
          quote_n (1, decimal_point),
          MAX_UNSCALED_DIGITS);

  enum simple_strtod_error e =
    simple_strtod_float (input_str, endptr, value, precision);
  if (e != SSE_OK && e != SSE_OK_PRECISION_LOSS)
    return e;

  devmsg ("  parsed numeric value: %Lf\n"
          "  input precision = %d\n", *value, (int)*precision);

  while (**endptr)
    {
      /* process suffix.  */

      /* Skip a single blank, NBSP or specified unit separator.
         Note an explicit empty --unit-sep should disable blank matching. */
      bool matched_unit_sep = false;
      if (unit_separator)
        {
          size_t sep_len = strlen (unit_separator);
          if (STREQ_LEN (*endptr, unit_separator, sep_len))
            {
              matched_unit_sep = true;
              (*endptr) += sep_len;
            }
        }
      if (!matched_unit_sep)
        {
          mcel_t g = mcel_scanz (*endptr);
          if (c32issep (g.ch) || c32isnbspace (g.ch))
            (*endptr) += g.len;
        }

      if (**endptr == '\0')
        break;  /* Treat as no suffix.  */

      if (!valid_suffix (**endptr))
        {
          /* Trailing blanks are allowed.  */
          *endptr = skip_str_matching (*endptr, newline_or_blank, true);
          if (**endptr == '\0')
            break;

          return SSE_INVALID_SUFFIX;
        }

      if (allowed_scaling == scale_none)
        return SSE_VALID_BUT_FORBIDDEN_SUFFIX;

      power = suffix_power (**endptr);
      (*endptr)++;                     /* skip first suffix character.  */

      if (allowed_scaling == scale_auto && **endptr == 'i')
        {
          /* auto-scaling enabled, and the first suffix character
             is followed by an 'i' (e.g. Ki, Mi, Gi).  */
          scale_base = 1024;
          (*endptr)++;              /* skip 'i' in suffix.  */
          devmsg ("  Auto-scaling, found 'i', switching to base %d\n",
                  scale_base);
        }
      else if (allowed_scaling == scale_IEC_I)
        {
          if (**endptr == 'i')
            (*endptr)++;
          else
            return SSE_MISSING_I_SUFFIX;
        }

      *precision = 0;  /* Reset, to select precision based on scale.  */

      /* Trailing blanks are allowed.  */
      *endptr = skip_str_matching (*endptr, newline_or_blank, true);

      break;
    }

  long double multiplier = powerld (scale_base, power);

  devmsg ("  suffix power=%d^%d = %Lf\n", scale_base, power, multiplier);

  /* TODO: detect loss of precision and overflows.  */
  (*value) = (*value) * multiplier;

  devmsg ("  returning value: %Lf (%LG)\n", *value, *value);

  return e;
}


static void
simple_strtod_fatal (enum simple_strtod_error err, char const *input_str)
{
  char const *msgid = NULL;

  switch (err)
    {
    case SSE_OK_PRECISION_LOSS:
    case SSE_OK:
      /* should never happen - this function isn't called when OK.  */
      unreachable ();

    case SSE_OVERFLOW:
      msgid = N_("value too large to be converted: %s");
      break;

    case SSE_INVALID_NUMBER:
      msgid = N_("invalid number: %s");
      break;

    case SSE_VALID_BUT_FORBIDDEN_SUFFIX:
      msgid = N_("rejecting suffix in input: %s (consider using --from)");
      break;

    case SSE_INVALID_SUFFIX:
      msgid = N_("invalid suffix in input: %s");
      break;

    case SSE_MISSING_I_SUFFIX:
      msgid = N_("missing 'i' suffix in input: %s (e.g Ki/Mi/Gi)");
      break;

    }

  if (inval_style != inval_ignore)
    error (conv_exit_code, 0, gettext (msgid), quote (input_str));
}

/* Convert VAL to a human format string using PRECISION in BUF of size
   BUF_SIZE.  Use SCALE, GROUP, and ROUND to format.  Return
   the number of bytes needed to represent VAL.  If this number is not
   less than BUF_SIZE, the buffer is too small; if it is negative, the
   formatting failed for some reason.  */
static int
double_to_human (long double val, int precision,
                 char *buf, idx_t buf_size,
                 enum scale_type scale, int group, enum round_type round)
{
  char fmt[sizeof "%'0.*Lfi%s%s%s%s" + INT_STRLEN_BOUND (zero_padding_width)];
  char *pfmt = fmt;
  *pfmt++ = '%';

  if (group)
    *pfmt++ = '\'';

  if (zero_padding_width)
    pfmt += sprintf (pfmt, "0%d", zero_padding_width);

  devmsg ("double_to_human:\n");

  if (scale == scale_none)
    {
      val *= powerld (10, precision);
      val = simple_round (val, round);
      val /= powerld (10, precision);

      devmsg ((group) ?
              "  no scaling, returning (grouped) value: %'.*Lf\n" :
              "  no scaling, returning value: %.*Lf\n", precision, val);

      strcpy (pfmt, ".*Lf%s");

      return snprintf (buf, buf_size, fmt, precision, val,
                       suffix ? suffix : "");
    }

  /* Scaling requested by user. */
  double scale_base = default_scale_base (scale);

  /* Normalize val to scale. */
  int power = 0;
  val = expld (val, scale_base, &power);
  devmsg ("  scaled value to %Lf * %0.f ^ %d\n", val, scale_base, power);

  /* Perform rounding. */
  int power_adjust = 0;
  if (user_precision != -1)
    power_adjust = MIN (power * 3, user_precision);
  else if (absld (val) < 10)
    {
      /* for values less than 10, we allow one decimal-point digit,
         so adjust before rounding. */
      power_adjust = 1;
    }

  val *= powerld (10, power_adjust);
  val = simple_round (val, round);
  val /= powerld (10, power_adjust);

  /* two special cases after rounding:
     1. a "999.99" can turn into 1000 - so scale down
     2. a "9.99" can turn into 10 - so don't display decimal-point.  */
  if (absld (val) >= scale_base)
    {
      val /= scale_base;
      power++;
    }

  /* should "7.0" be printed as "7" ?
     if removing the ".0" is preferred, enable the fourth condition.  */
  int show_decimal_point = (val != 0) && (absld (val) < 10) && (power > 0);
  /* && (absld (val) > simple_round_floor (val))) */

  devmsg ("  after rounding, value=%Lf * %0.f ^ %d\n", val, scale_base, power);

  strcpy (pfmt, ".*Lf%s%s%s%s");

  int prec = user_precision == -1 ? show_decimal_point : user_precision;

  return snprintf (buf, buf_size, fmt, prec, val,
                   (power > 0 && unit_separator) ? unit_separator : "",
                   power == 1 && scale == scale_SI
                   ? "k" : suffix_power_char (power),
                   &"i"[! (scale == scale_IEC_I && 0 < power)],
                   suffix ? suffix : "");
}

/* Convert a string of decimal digits, N_STRING, with an optional suffix
   to an integral value.  Suffixes are handled as with --from=auto.
   Upon successful conversion, return that value.
   If it cannot be converted, give a diagnostic and exit.  */
static uintmax_t
unit_to_umax (char const *n_string)
{
  strtol_error s_err;
  char const *c_string = n_string;
  char *t_string = NULL;
  size_t n_len = strlen (n_string);
  char *end = NULL;
  uintmax_t n;
  char const *suffixes = valid_suffixes;

  /* Adjust suffixes so K=1000, Ki=1024, KiB=invalid.  */
  if (n_len && ! c_isdigit (n_string[n_len - 1]))
    {
      t_string = xmalloc (n_len + 2);
      end = t_string + n_len - 1;
      memcpy (t_string, n_string, n_len);

      if (*end == 'i' && 2 <= n_len && ! c_isdigit (*(end - 1)))
        *end = '\0';
      else
        {
          *++end = 'B';
          *++end = '\0';
          suffixes = zero_and_valid_suffixes;
        }

      c_string = t_string;
    }

  s_err = xstrtoumax (c_string, &end, 10, &n, suffixes);

  if (s_err != LONGINT_OK || *end || n == 0)
    {
      free (t_string);
      error (EXIT_FAILURE, 0, _("invalid unit size: %s"), quote (n_string));
    }

  free (t_string);

  return n;
}

static enum simple_strtod_error
parse_human_number (char const *str, long double /*output */ *value,
                    size_t *precision)
{
  char *ptr = NULL;

  enum simple_strtod_error e =
    simple_strtod_human (str, &ptr, value, precision, scale_from);
  if (e != SSE_OK && e != SSE_OK_PRECISION_LOSS)
    {
      simple_strtod_fatal (e, str);
      return e;
    }

  if (ptr && *ptr != '\0')
    {
      if (inval_style != inval_ignore)
        error (conv_exit_code, 0, _("invalid suffix in input %s: %s"),
               quote_n (0, str), quote_n (1, ptr));
      e = SSE_INVALID_SUFFIX;
    }
  return e;
}
static bool
prepare_padded_number (const long double val, size_t precision,
                       intmax_t *padding)
{
  /* Generate Output. */
  size_t precision_used = user_precision == -1 ? precision : user_precision;

  /* Can't reliably print too-large values without auto-scaling. */
  int x;
  expld (val, 10, &x);

  if (scale_to == scale_none
      && x + precision_used > MAX_UNSCALED_DIGITS)
    {
      if (inval_style != inval_ignore)
        {
          if (precision_used)
            error (conv_exit_code, 0,
                   _("value/precision too large to be printed: '%Lg/%zu'"
                     " (consider using --to)"), val, precision_used);
          else
            error (conv_exit_code, 0,
                   _("value too large to be printed: '%Lg'"
                     " (consider using --to)"), val);
        }
      return false;
    }

  if (x > MAX_ACCEPTABLE_DIGITS - 1)
    {
      if (inval_style != inval_ignore)
        error (conv_exit_code, 0, _("value too large to be printed: '%Lg'"
                                    " (cannot handle values > 999Q)"), val);
      return false;
    }

  while (true)
    {
      int numlen = double_to_human (val, precision_used,
                                    padding_buffer, padding_buffer_size,
                                    scale_to, grouping, round_style);
      ptrdiff_t growth;
      if (numlen < 0 || ckd_sub (&growth, numlen, padding_buffer_size - 1))
        error (EXIT_FAILURE, 0,
               _("failed to prepare value '%Lf' for printing"), val);
      if (growth <= 0)
        break;
      padding_buffer = xpalloc (padding_buffer, &padding_buffer_size,
                                growth, -1, 1);
    }

  devmsg ("formatting output:\n  value: %Lf\n  humanized: %s\n",
          val, quote (padding_buffer));

  intmax_t pad = 0;
  if (padding_width)
    {
      int buf_width = mbswidth (padding_buffer,
                                MBSW_REJECT_INVALID | MBSW_REJECT_UNPRINTABLE);
      if (0 <= buf_width)
        {
          if (padding_width < 0)
            {
              if (padding_width < -buf_width)
                pad = padding_width + buf_width;
            }
          else
            {
              if (buf_width < padding_width)
                pad = padding_width - buf_width;
            }
        }
    }

  *padding = pad;
  return true;
}
static bool
process_suffixed_number (char *text, long double *result,
                         size_t *precision, long int field)
{
  char saved_suffix = '\0';

  if (suffix)
    {
      if (mbs_endswith (text, suffix))
        {
          saved_suffix = *(text + strlen (text) - strlen (suffix));
          *(text + strlen (text) - strlen (suffix)) = '\0';
          devmsg ("trimming suffix %s\n", quote (suffix));
        }
      else
        devmsg ("no valid suffix found\n");
    }

  /* Skip blanks - always.  */
  char *p = skip_str_matching (text, newline_or_blank, true);

  /* setup auto-padding.  */
  if (auto_padding)
    {
      padding_width = text < p || 1 < field
                      ? mbswidth (text,
                                  MBSW_REJECT_INVALID | MBSW_REJECT_UNPRINTABLE)
                      : 0;
      if (padding_width < 0)
        padding_width = strlen (text);
      devmsg ("setting Auto-Padding to %jd characters\n", padding_width);
    }

  long double val = 0;
  enum simple_strtod_error e = parse_human_number (p, &val, precision);
  if (e == SSE_OK_PRECISION_LOSS && debug)
    error (0, 0, _("large input value %s: possible precision loss"),
           quote (p));

  if (from_unit_size != 1 || to_unit_size != 1)
    val = (val * from_unit_size) / to_unit_size;

  *result = val;

  if (e == SSE_OK || e == SSE_OK_PRECISION_LOSS)
    return true;
  else
    {
      if (saved_suffix)
        *(text + strlen (text)) = saved_suffix;
      return false;
    }
}
_Static_assert(sizeof(long double) == 16 && _Alignof(long double) == 16,
               "pinned x86-64 GNU long double storage");
bool rboxc_numfmt_process_suffixed(char *text, void *value, size_t *precision, long field)
{ return process_suffixed_number(text, value, precision, field); }
bool rboxc_numfmt_prepare(void const *value, size_t precision, intmax_t *padding)
{ return prepare_padded_number(*(long double const *)value, precision, padding); }
