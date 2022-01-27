input = open('input.txt').readlines()

count = 0
for i in range(1, len(input)):
    count += 1 if input[int(i)] >= input[int(i) - 1] else 0

print(f'Solution 1 -> {count}') # 1559

groups: list[list[int]] = []
for i in range(0, len(input)):
    if len(input[i:i+3]) < 3:
        break
    groups.append(list(map( lambda n: int(n), input[i:i+3])))

count = 0
for i in range(1, len(groups)):
    count += 1 if sum(groups[i]) > sum(groups[i-1]) else 0

print(f'Solution 2 -> {count}') # 1600
