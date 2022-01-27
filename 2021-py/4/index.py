input = open('input.txt').read().split('\n\n')

numbers = list(map(int, input[0].split(',')))
boards = []

input = list(input[2:])
boards = [list(map(lambda l: " ".join(l.split()), b.splitlines())) for b in input]

def find_winner(board: list[str]):
    winner: str = 'M M M M M'
    total = 0
    if winner in board:
        total = sum([int(n) for i in board for n in i.split() if n.isdigit()])
    else:
        for i in range(0, 4):
            if winner == [r.split()[i] for r in board][0]:
                total = sum([int(n) for i in board for n in i.split() if n.isdigit()])

    return total

def draw_numbers():
    import re
    winner = 0

    for i, n in enumerate( numbers ):
        print(n)
        for bi, board in enumerate(boards):
            for ri, row in enumerate(board):
                boards[bi][ri] = re.sub(f'\\b{n}\\b', 'M', row)

            if i >= 4:
                winner = find_winner(boards[bi])
                if winner != 0:
                    print(winner, n, winner * n, boards[bi])
                    return

draw_numbers()
