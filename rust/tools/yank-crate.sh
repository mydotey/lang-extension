#!/bin/bash

for n in `seq $4 $5`; do
    crate_version="$1@$2.$3.$n"
    echo "yank $crate_version"
    cargo yank $crate_version
done
