import sys
from collections import defaultdict

res = 0
for line in sys.stdin:
    w = line.split(": ")
    x = w[1].strip()
    y = x.split("; ")

    h = defaultdict(int)
    for g in y:
        z = g.split(", ")
        for a in z:
            b = a.split(" ")
            h[b[1]] = max(h[b[1]], int(b[0]))
    p = 1
    for k in h:
        p *= h[k]
    res += p

print(res)
