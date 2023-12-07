import sys
from collections import defaultdict, Counter

res = 99999999999
lines = []
res = 0

strength = 'A K Q J T 9 8 7 6 5 4 3 2'.split()
def st(i):
    return strength.index(i)

hb = []
for line in sys.stdin:
    x = line.split()
    hb.append((x[0], int(x[1])))

def ranking(x):
    c = Counter(x[0])
    s = list(c.most_common())
    s = sorted(s, key=lambda x: x[1], reverse=True)
    if s[0][1] == 5:
        r = 1
    elif s[0][1] == 4:
        r = 2
    elif s[0][1] == 3 and s[1][1] == 2:
        r = 3
    elif s[0][1] == 3 and s[1][1] == 1:
        r = 4
    elif s[0][1] == 2 and s[1][1] == 2:
        r = 5
    elif s[0][1] == 2 and s[1][1] == 1 and s[2][1] == 1:
        r = 6
    else:
        r = 7
    return (r, list(map(lambda x: st(x[0]), list(x[0]))))


hb = sorted(hb, key=ranking, reverse=True)
print(hb)

res = 0
for i in range(len(hb)):
    res += (i + 1) * hb[i][1]
print(res)