input = open('input.txt').read()
test = '''1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581'''
t = open('test.txt').read()

def parse_input(inp: str) -> list[list[int]]:
    return list(map(lambda l: list(map(lambda r: int(r), list(l))), inp.split('\n')))

def scale_y(caves: list[list[int]]) -> list[list[int]]:
    c = caves
    le = len(caves)
    new = [*c]
    for _ in range(4):
        for o in range(le):
            r = c[o]
            t = [n+1 if n + 1 <= 9 else abs(9 - (n+1)) for n in r]
            c[o] = t

        new.extend(c)

    caves = new

    return caves

def scale_x(caves: list[list[int]]) -> list[list[int]]:
    for i in range(len(caves)):
        r = caves[i]
        l = []
        for p in range(5):
            l.extend([n+p if n + p <= 9 else abs(9 - (n+p)) for n in r])

        caves[i] = l

    return caves

def get_adjacent(point: tuple, i_max: int, j_max: int) -> list[tuple]:
    adj: list[tuple] = []
    i, j = point

    if j < j_max - 1:
        adj.append((i, j+1))
    if j > 0:
        adj.append((i, j-1))
    if i < i_max - 1:
        adj.append((i+1, j))
    if i > 0:
        adj.append((i-1, j))

    return adj

def find_path(caves: list[list[int]]):
    paths: dict[tuple, int] = {(0,0):0}
    final_map: dict[tuple, int] = {(0,0):0}
    end = (len(caves)-1, len(caves[0])-1)

    while(len(paths) > 0 ):
        point = min(paths.keys(), key=(lambda k: paths[k]))
        paths.pop(point)

        for p in get_adjacent(point, len(caves), len(caves[0])):
            if p not in final_map:
                i, j = p
                risk = final_map[point] + caves[i][j]
                final_map[p] = risk

                if p == end:
                    break

                paths[p] = risk

    print(final_map[end])

def sollution_1():
    caves = parse_input(input)
    find_path(caves)

def sollution_2():
    caves = parse_input(input)
    new_caves = scale_x(caves)
    new_caves = scale_y(new_caves)
    find_path(new_caves)

if __name__ == "__main__":
    sollution_1()
    sollution_2()
