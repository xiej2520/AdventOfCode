import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

res = 0

lines = []

for line in sys.stdin:
    lines.append(line.strip())

workflows = {}

i = 0
while i < len(lines):
    line = lines[i]
    idx = line.find('{')

    name = line[:idx]

    rules = line[idx+1:-1]
    rules = rules.split(',')
    workflows[name] = rules

    if len(line) == 0:
        i += 1
        break
    
    i += 1

print(workflows)

@cache
def dfs(wf, x, m, a, s):
    print(wf,x,m,a,s)
    if wf == 'R':
        return 0
    if x[1] <= x[0] or m[1] <= m[0] or a[1] <= a[0] or s[1] <= s[0]:
        return 0
    if wf == 'A':
        return (x[1] - x[0]) * (m[1] - m[0]) * (a[1] - a[0]) * (s[1] - s[0])

    res = 0
    for rule in workflows[wf]:
        if rule == 'A':
            res += (x[1] - x[0]) * (m[1] - m[0]) * (a[1] - a[0]) * (s[1] - s[0])
            break
        if rule == 'R':
            break
        if ':' not in rule:
            res += dfs(rule,x,m,a,s)
            return res
        t = rule.split(':')
        dest = t[1]
        t = t[0]
        if '>' in t:
            t = t.split('>')
            v = int(t[1])
            t = t[0]
            if t == 'x':
                res += dfs(dest, (max(x[0], v + 1), x[1]), m, a, s)
                x = (x[0], v+1)
            elif t == 'm':
                res += dfs(dest, x, (max(m[0],v+1),m[1]), a, s)
                m = (m[0], v+1)
            elif t == 'a':
                res += dfs(dest, x, m, (max(a[0],v+1),a[1]), s)
                a = (a[0], v+1)
            elif t == 's':
                res += dfs(dest, x, m, a, (max(s[0], v+1), s[1]))
                s = (s[0], v+1)
        else:
            #print("<","here")
            t = t.split('<')
            v = int(t[1])
            t = t[0]
            print(t,v)
            if t == 'x':
                res += dfs(dest, (x[0], v), m, a, s)
                x = (v, x[1])
            elif t == 'm':
                res += dfs(dest, x, (m[0], v), a, s)
                m = (v, m[1])
            elif t == 'a':
                res += dfs(dest, x, m, (a[0], v), s)
                a = (v, a[1])
            elif t == 's':
                res += dfs(dest, x, m, a, (s[0], v))
                s = (v, s[1])
    return res



res = dfs('in', (1,4001),(1,4001),(1,4001),(1,4001))


print(res)
print(167409079868000)
