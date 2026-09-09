# Test-only startup allowance. The instrumented client restores the effective soft limit to 7.
# SPDX-License-Identifier: GPL-3.0-or-later
ulimit() {
  if [[ $# == 2 && $1 == -n && $2 == 7 ]]; then
    builtin ulimit -S -n 64 || return
    export RBOXC_SORT_FD_LIMIT=7
  else
    builtin ulimit "$@"
  fi
}
