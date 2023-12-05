import sys
from collections import defaultdict

res = 99999999999
lines = []
for line in sys.stdin:
    lines.append(line)

n = len(lines)

seeds = list(map(int, lines[0].split(": ")[1].split()))

maps = []
i = 3
while i < n:
    m = []
    while i < n and not lines[i].isspace():
        m.append(list(map(int, lines[i].split())))
        i += 1
    maps.append(m)
    i += 2


for seed in seeds:
    j = 0
    x = seed
    for m in maps:
        for lm in m:
            if seed >= lm[1] and seed < lm[1] + lm[2]:
                seed = lm[0] + seed - lm[1]
                break
    res = min(res, seed)

print(res)
