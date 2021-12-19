def get_input(test=False) -> list[str]:
    out = ''
    if test:
        out = '[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]'
    else:
        with open('input.txt') as file:
            out = file.read()

    return out.split('\n')

def xplode_number(at: int, number: str):
    start = end = at+1

    for i in range(start, len(number)):
        if number[i] == ']':
            end = i
            break

    a, b = number[start: end].split(',')

    # find rightmost regular number
    t = number[end:]
    rightmost = []
    n = ''
    for i, c in enumerate(t):
        if c.isdigit():
            if len(rightmost) > 0:
                if i - rightmost[-1] > 1:
                    break

            rightmost.append(i)
            n += c

    if n != '':
        b = int(n) + int(b) # rightmost number
        number = number[:end + rightmost[0]] + str(b) + number[end + 1 + rightmost[-1]:] # add right number

    number = number[:start-1] + '0' + number[end+1:] # replace xploded list with 0

    # find leftmost regulat number
    t = number[:start-1][::-1]
    leftmost = []
    n = ''
    for i, c in enumerate(t):
        if c.isdigit():
            if len(leftmost) > 0:
                if abs((len(t) - i) - leftmost[-1]) > 1:
                    break

            leftmost.append(len(t) - i)
            n += c

    n = n[::-1]
    leftmost = leftmost[::-1]
    if n != '':
        a = int(n) + int(a) # leftmost number
        number = number[:leftmost[0]-1] + str(a) + number[leftmost[-1]:] # add left number

    return number

def split_number(number):
    import math

    digit = ''
    split_at = -1
    for index, char in enumerate(number):
        # split
        if char.isdigit():
            digit += char
        else:
            digit = ''

        if len(digit) == 2:
            split_at = index - 1
            break

    digit = int(digit) / 2
    new_digit = [math.floor(digit), math.ceil(digit)]

    number = number[:split_at] + f'[{new_digit[0]},{new_digit[1]}]' + number[split_at+2:]

    return number

def parse_number(inp: str):
    xplode_at = -1
    split_at = -1
    level = 0

    while xplode_at != 0 or split_at != 0:
        xplode_at = 0
        split_at = 0
        level = 0
        split = ''

        for index, char in enumerate(inp):
            # explode
            if level < 5:
                if char == '[':
                    level += 1
                if char == ']':
                    level -= 1
            else:
                level += 1

            # split
            if char.isdigit():
                split += char
            else:
                split = ''

            # check por explode
            if level == 5:
                xplode_at = index
                break

            # check for split
            if len(split) == 2 and split_at == 0:
                split_at = index - 1

        if xplode_at != 0:
            inp = xplode_number(xplode_at, inp)
            continue

        if split_at != 0:
            inp = split_number(inp)

    return inp

def add_numbers(n1, n2):
    res = ''
    res = f'[{n1},{n2}]'
    return res

def calculate(n):
    if type(n[0]) is list and type(n[1]) is list:
        l = calculate(n[0]) * 3
        r = calculate(n[1]) * 2
        return l + r

    if type(n[0]) is int and type(n[1]) is list:
        l = n[0]
        r = calculate(n[1])
        return l + r

    if type(n[1]) is int and type(n[0]) is list:
        r = n[1]
        l = calculate(n[0])
        return l + r

    return (n[0] * 3) + (n[1] * 2)


def get_magnitude(number: str) -> int:
    import ast
    n = ast.literal_eval(number)
    l = calculate(n[0]) * 3
    r = calculate(n[1]) * 2

    m = l + r

    return m

def solve(numbers) -> int:
    res = numbers.pop(0)
    if numbers and numbers[-1] == '':
        numbers.pop()

    while numbers:
        res = parse_number(res)
        n = numbers.pop(0)
        add = add_numbers(res, n)
        res = add

    res = parse_number(res)
    m = get_magnitude(res)

    return m

def solution_1():
    numbers: list[str] = get_input(False)
    m = solve(numbers)
    print(m)

def solution_2():
    numbers: list[str] = get_input(False)
    largest = 0
    seen = set()
    for n in numbers:
        for x in numbers:
            if (n,x) not in seen and (x,n) not in seen:
                seen.add((n,x))
                seen.add((x,n))
                if x != n:
                    m = solve([n, x])
                    m2 = solve([x, n])

                    l = max(m, m2)
                    if l > largest:
                        largest = l

    print(largest)

if __name__ == "__main__":
    solution_1()
    solution_2()
