#!/usr/bin/env bash

set -euo pipefail

openocd &
openocd_pid="${!}"

gdb_status=0
arm-none-eabi-gdb -q -x debug.gdb "${@}" || gdb_status="${?}"

kill "${openocd_pid}"

exit "${gdb_status}"
