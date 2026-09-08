/* Use the glibc files provider for passwd lookups in the tilde fixture.
   This test-only preload is shared by GNU and candidate processes.
   SPDX-License-Identifier: GPL-3.0-or-later */
#include <nss.h>
#include <unistd.h>

__attribute__((constructor)) static void use_files_for_passwd(void) {
  if (__nss_configure_lookup("passwd", "files") != 0)
    _exit(125);
}
