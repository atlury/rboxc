#!/usr/bin/env python3
"""Capture GNU Findutils' private encoder compile and link records."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,os,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint,original_link
ROOT=Path(__file__).resolve().parents[1]
pin=json.loads((ROOT/'inventory/sources.json').read_text())['findutils']
source=Path(pin['source'])/'locate/frcode.c'
assert fingerprint(source)==pin['private_helper_entry_sha256']['frcode']
build=ROOT/'build/gnu-findutils/locate';assert (build/'Makefile').exists(), 'prepare the pinned native Findutils build first'
with (ROOT/'evidence/raw/frcode-native-record-refresh.log').open('w') as log:
 subprocess.run(['make','-W',str(source),'frcode','CC=python3 '+str(ROOT/'scripts/record-provider-cc.py')],cwd=build,
  env={**os.environ,'RBOXC_CC_RECORDS':str(ROOT/'build/findutils-cc-records')},stdout=log,stderr=subprocess.STDOUT,check=True)
assert original_link(ROOT,'frcode')
print('Recorded the pinned GNU frcode compile and link commands')
