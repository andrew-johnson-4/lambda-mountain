#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
perf report -q -F overhead,symbol | awk '$2 == "[.]" { cmd = "'"${SCRIPT_DIR}"'/demangle \""$3"\""; cmd | getline sym; close(cmd); print $1 "\t" sym; }' | less
