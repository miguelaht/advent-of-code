input = open('input.txt').readlines()

class Node:
    name: str
    children: None

    def __init__(self, name: str) -> None:
        self.name = name

routes = {}
for route in input:
    r = route.split(' ')
    if r[0] not in routes:
        routes[r[0]][r[2]] = int(r[4])
    else:
        if r[2] in routes[r[0]]:
            routes[r[0]][r[2]] = int(r[4])
