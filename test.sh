#!/bin/bash

mkdir target -p

CFLAGS="-lm"
SOURCES=$(find src -type f -name "*.c" ! -name "main.c")
TARGET=target/"$(basename "$1" .c)"

rm "$TARGET" -f

# shellcheck disable=SC2086
gcc "$1" $SOURCES -o "$TARGET" $CFLAGS
if [[ ! -f "$TARGET" ]]; then
  echo "Error: $1 failed to compile"
  exit 1
fi

# Run
./target/"$(basename "$1" .c)"
