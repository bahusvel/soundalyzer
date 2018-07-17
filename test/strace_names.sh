#!/bin/bash
strace $@ 3>&1 1>&2 2>&3 | stdbuf -oL grep -Eo '^[^(]+'
