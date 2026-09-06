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
    match = re.search(r'^static [^\n]+\n'+re.escape(name)+r' \(', text, re.M)
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
