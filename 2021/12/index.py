def get_input(test=False) -> list[str]:
    out = ''
    if test:
        with open('test.txt') as file:
            out = file.read().splitlines()
    else:
        with open('input.txt') as file:
            out = file.read().splitlines()

    return out

def parse_input(inp: list[str]) -> dict[str, list[str]]:
    caves: dict[str, list[str]] = {}
    for line in inp:
        a, b = line.split('-')
        if b not in ['start']:
            if a in caves:
                caves[a].append(b)
            else:
                caves[a] = [b]


        if a not in ['start']:
            if b in caves:
                caves[b].append(a)
            else:
                caves[b] = [a]

    return caves


def find_end(path: str, paths: dict[str, list[str]], seen: list[str] = []) -> int:
    count = 0
    if path == 'end':
        return 1

    s = seen.copy()

    if path.islower():
        s.append(path)

    for p in paths[path]:
        if p not in s and 'twice' not in s:
            count += find_end(p, paths, s)

    return count


def solution():
    inp = get_input(test=True)
    paths = parse_input(inp)
    count = 0
    count += find_end('start', paths, [])

    print(count)

def find_end_2(path: str, paths: dict[str, list[str]], seen: list[str] = [], twice: bool = False) -> int:
    count = 0
    if path == 'end':
        return 1

    s = seen.copy()

    if not twice:
        if path.islower():
            s.append(path)

    for p in paths[path]:
        if p not in s:
            count += find_end_2(p, paths, s, False)

    for p in paths[path]:
        if p in s:
            count += find_end_2(p, paths, s, True)

    return count

def solution_2():
    inp = get_input(test=True)
    paths = parse_input(inp)
    count = 0
    start = paths['start']
    for p in start:
        t = p.islower()
        count += find_end_2(p, paths, [], t)

    print(count)
    pass


if __name__ == "__main__":
    solution()
