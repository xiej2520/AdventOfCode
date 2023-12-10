import heapq
import sys
from collections import defaultdict, Counter

lines = []
res = 0

sys.setrecursionlimit(100000000)

for line in sys.stdin:
    lines.append(line.strip())

m = len(lines)
n = len(lines[0])

sx = 0
sy = 0
for i in range(m):
    for j in range(n):
        if lines[i][j] == 'S':
            sx = i
            sy = j
            break
        
lc = []
for l in lines:
    lc.append(list(l))

def dfs(i, j, prev):

    c = lines[i][j]
    lc[i][j] = "#"
    if i == sx and j == sy:
        return 0
    if c == '|':
        if prev == 'u':
            return 1 + dfs(i+1, j, 'u')
        else:
            return 1 + dfs(i-1, j, 'd')
    elif c == '-':
        if prev == 'l':
            return 1 + dfs(i, j+1, 'l')
        else:
            return 1 + dfs(i, j-1, 'r')
    elif c == 'L':
        if prev == 'u':
            return 1 + dfs(i, j+1, 'l')
        else:
            return 1 + dfs(i-1, j, 'd')
    elif c == 'J':
        if prev == 'u':
            return 1 + dfs(i, j-1, 'r')
        else:
            return 1 + dfs(i-1, j, 'd')
    elif c == '7':
        if prev == 'l':
            return 1 + dfs(i+1, j, 'u')
        else:
            return 1 + dfs(i, j-1, 'r')
    elif c == 'F':
        if prev == 'd':
            return 1 + dfs(i, j+1, 'l')
        else:
            return 1 + dfs(i+1, j, 'u')
    else:
        return 999999

if sx < m - 1:
    a = dfs(sx+1, sy, 'u')
    pass
else:
    a = 999999999
#print("\n\n\nb")

if sx > 0:
    #b = dfs(sx-1, sy, 'd')
    b = 999999999
    pass
else:
    b = 999999999

#print("\n\n\nc")

#if sy < n - 1:
#    c = dfs(sx, sy+1, 'u')
#else:
#    c = 999999999

#print("\n\n\nd")

if sy > 0:
    #d = dfs(i, j-1, 'r')
    d = 999999999
    pass
else:
    d = 999999999

#print(a,b,c,d)
#print(min(a,b,c,d))
#print((min(a,b,c,d) + 1) / 2)

def f(i,j):
    if i < 0 or j < 0 or i >= m or j >= n:
        return
    if lc[i][j] == '#' or lc[i][j] == '0':
        return
    global res
    res += 1
    lc[i][j] = '0'
    f(i-1,j)
    f(i+1,j)
    f(i,j-1)
    f(i,j+1)


f(0,0)

res = 0
for i in range(m):
    v = 0
    for j in range(n):
        if lc[i][j] == '#':
            if lines[i][j] in ['|','S','F','7']:
                lc[i][j]='v'
                v += 1
        elif v % 2 == 1:
            lc[i][j] = '-'
            res += 1
print(res)

print("\n\n\n")
for i in lc:
    print("".join(i))

res = 0
for i in range(m):
    for j in range(n):
        if lc[i][j] != '#' and lc[i][j] != '0':
            lc[i][j] = '='
            res += 1
#print("\n\n\n")
#for i in lc:
#    print("".join(i))
#print(res)

fl = []
for i in range(m):
    t = []
    m = []
    b = []
    for j in range(n):
        c = lines[i][j]
        if lc[i][j] != '#':
            c = '.'
        if c == '|':
            t.extend(['.','#','.'])
            m.extend(['.','#','.'])
            b.extend(['.','#','.'])
        elif c == '-':
            t.extend(['.','.','.'])
            m.extend(['#','#','#'])
            b.extend(['.','.','.'])
        elif c == 'L':
            t.extend(['.','#','.'])
            m.extend(['.','#','#'])
            b.extend(['.','.','.'])
        elif c == 'J':
            t.extend(['.','#','.'])
            m.extend(['#','#','.'])
            b.extend(['.','.','.'])
        elif c == '7':
            t.extend(['.','.','.'])
            m.extend(['#','#','.'])
            b.extend(['.','#','.'])
        elif c == 'F':
            t.extend(['.','.','.'])
            m.extend(['.','#','#'])
            b.extend(['.','#','.'])
        elif c == 'S':
            t.extend(['.','#','.'])
            m.extend(['.','S','.'])
            b.extend(['.','#','.'])
        else:
            t.extend(['.','.','.'])
            m.extend(['.','.','.'])
            b.extend(['.','.','.'])
    fl.extend([t,m,b])

m = len(fl)
n = len(fl[0])

q = []
q.append((0,0))
while q:
    i, j = q.pop()
    #print(i,j,m,n,len(fl),len(fl[0]))
    if i < 0 or j < 0 or i >= m or j >= n:
        continue
    if fl[i][j] == '#' or fl[i][j] == '`' or fl[i][j] in ['S','J','L','7','F']:
        continue
    fl[i][j] = '`'
    q.append((i-1, j))
    q.append((i+1, j))
    q.append((i, j-1))
    q.append((i, j+1))

res = 0
for i in range(1,m,3):
    for j in range(1,n,3):
        if fl[i][j] == '.':
            possible = True
            for (dx,dy) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]:
                if fl[i+dx][j+dy] != '.':
                    possible = False
                    break
            if possible:
                for (dx,dy) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]:
                    fl[i+dx][j+dy] = '0'
                fl[i][j] = '0'
                res += 1

for i in fl:
    print("".join(i))
print(res)