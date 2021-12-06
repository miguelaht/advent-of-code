import * as fs from 'fs'

const input: string = fs.readFileSync('input.txt', 'utf8')
const test: string = '3,4,3,1,2'

function first(): void {
  const initialState: number[] = input.split(',').map((n) => Number(n))
  const numberOfDays: number = 80

  let uniqueNumbers = new Map<number, number>()
  initialState
    .map((n) => Number(n))
    .map((n) => { uniqueNumbers.set(n, (uniqueNumbers.get(n) ?? 0) + 1) })
  let fishes = [0, 0, 0, 0, 0, 0, 0, 0, 0]
  uniqueNumbers.forEach((v, k) => {
    fishes[k] = v
  })

  let currentDay: number = 1
  while (currentDay <= numberOfDays) {
    let newFish = fishes[0]
    for (let i = 1; i < fishes.length; i++) {
      fishes[i - 1] = fishes[i]
    }
    fishes[6] += newFish
    fishes[8] = newFish
    currentDay++
  }
  const total = fishes.reduce((p, c) => p + c)
  console.log(total)
}
first()

function second(): void {
  const initialState: number[] = input.split(',').map((n) => Number(n))
  const numberOfDays: number = 256

  let uniqueNumbers = new Map<number, number>()
  initialState
    .map((n) => Number(n))
    .map((n) => { uniqueNumbers.set(n, (uniqueNumbers.get(n) ?? 0) + 1) })
  let fishes = [0, 0, 0, 0, 0, 0, 0, 0, 0]
  uniqueNumbers.forEach((v, k) => {
    fishes[k] = v
  })

  let currentDay: number = 1
  while (currentDay <= numberOfDays) {
    let newFish = fishes[0]
    for (let i = 1; i < fishes.length; i++) {
      fishes[i - 1] = fishes[i]
    }
    fishes[6] += newFish
    fishes[8] = newFish
    currentDay++
  }
  const total = fishes.reduce((p, c) => p + c)
  console.log(total)
}
second()
