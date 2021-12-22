import * as fs from 'fs'

const input: string = fs.readFileSync('input.txt', 'utf8')
const testInput: string = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`

interface Line {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
}

function parseLineToPoints(line: string): Line {
  const [[x1 = 0, y1 = 0], [x2 = 0, y2 = 0]] = line.split(' -> ').map((l) => l.split(',').map((n) => Number(n)))
  return { x1, y1, x2, y2 }
}

function first(): number {
  const data: string[] = input.split('\n')
  let diagram: { [id: string]: number } = {}

  for (const l of data) {
    const { x1, y1, x2, y2 } = parseLineToPoints(l)


    if (x1 === x2) {
      let hy = Math.max(y1, y2) // max y
      let ly = Math.min(y1, y2) // min y
      ly = ly === 0 ? 1 : ly

      for (let i = ly; i < hy + 1; i++) {
        const key = `${x1},${i}`
        if (diagram[key]) diagram[key]++; else diagram[key] = 1
      }
    }

    if (y1 === y2) {
      let hx = Math.max(x1, x2) // max x
      let lx = Math.min(x1, x2) // min x

      for (let i = lx; i < hx + 1; i++) {
        const key = `${i},${y1}`
        if (diagram[key]) diagram[key]++; else diagram[key] = 1
      }
    }

  }
  let counter = Object.values(diagram).filter((v) => {
    if (v > 1)
      return v
  }).length

  return counter
}
console.log(first())

function second(): number {
  const data: string[] = input.split('\n')
  let diagram: { [id: string]: number } = {}

  for (const l of data) {
    const { x1, y1, x2, y2 } = parseLineToPoints(l)

    if (x1 === x2) {
      let hy = Math.max(y1, y2) // max y
      let ly = Math.min(y1, y2) // min y
      ly = ly === 0 ? 1 : ly

      for (let i = ly; i < hy + 1; i++) {
        const key = `${x1},${i}`
        if (diagram[key]) diagram[key]++; else diagram[key] = 1
      }
    } else if (y1 === y2) {
      let hx = Math.max(x1, x2) // max x
      let lx = Math.min(x1, x2) // min x

      for (let i = lx; i < hx + 1; i++) {
        const key = `${i},${y1}`
        if (diagram[key]) diagram[key]++; else diagram[key] = 1
      }
    } else {
      const xMultiplier = x1 > x2 ? -1 : 1
      const yMultiplier = y1 > y2 ? -1 : 1

      const dif = Math.abs(x2 - x1)
      for (let i = 0; i < dif + 1; i++) {
        const x = x1 + i * xMultiplier
        const y = y1 + i * yMultiplier
        const key = `${x},${y}`
        if (diagram[key]) diagram[key]++; else diagram[key] = 1
      }
    }
  }

  let counter = Object.values(diagram).filter((v) => {
    if (v > 1)
      return v
  }).length

  return counter
}
console.log('second', second())
