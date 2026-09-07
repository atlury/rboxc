#!/usr/bin/env python3
"""Give one original test a private controlling input terminal."""
# SPDX-License-Identifier: GPL-3.0-or-later
import fcntl
import json
import os
import signal
import sys
import termios


def main():
    master, slave = os.openpty()
    child = os.fork()
    if child == 0:
        os.close(master)
        os.setsid()
        fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
        os.dup2(slave, 0)
        if slave != 0:
            os.close(slave)
        assert os.isatty(0) and os.tcgetpgrp(0) == os.getpgrp()
        print('RBOXC_TERMINAL_PROFILE '+json.dumps({'private': True, 'controlling_stdin': True}),
              file=sys.stderr, flush=True)
        os.execvp(sys.argv[1], sys.argv[1:])
    os.close(slave)

    def forward(signum, frame):
        try:
            os.killpg(child, signum)
        except ProcessLookupError:
            pass
        signal.alarm(3)

    def terminate(signum, frame):
        try:
            os.killpg(child, signal.SIGKILL)
        except ProcessLookupError:
            pass

    for signum in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(signum, forward)
    signal.signal(signal.SIGALRM, terminate)
    try:
        _, status = os.waitpid(child, 0)
        signal.alarm(0)
        code = os.waitstatus_to_exitcode(status)
        return code if code >= 0 else 128-code
    finally:
        os.close(master)


if __name__ == '__main__':
    raise SystemExit(main())
