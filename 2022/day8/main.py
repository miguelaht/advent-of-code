#!/usr/bin/env

def parse_line(line: str) -> list[int]:
    return list(map(int, line))

def is_visible(current_tree: int, x:int, y: int) -> bool:
    line: list[int] = tree_map[y]
    column: tuple[int] = columns[x]

    left = all(n < current_tree for n in line[:x])
    right = all(n < current_tree for n in line[x+1:])

    up = all(n < current_tree for n in column[:y])
    down = all(n < current_tree for n in column[y+1:])

    return left or right or up or down

def get_scenic_score(current_tree: int, x: int, y: int) -> int:
    line: list[int] = tree_map[y]
    column: tuple[int] = columns[x]

    def calculate(trees):
        counter = 0
        for i in trees:
            if i > current_tree:
                break

            counter += 1

            if i is current_tree:
                break

        return counter

    left = calculate(line[:x][::-1])

    right = calculate(line[x+1:])

    up = calculate(column[:y][::-1])

    down = calculate(column[y+1:])

    return left * right * up * down

input: str = open("input.txt").read()

tree_map: list[list[int]] = list(map(parse_line, input.splitlines()))

length: int = len(tree_map[0])
height: int = len(tree_map)

count = ((length + height) * 2) - 4
columns = list(zip(*tree_map))

max_scenic_score: int = 0

for y, line in enumerate(tree_map):
    if y == 0 or y == height - 1:
        continue
    for x, tree in enumerate(line):
        if x == 0 or x == length - 1:
            continue

        visible = is_visible(tree, x, y)
        if visible:
            count += 1

        scenic_score: int = get_scenic_score(tree, x, y)
        if scenic_score > max_scenic_score:
            max_scenic_score = scenic_score

print(count)
print(max_scenic_score)
