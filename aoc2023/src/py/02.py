import sys
from collections import defaultdict

res = 0
for line in sys.stdin:
    w = line.split(": ")
    x = w[1].strip()
    y = x.split("; ")

    poss = defaultdict(int)
    poss["red"] = 12
    poss["green"] = 13
    poss["blue"] = 14
    possible = True
    for g in y:
        z = g.split(", ")
        h = defaultdict(int)
        for a in z:
            b = a.split(" ")
            h[b[1]] += int(b[0])
        for k in h:
            if h[k] > poss[k]:
                possible = False
    if possible:
        id = int(w[0].split(" ")[1])
        res += id

print(res)
