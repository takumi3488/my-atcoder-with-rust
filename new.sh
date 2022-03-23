#!/bin/zsh

read "id?Project id: " && \
cargo atcoder new $id && \
python3 add_packages.py $id