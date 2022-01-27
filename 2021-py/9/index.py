input: str = open("input.txt").read()
test_input: str = """2199943210
3987894921
9856789892
8767896789
9899965678"""

matrix: list[list[int]] = [[int(n) for n in line] for line in input.split("\n")]


def find_adjacent(i, j) -> list[int]:
    adj: list[int] = []
    if i - 1 >= 0:
        adj.append(matrix[i - 1][j])
    if i + 1 < len(matrix):
        adj.append(matrix[i + 1][j])
    if j - 1 >= 0:
        adj.append(matrix[i][j - 1])
    if j + 1 < len(matrix[0]):
        adj.append(matrix[i][j + 1])

    return adj


def solution_1() -> None:
    result: list[int] = []
    for i in range(0, len(matrix)):  # v
        for j in range(0, len(matrix[0])):  # h
            adj = min(find_adjacent(i, j))
            if matrix[i][j] < adj:
                result.append(matrix[i][j])

    print(result, sum(result) + len(result))


result: list[int] = []


def find_basin(i, j):
    if (
        i < 0
        or i >= len(matrix)
        or j < 0
        or j >= len(matrix[0])
        or matrix[i][j] == 9
        or matrix[i][j] == -1
    ):
        return
    matrix[i][j] = -1
    result[-1] += 1
    find_basin(i - 1, j)
    find_basin(i + 1, j)
    find_basin(i, j - 1)
    find_basin(i, j + 1)


def solution_2() -> None:
    for i in range(0, len(matrix)):  # v
        for j in range(0, len(matrix[0])):  # h
            result.append(0)
            find_basin(i, j)

    result.sort(reverse=True)
    print(result[0] * result[1] * result[2])


solution_2()
