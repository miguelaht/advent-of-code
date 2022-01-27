import itertools
from functools import cache

def get_input(test=False) -> list[int]:
    if test:
        return [4,8]
    else:
        return [6,3]

def warm_up():
    scores = [0,0]
    players = get_input(test=False)
    rolls = 0
    winner = False

    pos = 0
    dice = [x for x in range(1,101)]
    while not winner:
        rolls += 3
        sl = dice[0:3]
        s = sum(sl)
        dice = dice[3:] + sl

        players[pos] += s
        if players[pos] > 10:
            players[pos] = (players[pos] - 1) % 10 + 1

        scores[pos] += players[pos]

        pos = 0 if pos == 1 else 1

        if scores[pos] >= 1000:
            winner = True
            break

    res = min(scores) * rolls
    print(res)

throws = [sum(x) for x in itertools.product([1, 2, 3], repeat=3)]

@cache
def dirac_dice(p1, s1, p2, s2, turn):
    if s1 >= 21:
        return (1,0)
    if s2 >= 21:
        return (0,1)

    pos = p1 if turn else p2
    new_pos = [(pos + throw - 1) % 10 +  1 for throw in throws]
    if turn:
        sub = (dirac_dice(new_p, s1 + new_p, p2, s2, False) for new_p in new_pos)
    else:
        sub = (dirac_dice(p1, s1, new_p, s2 + new_p, True) for new_p in new_pos)
    return sum(a for a, _ in sub), sum(b for _, b in sub)


def play():
    p1, p2 = get_input(test=False)
    print(max(dirac_dice(p1, 0, p2, 0, True)))

if __name__ == "__main__":
    warm_up()
    play()


