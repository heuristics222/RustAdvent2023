from collections import defaultdict, deque
from pprint import pprint

with open("input.txt") as f:
    #content = f.readlines()
    content = [x.strip() for x in f.readlines()]

grid = {}
start = None
w = len(content[0])
h = len(content)
start = (65,65)
for y, v1 in enumerate(content):
    for x, v2 in enumerate(v1):
        grid[(x, y)] = v2

steps = 26501365
neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
visited = defaultdict(set)
visited[0].add(start)
prev_len = 0
a=[]
for s in range(steps):
    for point in visited[s]:
        x, y = point
        for n in neighbors:
            dx, dy = n
            ix, iy = x+dx, y+dy
            # print(x, dx, y, dy, ix, iy, grid.get((ix, iy), None))
            if grid.get((ix % w, iy % h), None) in ['.']:
                visited[s+1].add((ix, iy))

    if s % w == steps % w:
        print(s, len(visited.get(s)), len(visited.get(s)) - prev_len, s // w)
        prev_len = len(visited.get(s))
        a.append(prev_len)

    if len(a) == 3:
        break

# print(len(visited.get(len(visited)-1)))


def f(n):
    b0 = a[0]
    b1 = a[1]-a[0]
    b2 = a[2]-a[1]
    return b0 + b1*n + (n*(n-1)//2)*(b2-b1)
print(f(steps//w))
