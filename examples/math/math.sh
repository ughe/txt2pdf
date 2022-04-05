#!/usr/bin/env bash
# Generate math problems (2 digit numbers). Note 56 lines fit on a page
TOTAL=$(( 56 * 2 ))
OP='*' # OPERATOR
PROBLEMS=math
ANSWERS=math_answers
SEED=0 # Deterministic problems
python3 -c "NR=${TOTAL}; import sys; import random; random.seed($SEED); xs = list(range(10, 100)); x = lambda: random.choice(xs); rows = [(x(), x()) for _ in range(NR)]; print('\n'.join([f'{a} ${OP} {b} = ' for (a, b) in rows])); print('\n'.join([f'{a} ${OP} {b} = {a ${OP} b}' for (a, b) in rows]), file=sys.stderr)" > $PROBLEMS.txt 2> $ANSWERS.txt
../../txt2pdf -n"|3" -2 $PROBLEMS.txt > $PROBLEMS.pdf
../../txt2pdf -n"|3" -2 $ANSWERS.txt > $ANSWERS.pdf
rm $PROBLEMS.txt $ANSWERS.txt
