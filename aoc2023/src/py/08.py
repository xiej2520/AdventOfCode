import heapq
import sys
from collections import defaultdict, Counter

lines = []
res = 0

for line in sys.stdin:
    lines.append(line)

adj = defaultdict(lambda: ("",""))

for i in range(2, len(lines)):
    a = lines[i][0:3]
    l = lines[i][7:10]
    r = lines[i][12:15]
    adj[a] = (l, r)

visited = defaultdict(lambda:False)

q = []
heapq.heappush(q, ('AAA', 0))

#dist = defaultdict(lambda: 99999)
#
#while q:
#    i, d = heapq.heappop(q)
#    if d + 1 < dist[adj[i][0]]:
#        heapq.heappush(q, (adj[i][0], d + 1))
#        dist[adj[i][0]] = d + 1
#    if d + 1 < dist[adj[i][1]]:
#        heapq.heappush(q, (adj[i][1], d + 1))
#        dist[adj[i][1]] = d + 1

dirs = lines[0].strip()

cur = 'AAA'
i = 0
while cur != 'ZZZ':
    print("cur", cur)
    print(i, dirs[i % len(dirs)])
    if dirs[i % len(dirs)] == 'R':
        cur = adj[cur][1]
    else:
        cur = adj[cur][0]
    i += 1
print(cur)

print(i)