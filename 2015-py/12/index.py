import ast

input = open('input.txt')
data: dict[str, list | dict] = ast.literal_eval(input.read())

def process_object(obj: dict):
    counter = 0
    if 'red' in list(obj.values()):
        return 0

    for k in obj:
        counter += process(obj[k])

    return counter

def process_list(arr: list):
    counter = 0
    for v in arr:
        counter += process(v)

    return counter

def process(data):
    counter = 0
    t = type(data)
    if t is dict:
        counter += process_object(data)
    elif t is list:
        counter += process_list(data)
    elif t is int:
        counter += data

    return counter


assert process([[[3]]]) == 3
assert process([1,{"c":"red","b":2},3]) == 4
assert process({"d":"red","e":[1,2,3,4],"f":5}) == 0
assert process([1,"red",5]) == 6

print(process(data))
