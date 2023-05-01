#!/usr/bin/env bash

if [ ! -z "$1" ]
then
    bsatool add ../Starwind.bsa "$(echo $1 | sed 's|^./||')"
fi
