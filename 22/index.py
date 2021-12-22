def get_input(test=False) -> list[str]:
    out = ''
    if test:
        with open('test.txt') as file:
            out = file.read().splitlines()
    else:
        with open('input.txt') as file:
            out = file.read().splitlines()

    return out

def parse_input(inp: str, limit: int = 0) -> tuple[str, list[int], list[int], list[int]]:
    t = inp.split(' ')
    s = t[0]
    d = t[1].split(',')
    steps: list[list[int]] = []
    for a in d:
        temp = a.split('..')
        temp = [int(temp[0][2:]), int(temp[1])]
        if limit > 0:
            temp.sort()
            if temp[0] < -limit:
                temp[0] = -limit
            if temp[1] > limit:
                temp[1] = limit

        steps.append(temp)

    return (s, steps[0], steps[1], steps[2])


def solve(ignore: int = 2, limit_region: int = 50):
    i = get_input(test=False)
    steps = [parse_input(s, limit_region) for s in i]
    if ignore > 0:
        steps = steps[:-ignore]
    cubes: dict[tuple, bool] = {}

    min_x = 0
    max_x = 0
    min_y = 0
    max_y = 0
    min_z = 0
    max_z = 0
    while steps:
        s, x, y, z = steps.pop(0)
        state = True if s == 'on' else False
        if state:
            if x[0] < min_x:
                min_x = x[0]
            if x[1] > max_x:
                max_x = x[1]
            if y[0] < min_y:
                min_y = y[0]
            if y[1] > max_y:
                max_y = y[1]
            if z[0] < min_z:
                min_z = z[0]
            if z[1] > max_z:
                max_z = z[1]

        # ignore off cubes
        if not state:
            if x[0] < min_x:
                x[0] = min_x
            if x[1] > max_x:
                x[1] = max_x
            if y[0] < min_y:
                y[0] = min_y
            if y[1] > max_y:
                y[1] = max_y
            if z[0] < min_z:
                z[0] = min_z
            if z[1] > max_z:
                z[1] = max_z


        for i in range(x[0], x[1]+1):
            for j in range(y[0], y[1]+1):
                for k in range(z[0], z[1]+1):
                    cubes[(i,j,k)] = state

    count = len(list(filter(lambda k: k, list(cubes.values()))))
    print(count)



if __name__ == "__main__":
    solve(2, 50)
