def get_input(test=False) -> list[str]:
    out = ''
    if test:
        with open('test.txt') as file:
            out = file.read().splitlines()
    else:
        with open('input.txt') as file:
            out = file.read().splitlines()

    return out

def solve():
    import numpy as np
    data: list[list[str]] = list(map(lambda l: list(l), get_input(test=False)))
    steps = 0
    moves = 1

    while moves > 0:
        moves = 0
        seen_e: set[tuple[int, int]] = set()
        for y, row in enumerate(data):
            for x, c in enumerate(row):
                if (y, x) not in seen_e:
                    if c == '>':
                        pos = -1
                        if x+1 == len(row):
                            if row[0] == '.':
                                pos = 0
                        elif row[x+1] == '.':
                            pos = x+1
                        if pos != -1:
                            if (y, pos) not in seen_e:
                                data[y][pos] = '>'
                                data[y][x] = '.'
                                seen_e.add((y, x))
                                seen_e.add((y, pos))
                                moves += 1

        seen_s: set[tuple[int, int]] = set()
        for y, row in enumerate(data):
            for x, c in enumerate(row):
                if (y, x) not in seen_s:
                    if c == 'v':
                        pos = -1
                        if y+1 == len(data):
                            if data[0][x] == '.':
                                pos = 0
                        elif data[y+1][x] == '.':
                            pos = y+1
                        if pos != -1:
                            if (pos, x) not in seen_s:
                                data[y][x] = '.'
                                data[pos][x] = 'v'
                                seen_s.add((y, x))
                                seen_s.add((pos, x))
                                moves += 1

        if moves > 0:
            steps += 1
            print(steps, '\n', np.matrix(data), '\n')

    print(steps + 1)



if __name__ == "__main__":
    solve()
