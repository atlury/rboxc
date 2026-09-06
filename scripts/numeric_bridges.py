"""Keep native floating ABI inside small GNU C helpers during translation."""
import hashlib
import re


def braced_end(text, opening):
    depth, i, mode = 1, opening + 1, None
    while depth:
        c = text[i]
        pair = text[i:i+2]
        if mode in ('"', "'"):
            if c == '\\':
                i += 2
                continue
            if c == mode:
                mode = None
        elif mode == '/*':
            if pair == '*/':
                mode = None
                i += 2
                continue
        elif mode == '//':
            if c == '\n':
                mode = None
        elif pair in ('/*', '//'):
            mode = pair
            i += 2
            continue
        elif c in ('"', "'"):
            mode = c
        elif c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
        i += 1
    return i


def function(text, name):
    match = re.search(r'^(?:ATTRIBUTE_\w+\n)*static [^\n]+\n'+re.escape(name)+r' \(', text, re.M)
    assert match, name
    opening = text.index('{', match.end())
    return text[match.start():braced_end(text, opening)]


def prepare(root, name, source, stage):
    original = source.read_text()
    text = original
    notice = re.match(r'\s*(/\*.*?\*/)', original, re.S)[1]
    includes = '\n#include <config.h>\n#include "system.h"\n'
    if name == 'sort':
        nan = function(text, 'nan_compare')
        compare = function(text, 'general_numcompare')
        bridge = notice+includes+'#include <math.h>\n'+nan+'\n'+compare.replace(
            'static int\ngeneral_numcompare', 'int\nrboxc_general_numcompare')
        bridge += '\nvoid rboxc_strtold_end(char const *s, char **end) { (void) strtold(s, end); }\n'
        text = text.replace(nan, '')
        text = text.replace(compare, 'extern int rboxc_general_numcompare(char const *, char const *);\n'
                            'extern void rboxc_strtold_end(char const *, char **);')
        text = text.replace('general_numcompare (', 'rboxc_general_numcompare (')
        text = text.replace('ignore_value (strtold (beg, &tighter_lim));', 'rboxc_strtold_end (beg, &tighter_lim);')
        end = '  main_exit (EXIT_SUCCESS);\n}'
        assert text.count(end) == 1
        text = text.replace(end, '  if (!files_from)\n    free (files);\n\n'+end)
        helpers = ['rboxc_general_numcompare', 'rboxc_strtold_end']
    elif name == 'printf':
        macro = original[original.index('#define STRTOX('):original.index('STRTOX (intmax_t,')]
        macro = macro.replace('FUNC_NAME (char const *s)',
                              'FUNC_NAME (char const *s, bool posixly_correct, void (*verify_numeric)(char const *, char const *))')
        message = re.search(r'static char const \*const cfcc_msg =.*?;', original, re.S)[0]
        opening = text.index('{', text.index("    case 'G':"))
        end = braced_end(text, opening)
        block = text[opening:end]
        assert block.count('vstrtold (argument)') == 1
        bridge = notice+includes+'#include <wchar.h>\n#include "cl-strtod.h"\n#include "xprintf.h"\n'
        bridge += message+'\n'+macro+'\nSTRTOX (long double, vstrtold, cl_strtold (s, &end))\n'
        signature = ('void rboxc_printf_float(char const *p, char const *argument, '
                     'bool have_field_width, int field_width, bool have_precision, int precision, '
                     'bool posixly_correct, void (*verify_numeric)(char const *, char const *))')
        bridge += signature+'\n'+block.replace('vstrtold (argument)', 'vstrtold (argument, posixly_correct, verify_numeric)')+'\n'
        text = text[:opening] + '{ rboxc_printf_float(p, argument, have_field_width, field_width, have_precision, precision, posixly_correct, verify_numeric); }' + text[end:]
        invocation = 'STRTOX (long double, vstrtold,   cl_strtold (s, &end))'
        assert text.count(invocation) == 1
        text = text.replace(invocation, 'extern '+signature+';')
        helpers = ['rboxc_printf_float']
    elif name == 'od':
        types = original[original.index('#if FLOAT16_SUPPORTED'):original.index('\nenum size_spec')]
        macro_start = original.index('#define PRINT_FIELDS(')
        macro_end = original.index('PRINT_TYPE (print_s_char,')
        macros = original[macro_start:macro_end].replace('static void', 'void')
        macros = macros.replace('int width, idx_t pad)', 'int width, idx_t pad, bool input_swap)')
        floating = re.findall(r'^PRINT_FLOATTYPE \(print_.*$', original, re.M)
        assert len(floating) == 5
        bridge = notice+includes+'#include "ftoastr.h"\n#include "xprintf.h"\n'
        bridge += types+'\n'+function(original, 'pad_at')+'\n'+macros
        helpers = []
        for invocation in floating:
            method = re.search(r'\((\w+),', invocation)[1]
            helper_name = 'rboxc_'+method
            helpers.append(helper_name)
            bridge += invocation.replace(method, helper_name, 1)+'\n'
            signature = '(idx_t fields, idx_t blank, void const *block, char const *fmt, int width, idx_t pad'
            replacement = ('extern void '+helper_name+signature+', bool input_swap);\n'
                           'static void '+method+signature+') {\n'
                           '  '+helper_name+'(fields, blank, block, fmt, width, pad, input_swap);\n}\n')
            text = text.replace(invocation, replacement)
        # These types now appear only in sizeof tables. Data decoding stays
        # in the C helpers above, with the actual native floating types.
        text = text.replace('typedef _Float16 float16;', 'typedef struct { unsigned char bytes[2]; } float16;')
        text = text.replace('typedef __bf16 bfloat16;', 'typedef struct { unsigned char bytes[2]; } bfloat16;')
        text = text.replace('sizeof (long double)', '16 /* pinned x86-64 GNU long double size */')
    elif name == 'numfmt':
        # Keep CLI/field processing in Rust. The numeric worker uses the same
        # Rust-owned option state, with prefixed symbols avoiding other applets.
        state = original[original.index('static enum scale_type scale_from ='):
                         original.index('static bool\nnewline_or_blank')]
        declarations = re.findall(r'^static [^\n]+;', state, re.M)
        assert len(declarations) == 28
        aliases = ''
        prefix = original[:original.index('\nvoid\nusage (')]
        for declaration in declarations:
            declarator = declaration.removeprefix('static ').split(' =')[0].rstrip(';')
            symbol = re.search(r'(\w+)$', declarator)[1]
            aliases += '#define '+symbol+' rboxc_numfmt_'+symbol+'\n'
            text = text.replace(declaration, declaration.removeprefix('static '), 1)
            prefix = prefix.replace(declaration, 'extern '+declarator+';', 1)
        marker = '/* The official name of this program'
        text = text.replace(marker, aliases+'\n'+marker, 1)
        prefix = prefix.replace(marker, aliases+'\n'+marker, 1)
        worker_names = ['parse_human_number', 'prepare_padded_number', 'process_suffixed_number']
        bridge = prefix+'\n'+'\n'.join(function(original, n) for n in worker_names)
        bridge += '''
_Static_assert(sizeof(long double) == 16 && _Alignof(long double) == 16,
               "pinned x86-64 GNU long double storage");
bool rboxc_numfmt_process_suffixed(char *text, void *value, size_t *precision, long field)
{ return process_suffixed_number(text, value, precision, field); }
bool rboxc_numfmt_prepare(void const *value, size_t precision, intmax_t *padding)
{ return prepare_padded_number(*(long double const *)value, precision, padding); }
'''
        floating_names = ['powerld', 'absld', 'expld', 'simple_round_ceiling',
                          'simple_round_floor', 'simple_round_from_zero', 'simple_round_to_zero',
                          'simple_round_nearest', 'simple_round', 'simple_strtod_int',
                          'simple_strtod_float', 'simple_strtod_human', 'simple_strtod_fatal',
                          'double_to_human', *worker_names]
        for method in floating_names:
            text = text.replace(function(original, method), '')
        field = function(text, 'process_field')
        replacement = field.replace('long double val = 0;',
                                     'struct { _Alignas(16) unsigned char bytes[16]; } val = {0};')
        replacement = replacement.replace('process_suffixed_number (', 'rboxc_numfmt_process_suffixed (')
        replacement = replacement.replace('prepare_padded_number (val,', 'rboxc_numfmt_prepare (&val,')
        prototypes = ('extern bool rboxc_numfmt_process_suffixed(char *, void *, size_t *, long);\n'
                      'extern bool rboxc_numfmt_prepare(void const *, size_t, intmax_t *);\n')
        text = text.replace(field, prototypes+replacement)
        stdin_done = ('      if (ferror (stdin))\n'
                      '        error (EXIT_FAILURE, errno, _("error reading input"));\n')
        assert text.count(stdin_done) == 1
        text = text.replace(stdin_done, stdin_done+'      free (line);\n')
        helpers = ['rboxc_numfmt_process_suffixed', 'rboxc_numfmt_prepare']
    elif name == 'seq':
        prefix = original[:original.index('\nvoid\nusage (')]
        aliases = ''
        for symbol in ['locale_ok', 'equal_width', 'separator', 'terminator']:
            declaration = re.search(r'^static [^\n]*\b'+symbol+r'\b[^\n]*;', original, re.M)[0]
            declarator = declaration.removeprefix('static ').split(' =')[0].rstrip(';')
            aliases += '#define '+symbol+' rboxc_seq_'+symbol+'\n'
            text = text.replace(declaration, declaration.removeprefix('static '), 1)
            prefix = prefix.replace(declaration, 'extern '+declarator+';', 1)
        marker = '/* True if the locale settings were honored.'
        text = text.replace(marker, aliases+'\n'+marker, 1)
        prefix = prefix.replace(marker, aliases+'\n'+marker, 1)
        # The worker calls the translated command's existing usage function.
        prefix = prefix.replace('#include <config.h>', '#define usage _usage_seq\n#include <config.h>', 1)
        types = original[original.index('struct operand\n'):original.index('/* Read a long double value')]
        workers = ['scan_arg', 'print_numbers', 'get_default_format']
        bridge = prefix+'\n'+types+'\n'+'\n'.join(function(original, n) for n in workers)
        wrappers = '''
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
'''
        bridge += wrappers
        prototypes = '''
extern void rboxc_seq_one(operand *);
extern void rboxc_seq_scan(char const *, operand *);
extern bool rboxc_seq_parse_step(char const *, void *);
extern bool rboxc_seq_fast_step(void const *);
extern bool rboxc_seq_zero(void const *);
extern bool rboxc_seq_nonnegative(void const *);
extern bool rboxc_seq_finite(void const *);
extern uintmax_t rboxc_seq_uintmax(void const *);
extern char *rboxc_seq_integer_string(void const *);
extern char const *rboxc_seq_default(operand const *, operand const *, operand const *);
extern void rboxc_seq_print(char const *, struct layout const *, void const *, void const *, void const *);
'''
        for method in workers:
            text = text.replace(function(original, method), prototypes if method == 'scan_arg' else '')
        changes = {
            'long double value;': 'struct { _Alignas(16) unsigned char bytes[16]; } value;',
            'operand step = { 1, 1, 0 };': 'operand step; rboxc_seq_one(&step);',
            'operand first = { 1, 1, 0 };': 'operand first; rboxc_seq_one(&first);',
            'operand last = scan_arg (argv[optind++]);': 'operand last; rboxc_seq_scan(argv[optind++], &last);',
            'last = scan_arg (argv[optind++]);': 'rboxc_seq_scan(argv[optind++], &last);',
            'xstrtold (argv[optind + 1], NULL, &step.value, cl_strtold)': 'rboxc_seq_parse_step(argv[optind + 1], &step.value)',
            '0 < step.value && step.value <= SEQ_FAST_STEP_LIMIT': 'rboxc_seq_fast_step(&step.value)',
            'seq_fast (s1, s2, step.value)': 'seq_fast (s1, s2, rboxc_seq_uintmax(&step.value))',
            'step.value == 0': 'rboxc_seq_zero(&step.value)',
            'isfinite (first.value)': 'rboxc_seq_finite(&first.value)',
            'isfinite (last.value)': 'rboxc_seq_finite(&last.value)',
            '0 <= first.value': 'rboxc_seq_nonnegative(&first.value)',
            '0 <= last.value': 'rboxc_seq_nonnegative(&last.value)',
            'xasprintf ("%0.Lf", first.value)': 'rboxc_seq_integer_string(&first.value)',
            'xasprintf ("%0.Lf", last.value)': 'rboxc_seq_integer_string(&last.value)',
            'get_default_format (first, step, last)': 'rboxc_seq_default(&first, &step, &last)',
            'print_numbers (format_str, layout, first.value, step.value, last.value)':
                'rboxc_seq_print(format_str, &layout, &first.value, &step.value, &last.value)',
        }
        for before, after in changes.items():
            assert before in text, before
            text = text.replace(before, after)
        format_setup = ('  if (format_str)\n'
                        '    format_str = long_double_format (format_str, &layout);')
        assert text.count(format_setup) == 1
        text = text.replace(format_setup,
                            '  char *owned_format = NULL;\n  if (format_str) {\n'
                            '    format_str = long_double_format (format_str, &layout);\n'
                            '    owned_format = (char *)format_str;\n  }')
        done = '  main_exit (EXIT_SUCCESS);\n}'
        assert text.count(done) == 1
        text = text.replace(done, '  free (owned_format);\n'+done)
        helpers = re.findall(r'\b(rboxc_seq_\w+)\(', prototypes)
    else:
        return source, None
    adapted = stage/(name+'.c')
    adapted.write_text(text)
    helper = root/'src/bridges'/(name+'-numeric.c')
    helper.write_text(bridge)
    return adapted, {'helpers': helpers, 'bridge_file': str(helper.relative_to(root)),
                     'adapted_source_sha256': hashlib.sha256(text.encode()).hexdigest(),
                     'bridge_sha256': hashlib.sha256(bridge.encode()).hexdigest(),
                     'boundary': 'byte pointers, integer flags, and callbacks; no floating values cross into Rust'}
