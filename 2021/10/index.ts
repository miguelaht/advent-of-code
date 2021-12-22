import * as fs from 'fs'

const input: string = fs.readFileSync('input.txt', 'utf8')
const testInput: string = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`

const Tags: { [id: string]: string } = {
  '}': '{',
  ']': '[',
  '>': '<',
  ')': '(',
}

function solution1() {
  const lines: string[] = input.split('\n')
  let stack: string[] = []
  let count = new Map<string, number>()

  lines.map((line) => {
    for (const char of line) {
      if (Object.values(Tags).indexOf(char) !== -1) {
        stack.push(char)
      } else {
        if (stack[stack.length - 1] === Tags[char]) {
          stack.pop()
        } else {
          count.set(char, (count.get(char) ?? 0) + 1)
          break
        }
      }
    }
    stack = []
  })

  const res =
    ((count.get(')') ?? 0) * 3) +
    ((count.get(']') ?? 0) * 57) +
    ((count.get('}') ?? 0) * 1197) +
    ((count.get('>') ?? 0) * 25137)

  console.log(res)
}

solution1()

const Points: string[] = ['(', '[', '{', '<']

function solution2() {
  const lines: string[] = input.split('\n')
  let stack: string[] = []
  let count: number = 0
  let scores: number[] = []

  lines.map((line) => {
    for (const char of line) {
      if (Object.values(Tags).indexOf(char) !== -1) {
        stack.push(char)
      } else if (stack[stack.length - 1] === Tags[char]) {
        stack.pop()
      } else {
        // corrupted line
        count = 0
        stack = []
        break
      }
    }

    for (const char of stack.reverse()) {
      count *= 5
      count += (Points.indexOf(char) + 1)
    }
    if (count > 0)
      scores.push(count)

    count = 0
    stack = []
  })

  scores = scores.sort((a, b) => a - b)
  console.log(scores[(scores.length - 1) / 2])
}
solution2()
