#!/usr/bin/env sh
for f in poem_*.txt; do ../../txt2pdf -n $f > ${f%.txt}.pdf ; done
