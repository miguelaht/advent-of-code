import math

input = open('input.txt').read()
test = '''6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5'''

points = list(map(lambda p: list(map(lambda n: int(n), p.split(','))), input.split('\n\n')[0].split('\n')))
instructions = list(map(lambda i: i.split(' ')[2].split('='), input.split('\n\n')[1].split('\n')))

def fold_x(n: int, paper: list[tuple]) -> list[tuple]:
    res = set()

    for x, y in paper:
        if x > n:
            new_pos = (n - (x-n))
            res.add((new_pos, y))
        else:
            res.add((x,y))

    return list(res)

def fold_y(n: int, paper: list[tuple]) -> list[tuple]:
    res = set()

    for x, y in paper:
        if y > n:
            new_pos = (n - (y-n))
            res.add((x, new_pos))
        else:
            res.add((x,y))

    return list(res)

def graph(paper):
    import numpy as np
    paper.sort()
    table = [[' ' for _ in range(6)] for _ in range(39)]

    for x, y in paper:
        table[x][y] = '#'

    print(np.matrix(table))

def solution():
    paper: list[tuple] = []

    for x, y in points: paper.append((x,y))
    # fold
    for axis, n in instructions:
        if axis == 'x':
            paper = fold_x(int(n), paper)
        else:
            paper = fold_y(int(n), paper)

    graph(paper)

solution()
