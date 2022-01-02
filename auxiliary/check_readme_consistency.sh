#!/bin/bash

cargo readme -r rinflux -t ../README.tpl > README.md.expected

diff README.md README.md.expected

if [ $? -eq 0 ]
then
    echo 'README.md is up to date!'
    exit 0
else
    echo 'README.md out of date. Run "cargo readme -r rinflux -t ../README.tpl > README.md" and commit again.'
    exit 1
fi