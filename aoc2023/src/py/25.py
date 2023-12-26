import networkx as nx
import sys

g = nx.Graph()

for line in sys.stdin:
    i, c = line.strip().split(': ')
    c = c.split(' ')
    for j in c:
        g.add_edge(i, j)

cut = nx.minimum_edge_cut(g)
for i, j in cut:
    g.remove_edge(i, j)

comps = list(nx.connected_components(g))
print(len(comps[0]) * len(comps[1]))
