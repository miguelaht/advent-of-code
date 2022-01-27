input = open('input.txt').read()
test = '''NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C'''

def parse_pairs(pairs: list[list[str]]):
    p = {}
    for pair in pairs:
        p[pair[0]] = pair[1]

    return p


def solution():
    template, p = input.split('\n\n')
    polymer = template
    pairs = list(map(lambda p: p.split(' -> '), p.split('\n')))
    pairs = parse_pairs(pairs)

    for _ in range(10):
        new_polymer = polymer
        for i in range(len(polymer)):
            p = polymer[i:i+2]
            if len(p) == 2 and p in pairs.keys():
                pos = new_polymer.find(p)
                new_polymer = ''.join((new_polymer[:pos+1], pairs[p].lower(), new_polymer[pos+1:]))

        polymer = new_polymer.upper()

    count: dict[str,int] = {}
    for i in range(len(polymer)):
        c = polymer[i]
        if c in count.keys():
            count[c] += 1
        else:
            count[c] = 1

    vals = list(count.values())
    vals.sort()
    print(vals[-1] - vals[0])

def solution_2():
    template, rules = input.split('\n\n')
    rules = list(map(lambda p: p.split(' -> '), rules.split('\n')))
    rules = parse_pairs(rules)

    polymer = {}
    for i in range(len(template) - 1):
        key = template[i:i+2]
        if key in polymer:
            polymer[key] += 1
        else:
            polymer[key] = 1

    for _ in range(40):
        pol = {}
        for k in polymer:
            key = k[0] + rules[k]
            if key in pol:
                pol[key] += polymer[k]
            else:
                pol[key] = polymer[k]
            key = rules[k] + k[1]
            if key in pol:
                pol[key] += polymer[k]
            else:
                pol[key] = polymer[k]

        polymer = pol

    count = {}
    for key in polymer:
        if key[0] in count:
            count[key[0]] += polymer[key]
        else:
            count[key[0]] = polymer[key]

    high = max(count.keys(), key=(lambda k: count[k]))
    low = min(count.keys(), key=(lambda k: count[k]))
    print(count[high]-count[low])


if __name__ == "__main__":
    solution_2()
