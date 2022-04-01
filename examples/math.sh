#!/usr/bin/env bash
# Generate mental_math.pdf and mental_math_answers.pdf for practicing math
TOTAL=$(( 56 * 2 ))
python3 -c "NR=$TOTAL; import sys; import secrets; xs = list(range(10, 100)); x = lambda: secrets.choice(xs); rows = [(x(), x()) for _ in range(NR)]; print('\n'.join([f'{a} x {b} = ' for (a, b) in rows])); print('\n'.join([f'{a} x {b} = {a*b}' for (a, b) in rows]), file=sys.stderr)" > mental_math.txt 2> mental_math_answers.txt
../txt2pdf -n"|3" -2 mental_math.txt > mental_math.pdf
../txt2pdf -n"|3" -2 mental_math_answers.txt > mental_math_answers.pdf
rm mental_math.txt mental_math_answers.txt
