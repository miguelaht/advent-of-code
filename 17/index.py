input = 'target area: x=153..199, y=-114..-75'
test = 'target area: x=20..30, y=-10..-5'

def parse_input(inp: str) -> dict[str, list[int]]:
    p = inp.split(' ')
    x = p[2] # x=0...0,
    y = p[3] # y=0...0

    x = x[2:-1].split('..')
    x_start, x_end = [int(n) for n in x]
    y = y[2:].split('..')
    y_start, y_end = [int(n) for n in y]

    return { 'y': [y_start, y_end], 'x': [x_start, x_end] }

def hit(x, y, x_min, x_max, y_min, y_max):
    return x_min <= x <= x_max and y_min  <= y <= y_max

def launch(xv, yv, bounds):
    x_min, x_max = bounds['x']
    y_min, y_max = bounds['y']
    x, y = 0, 0
    hits = False
    highest = 0

    while True:
        x += xv
        y += yv

        highest = max(y, highest)

        if xv > 0:
            xv -= 1
        elif xv < 0:
            xv += 1
        else:
            xv

        yv -= 1

        if x > x_max or y < y_min:
            break

        hits = hit(x, y, x_min, x_max, y_min, y_max)
        if hits:
            break

    return hits, highest


def solution():
    t = parse_input(input)
    _, x_max = t['x']
    y_min, _ = t['y']
    velocities = 0
    highest = 0

    for x in range(x_max+1):
        for y in range(y_min,abs(y_min)):
            hits, h= launch(x, y, t)
            highest = max(h, highest)
            if hits:
                velocities += 1

    print(highest)
    print(velocities)


if __name__ == "__main__":
    solution()
