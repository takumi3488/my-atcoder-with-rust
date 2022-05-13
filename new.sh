#!/bin/zsh

if [[ $1 =~ "^[0-9]+$" ]] then
  cargo atcoder new "abc$1" && \
  python3 add_packages.py "abc$1"
else
  cargo atcoder new $1 && \
  python3 add_packages.py $1
fi
