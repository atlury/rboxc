#!/bin/sh
# various csplit tests

# Copyright (C) 2001-2026 Free Software Foundation, Inc.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

. "${srcdir=.}/tests/init.sh"; path_prepend_ ./src
print_ver_ csplit


# 'echo | csplit -b '%0#6.3x' - 1' incorrectly warned about the format
# up through coreutils 8.6.
echo > in
csplit -b '%0#6.3x' in 1 > out 2> err || fail=1
cat <<EOF > exp
0
1
EOF
compare exp out || fail=1
touch experr
compare experr err || fail=1
compare 'xx   000' experr || fail=1
compare 'xx 0x001' in || fail=1
rm -f in out exp err experr xx*

# make sure 'csplit FILE 0' fails.
echo > in
csplit in 0 > out 2> err && fail=1
csplit in 2 1 > out 2>> err && fail=1
csplit in 3 3 > out 2>> err && fail=1
cat <<\EOF > experr
csplit: 0: line number must be greater than zero
csplit: line number '1' is smaller than preceding line number, 2
csplit: warning: line number '3' is the same as preceding line number
csplit: '3': line number out of range
EOF
compare experr err || fail=1

# Ensure file not created for empty input
# which was the case with coreutils <= 9.5
rm -f xx??
csplit /dev/null 1 >/dev/null 2>err && fail=1
test -f xx00 && fail=1
cat <<\EOF > experr
csplit: '1': line number out of range
EOF
compare experr err || fail=1

Exit $fail
