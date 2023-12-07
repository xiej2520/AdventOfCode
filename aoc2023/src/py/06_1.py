import sys
from collections import defaultdict

res = 99999999999
lines = []
for line in sys.stdin:
    lines.append(line)

time = int("".join(lines[0].split(":")[1].split()))
dist = int("".join(lines[1].split(":")[1].split()))

w = 0
for t in range(0, time+1):
    if t * (time - t) > dist:
        w += 1

print(w)
