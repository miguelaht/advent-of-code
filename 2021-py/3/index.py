input = open('input.txt').read().splitlines()

gamma = epsilon = ''

for i in range(len(input[0])):
    zero = one = 0
    for j in range(len(input)):
        if input[j][i] == '0':
            zero += 1
        else:
            one += 1

    if zero > one:
        gamma = ''.join((gamma, '0'))
        epsilon = ''.join((epsilon, '1'))
    else:
        gamma = ''.join((gamma, '1'))
        epsilon = ''.join((epsilon, '0'))

gamma = int(gamma, 2)
epsilon = int(epsilon, 2)
print(f'Solution 1 -> {gamma * epsilon} ')

def oxigen():
    data = input
    pos = 0
    while len(data) > 1:
        zero = []
        one = []
        for n in data:
            if n[pos] == '1':
                one.append(n)
            else:
                zero.append(n)

        data = one if len(one) >= len(zero) else zero
        pos += 1

    return data[0]

def co2():
    data = input
    pos = 0
    while len(data) > 1:
        zero = []
        one = []
        for n in data:
            if n[pos] == '1':
                one.append(n)
            else:
                zero.append(n)

        data = zero if len(one) >= len(zero) else one
        pos += 1

    return data[0]

ox = int(oxigen(), 2)
co = int(co2(), 2)
print(f'Solution 2 -> {ox * co} ')
