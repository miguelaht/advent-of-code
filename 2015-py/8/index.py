import ast
input = open('input.txt').readlines()

first = sum([len(line) - 1 - (len(ast.literal_eval(line))) for line in input])
print(first)

second = sum(2+s.count('"')+s.count('\\') for s in open('input.txt'))
print(second)
