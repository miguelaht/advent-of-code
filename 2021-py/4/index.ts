import * as fs from 'fs'

const input: string = fs.readFileSync('input.txt', 'utf8')
const testInput: string = `7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7`

// check if the updated board is a winner
// gets the index of columns marked as drawned
function findWinner(input: string[]): boolean {
  // [ '1 2 3', '4 5 6']
  let board = input.map((r) => r.split(' '))
  let columns: string[] = []

  for (let i = 0; i < board.length; i++) {
    if (!input[i].match(new RegExp('\[0-9]'))) {
      return true
    }

    for (let j = 0; j < board[i].length; j++) {
      if (columns[j]) {
        columns[j] = columns[j].concat(board[i][j])
      } else {
        columns[j] = board[i][j]
      }
    }
  }


  for (let i = 0; i < columns.length; i++) {
    if (!columns[i].match(new RegExp('\[0-9]'))) {
      return true
    }
  }

  return false
}

function first(): void {
  const data = input.split('\n\n')
  const numbers = data[0].split(',')
  let boards = data
    .slice(1)
    .map((b) =>
      b.split('\n')
        .map((r) => r.trim().replace(/\s{2,}/g, ' '))) // '22 13 17 11  0'

        console.log(data)
  const rowLen = boards[0][0][1].length
  let index = 0
  let winner: boolean = false
  let winnerBoard: string[] = []
  for (const n of numbers) {
    for (let i = 0; i < boards.length; i++) {
      for (let j = 0; j < boards[i].length; j++) {
        boards[i][j] = boards[i][j].replace(new RegExp(`\\b${n}\\b`), 'M')

        if (index >= rowLen) {
          let indexes: number[] = [];
          let x: number = 0
          while (~(x = boards[i][j].indexOf('M', x + 'M'.length))) indexes.push(i);
          if (indexes.length > 0) {
            winner = findWinner(boards[i])
            if (winner) {
              winnerBoard = boards[i]
              break
            }
          }
        }
      }
      if (winner) {
        break
      }
    }
    if (winner) {
      break
    }
    index++
  }

  let sum = winnerBoard.map((r) => r.split(' ').filter((n) => {
    if (Number(n))
      return n

    return 0
  }))
    .filter((r) => {
      if (r.length > 0)
        return r
    }).map((r) => r.map((n) => Number(n)).reduce((p: number, c: number) => Number(p) + Number(c)))
    .reduce((p, c) => p + c)

  console.log('FIRST', sum * Number(numbers[index]), numbers[index])
}
first()

function second(): void {
  const data = input.split('\n\n')
  const numbers = data[0].split(',')
  let boards = data
    .slice(1)
    .map((b) =>
      b.split('\n')
        .map((r) => r.trim().replace(/\s{2,}/g, ' '))) // '22 13 17 11  0'

  const rowLen = boards[0][0][1].length
  let index = 0
  let winner: boolean = false
  let winnerBoard: string[] = []
  let winners = 0
  for (const n of numbers) {
    for (let i = 0; i < boards.length; i++) {
      for (let j = 0; j < boards[i].length; j++) {
        boards[i][j] = boards[i][j].replace(new RegExp(`\\b${n}\\b`), 'M')

        if (index >= rowLen) {
          let indexes: number[] = [];
          let x: number = 0
          while (~(x = boards[i][j].indexOf('M', x + 'M'.length))) indexes.push(i);
          if (indexes.length > 0) {
            winner = findWinner(boards[i])
            if (winner) {
              winners++
              if (winners >= boards.length) {
                winnerBoard = boards[i]
                break
              }
              boards[i] = []
            }
          }
        }
      }
      if (winners === boards.length) {
        break
      }
    }
    if (winners === boards.length) {
      break
    }
    index++
  }

  console.log(winnerBoard, numbers[index])
  let sum = winnerBoard.map((r) => r.split(' ').filter((n) => {
    if (Number(n))
      return n

    return 0
  }))
    .filter((r) => {
      if (r.length > 0)
        return r
    }).map((r) => r.map((n) => Number(n)).reduce((p: number, c: number) => Number(p) + Number(c)))
    .reduce((p, c) => p + c)
  console.log('SECOND', sum * Number(numbers[index]))
}
second()
