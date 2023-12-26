import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math
from queue import Queue

sys.setrecursionlimit(100000000)

lines = []

for line in sys.stdin:
    lines.append(line.strip())

modules = {}

for line in lines:
    x = line.split(" -> ")
    s = x[0]
    if s[0] in '%&':
        name = s[1:]
    else:
        name = s
    modules[name] = (s[0], x[1].split(", "))

print(modules)

low_sent, high_sent = 0, 0

states = {}
for k in modules:
    if modules[k][0] == '&':
        states[k] = {}
    else:
        states[k] = False

for k in modules:
    for r in modules[k][1]:
        if r in modules and modules[r][0] == '&':
            states[r][k] = False

def push():
    q = Queue()
    q.put(('broadcaster', False, 'button'))
    sent = defaultdict(lambda: 0)
    
    low_sent, high_sent = 0, 0
    while not q.empty():
        recv, level, send = q.get()
        if send in ['xf', 'fz', 'hn', 'mp'] and level:
        #if send in ['jn', 'jl', 'gp', 'fb'] and not level:
            sent[send] += 1
            print(low_sent, high_sent)

        #global low_sent, high_sent
        if level:
            high_sent += 1
        else:
            low_sent += 1
        #print(send, '-',level,'->',recv)

        if recv not in modules:
            pass
        elif modules[recv][0] == 'b': # broadcaster
            for c in modules['broadcaster'][1]:
                q.put((c, level, recv))

        elif modules[recv][0] == '%':
            if not level:
                states[recv] = not states[recv]
                for c in modules[recv][1]:
                    q.put((c, states[recv], recv))

        elif modules[recv][0] == '&':
            all_high = True
            states[recv][send] = level
            for k in states[recv]:
                if not states[recv][k]:
                    all_high = False
                    break
            #print(recv, states[recv], all_high)
            for c in modules[recv][1]:
                q.put((c, not all_high, recv))
    return sent

print(states['hn'])
print(states['mp'])
print(states['xf'])
print(states['fz'])
print(states['xn'])
i = 1
while True:
    res = push()
    #print(res)
    #print(i, states['xn'])
    if bool(res):
        print(i, res)
    #for j in states['xn']:
    #    if states['xn'][j]:
    #        print(i, j)

    if res == 1:
        print(i)
        break
    i += 1
#for i in range(1000):
#    print(i, push())
    #print('------')
