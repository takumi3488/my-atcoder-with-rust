#!/bin/bash

read -p "Project id: " id
cargo atcoder new $id
python3 add_packages.py $id