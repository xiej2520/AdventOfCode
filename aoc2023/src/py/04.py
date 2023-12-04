import sys
from collections import defaultdict

res = 0
for line in sys.stdin:
    id, c = line.split(": ")
    id = id[5:]
    c1, c2 = c.split(" | ")
    m = {}
    for i in c1.split():
        m[int(i)] = True
    matches = 0
    for j in c2.split():
        j = int(j)
        if j in m:
            matches += 1
    print(matches)
    if matches > 0:
        res += (1 << (matches - 1))



print(res)
