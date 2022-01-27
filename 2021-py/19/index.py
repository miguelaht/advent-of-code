def parse_input(input: str) -> dict[int, tuple]:
    scanners = input.split('\n\n')
    report = {}

    for i, s in enumerate(scanners):
        l = list(map(lambda l: tuple(map(lambda n: int(n), l.split(','))), s.split('\n')[1:]))
        report[i] = l

    return report

def compare(b1, b2):
    rotate = [(1, 1, 1), (-1, 1, 1), (-1, -1, 1), (-1, -1, -1), (1, -1, -1), (1, 1, -1), (1, -1, 1), (-1, 1, -1)]
    pass


def solve():
    input = open('input.txt').read()
    scanners = parse_input(input)
    scanner_positions: dict[int, tuple[int, int]] = dict()

    for s1, b1 in scanners.items():
        for s2, b2 in scanners.items():
            pass

def test_parsing():
    input = open('test.txt').read()
    out = parse_input(input)
    assert 5 == len(list(out.keys()))

if __name__ == "__main__":
    import sys
    arg = sys.argv[1] if len(sys.argv) > 1 else ''
    if arg == 'test':
        test_parsing()
    else:
        solve()



