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

res = 0
while i < len(lines):
    part = lines[i][1:-1]
    part = part.split(',')
    desc = {}
    for d in part:
        x = d.split('=')
        desc[x[0]] = int(x[1])
    print(part, desc)
    
    workflow = 'in'
    while workflow not in ['A', 'R']:
        print(workflow)
        rules = workflows[workflow]
        for rule in rules:
            #print('.',rule)
            if rule == 'A':
                workflow = 'A'
                break
            if rule == 'R':
                workflow = 'R'
                break
            if ':' not in rule:
                workflow = rule
                break

            x = rule.split(':')
            if '>' in x[0]:
                y = x[0].split('>')
                if desc[y[0]] > int(y[1]):
                    workflow = x[1]
                    break
            elif '<' in x[0]:
                y = x[0].split('<')
                if desc[y[0]] < int(y[1]):
                    workflow = x[1]
                    break
    print(workflow)
    if workflow == 'A':
        res += desc['x'] + desc['m'] + desc['a'] + desc['s']
        print(res)

    i += 1


print(res)
