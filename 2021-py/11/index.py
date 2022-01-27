import numpy as np
from copy import deepcopy

input = open('input.txt').read()
test = '''5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526'''

data = list(map(lambda l: list(map(lambda o: int(o), l.strip())), input.split('\n')))

def adjacent_octo(octos: list[list[int]]) -> int:

    for row in octos:
        for i, octo in enumerate(row):
            row[i] = octo + 1

    flashed = set()
    new_flash = set()
    new_flash.add((1,1))
    while new_flash:
        new_flash.clear()
        for i, row in enumerate(octos):
            for j, octopus in enumerate(row):
                if octopus > 9 and (i,j) not in flashed:
                    flashed.add((i,j))
                    new_flash.add((i,j))
                    x = [i]
                    if i > 0:
                        x.append(i-1)
                    if i < len(octos) - 1:
                        x.append(i+1)
                    y = [j]
                    if j > 0:
                        y.append(j-1)
                    if j < len(octos[0]) - 1:
                        y.append(j + 1)

                    for nx in x:
                        for ny in y:
                            octos[nx][ny] += 1

    for i, j in flashed:
        octos[i][j] = 0

    return len(flashed)


def solution():
    count = 0
    octos = deepcopy(data)
    for _ in range(100):
        count += adjacent_octo(octos)

def validate(octos):
    sync = [[0] * len(octos)] * len(octos)
    return octos == sync

def solution_2():
    step = 0
    octos = deepcopy(data)
    sync = False
    while not sync:
        adjacent_octo(octos)
        sync = validate(octos)
        step+=1

    print(step)

if __name__ == "__main__":
    solution_2()
