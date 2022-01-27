import * as fs from 'fs'

const input = fs.readFileSync('input.txt', 'utf8')
const testInput = '16,1,2,0,4,2,7,1,2,14'

function first(): void {
  const crabs = new Map<number, number>()
  const numbers = input.split(',').map((n) => Number(n)).sort((a, b) => a - b)

  const median = (): number => {
    const mid = Math.floor(numbers.length / 2)
    if (numbers.length % 2)
      return numbers[mid]

    return (numbers[mid - 1] + numbers[mid]) / 2
  }

  let fuel = 0
  input.split(',').map((n) => {
    crabs.set(Number(n), (crabs.get(Number(n)) ?? 0) + 1)
  })

  crabs.forEach((v, k) => {
    const cost = Math.abs((k * v) - (median() * v))
    fuel += cost
  })

  console.log(fuel)
}

first()

function second(): void {
  const numbers = input.split(',').map((n) => Number(n))

  const avg = Math.floor(numbers.reduce((a, b) => a + b) / numbers.length)

  const gauss = (n: number): number => (n * (1 + n)) / 2

  let fuel = 0
  numbers.map((b) => {
    fuel += gauss(Math.abs(avg - b))
  })

  console.log(fuel)
}

second()
