import sys

res = 0
ll = {'0':0,'1':1,'2':2,'3':3,'4':4,'5':5,'6':6,'7':7,'8':8,'9':9,
'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five':5, 'six':6, 'seven':7, 'eight':8, 'nine':9}
for line in sys.stdin:
    l = []
    if not line:
        pass
    else:
        for i in range(0, len(line)):
            for j in ll:
                if line[i:i+len(j)] == j:
                    l.append(j)
                    break

    res += int(str(ll[l[0]]) + str(ll[l[-1]]))

print(res)
