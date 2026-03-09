#!/usr/bin/env bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 \"command to execute\""
  exit 1
fi

SCRIPT_DIR=$(cd -- "$(dirname -- "$0")" &>/dev/null && pwd)
TIMESTAMP=$(date +"%Y%m%d_%H%m%s")
LOG_FILE="$SCRIPT_DIR/out-$TIMESTAMP.log"

printf "Command: $1\n==================================\n" > "$LOG_FILE"
eval "$1" >> "$LOG_FILE" 2>&1 &
pid=$!

trap "kill -TERM -$pid 2>/dev/null; printf '\n\n [!] Run cancelled. Log file in %s\n' \"$LOG_FILE\"; exit 1" INT

spin='-\|/'
i=0

while kill -0 $pid 2>/dev/null; do
  i=$(( (i+1) % 4 ))
  printf "\r [%c] Running: %s... " "${spin:$i:1}" "${1:0:30}"
  sleep 1
done

trap - INT
wait $pid
res=$?

if [ $res -eq 0 ]; then
  printf "\r [+] Completed! ($1)                                    \n"
  rm "$LOG_FILE"
else
  printf "\r [!] Error (exit code: $res for command $1). Log in: %s \n" "$(basename "$LOG_FILE")"
fi
