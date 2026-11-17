#!/usr/bin/env python3
"""Drives the interactive menu in a pseudo-terminal with a scripted key sequence.

Read-only: it opens the menu, moves, opens an analysis, toggles the
"failing only" filter, ignores and un-ignores an item in a temporary config
directory, and quits with q. It never presses f, so no fix runs.
"""
import os
import pty
import select
import sys
import tempfile
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CLI = os.environ.get("MACGUARD_BIN", os.path.join(ROOT, "target", "release", "macguard"))


def read_until(fd, needle, timeout=60):
    buf = b""
    end = time.time() + timeout
    while time.time() < end:
        r, _, _ = select.select([fd], [], [], 0.2)
        if r:
            try:
                chunk = os.read(fd, 65536)
            except OSError:
                break
            if not chunk:
                break
            buf += chunk
            if needle in buf:
                return buf
    raise AssertionError("timed out waiting for %r; last output: %r" % (needle, buf[-400:]))


def main():
    cfg = tempfile.mkdtemp()
    env = dict(os.environ, MACGUARD_LANG="en", MACGUARD_CONFIG_DIR=cfg, TERM="xterm", COLUMNS="120", LINES="40")
    pid, fd = pty.fork()
    if pid == 0:
        os.execve(CLI, [CLI], env)

    def send(keys, wait_for):
        os.write(fd, keys)
        return read_until(fd, wait_for)

    read_until(fd, b"Enter analyse")                 # list rendered after the scan
    send(b"j", b"Enter analyse")                     # move down
    send(b"\r", b"What it does")                     # open the analysis of the selected item
    send(b"\x1b", b"Enter analyse")                  # Esc goes back to the list
    send(b"a", b"failing items only")        # filter
    send(b"i", b"ignored; it will not be checked")   # ignore the selected item
    ignored = open(os.path.join(cfg, "ignored")).read().split()
    assert len(ignored) == 1, ignored
    send(b"i", b"will be checked again")             # un-ignore it
    assert open(os.path.join(cfg, "ignored")).read().split() == [], "ignore list not cleared"
    os.write(fd, b"q")
    # Keep draining the terminal so the menu never blocks on a full buffer.
    end = time.time() + 30
    while time.time() < end:
        done, status = os.waitpid(pid, os.WNOHANG)
        if done:
            break
        if select.select([fd], [], [], 0.2)[0]:
            try:
                os.read(fd, 65536)
            except OSError:
                pass
    else:
        os.kill(pid, 9)
        raise AssertionError("menu did not quit on q")
    assert os.WEXITSTATUS(status) == 0, status
    print("ok   interactive menu (move, analyse, back, filter, ignore, unignore, quit)")


if __name__ == "__main__":
    try:
        main()
    except AssertionError as e:
        print("FAIL interactive menu: %s" % e)
        sys.exit(1)
