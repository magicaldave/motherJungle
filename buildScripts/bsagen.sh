#!/usr/bin/env bash

if [ ! -z "$1" ] && [ ! -z "$2" ]
then
    bsatool add ../$2 "$(echo "$1" | sed 's|^./||')"
fi
