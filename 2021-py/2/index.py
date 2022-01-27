from functools import reduce
from operator import mul

input = open('input.txt').readlines()

pos = {'x':0, 'y':0}
for c in input:
    d, n = c.split(' ')

    if d == 'forward':
      pos['x'] += int(n)
    if d == 'up':
      pos['y'] -= int(n)
    if d == 'down':
      pos['y'] += int(n)

print(f'Solution 1 -> {reduce(mul, pos.values(), 1)}')

pos = {'aim': 0, 'x':0, 'y':0}
for c in input:
    d, n = c.split(' ')

    if d == 'forward':
      pos['x'] += int(n)
      pos['y'] += int(n) * pos['aim']
    if d == 'up':
      pos['aim'] -= int(n)
    if d == 'down':
      pos['aim'] += int(n)

pos['aim'] = 1
print(f'Solution 2 -> {reduce(mul, pos.values(), 1)}')
