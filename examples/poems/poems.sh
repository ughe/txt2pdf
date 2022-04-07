#!/usr/bin/env sh
for f in poem_*.txt; do ../../txt2pdf -p12 -n $f > ${f%.txt}.pdf ; done
