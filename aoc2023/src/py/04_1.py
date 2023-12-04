import sys
from collections import defaultdict

res = 0
lines = []
for line in sys.stdin:
    lines.append(line)

cards = [1 for _ in range(len(lines))]
matches_ = []

for index, line in enumerate(lines):
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
    matches_.append(matches)

for i in range(len(lines)):
    res += cards[i]
    for j in range(i+1, i+1+matches_[i]):
        if j >= len(cards):
            break
        cards[j] += cards[i]

print(cards)
print(matches_)

print(res)
