def parse_bits(input):
    bits = []
    for c in input:
        b = bin(int(c, 16)).zfill(8)
        a = b.split('b')[1]
        if len(a) < 4:
            m = 4 - len(a)
            a = '0'*m + a

        bits.append(a)

    return ''.join(bits)

def get_input(test:bool = False):
    if test:
        test_input = '04005AC33890'
        return test_input

    with open('input.txt') as file:
        input = file.read()
        return input

def literal(bits: str) -> tuple[str, int, int]:
    '''
    returns (bits, version, sum)
    '''
    val = ''
    end = False
    ver = int(bits[0:3], 2)
    bits = bits[6:]

    while not end:
        n = bits[0:5]
        bits = bits[5:]
        val += n[1:]

        if n[0] == '0':
            end = True

    return bits, ver, int(val, 2)

def is_literal(pac: str) -> bool:
    return int(pac[3:6], 2) == 4

def operator_0(bits: str) -> tuple[str, int, list[int]]:
    '''
    returns (bits, version)
    '''
    versions = 0
    size = int(bits[:15], 2)
    bits = bits[15:]
    temp = bits[:size]
    bits = bits[size:]
    vals = []
    while temp:
        temp, v, n = solve(temp)
        vals.append(n)
        versions += v
        if len(temp) <= 6:
            break

    return bits, versions, vals

def operator_1(bits: str) -> tuple[str, int, list[int]]:
    '''
    returns (bits, version)
    '''
    q = int(bits[:11], 2)
    bits = bits[11:]
    versions = 0
    vals = []
    for _ in range(q):
        bits, v, n = solve(bits)
        vals.append(n)
        versions += v
        if len(bits) <= 6:
            break

    return bits, versions, vals

def operator(bits: str) -> tuple:
    import math
    if len(bits) <= 6:
        return '', 0
    versions = int(bits[:3], 2)
    type_id = int(bits[3:6], 2)
    size = bits[6]
    bits = bits[7:]
    vals = []

    if size == '0':
        bits, v, vals = operator_0(bits)
        versions += v
    else:
        bits, v, vals = operator_1(bits)
        versions += v

    n = 0
    if type_id == 0:
        n = sum(vals)
    elif type_id == 1:
        n = math.prod(vals)
    elif type_id == 2:
        n = min(vals)
    elif type_id == 3:
        n = max(vals)
    elif type_id == 5:
        n = 1 if vals[0] > vals[1] else 0
    elif type_id == 6:
        n = 1 if vals[0] < vals[1] else 0
    elif type_id == 7:
        n = 1 if vals[0] == vals[1] else 0

    return bits, versions, n

def solve(bits: str) -> tuple[str, int, int]:
    if len(bits) <= 6:
        return '', 0, 0

    v = 0
    n = 0
    if is_literal(bits):
        print('literal')
        bits, v, n = literal(bits)
    else: # operator
        print('operator')
        bits, v, n = operator(bits)

    return bits, v, n

def solution():
        input = get_input(test=False)
        bits = parse_bits(input)
        versions = 0

        end = False
        n = 0
        while not end:
            if len(bits) <= 6:
                break
            bits, v, n = solve(bits)
            versions += v
            if v == 0 or not len(bits) <= 6:
                end = True

        print(versions, n)

def p_1():
    solution()

if __name__ == "__main__":
    p_1()
