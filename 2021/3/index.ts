import * as fs from 'fs'
const input: string = fs.readFileSync('input.txt', 'utf8')

function first(): void {
  let gamma: string = '', epsilon: string = ''

  //const content: string[][] = `00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010`.split('\n').map((n) => n.split(''))
  const content: string[][] = input.split('\n').map((n) => n.split(''))
  const cols = content[0].length
  const rows = content.length

  for (let i = 0; i < cols; i++) {
    let ones = 0;
    let zeros = 0

    for (let j = 0; j < rows; j++) {
      if (content[j][i] === '0') {
        zeros += 1
      } else {
        ones += 1
      }
    }

    if (zeros > ones) {
      gamma = gamma.concat('0')
      epsilon = epsilon.concat('1')
    } else {
      gamma = gamma.concat('1')
      epsilon = epsilon.concat('0')
    }
  }
  console.log(gamma, epsilon)
  console.log(parseInt(gamma, 2) * parseInt(epsilon, 2))
}
first()

function second(): void {
  //const content: string[] = `00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010`.split('\n')
  const content: string[] = input.split('\n')

  console.log(parseInt(oxigen(content), 2) * parseInt(co2(content), 2))
}

function oxigen(report: string[]): string {
  let data: string[] = report
  let pos = 0;
  while (data.length > 1) {
    let zeros: string[] = []
    let ones: string[] = []

    data.map((n) => {
      if (n[pos] === '1') {
        ones.push(n)
      } else {
        zeros.push(n)
      }
    })
    data = ones.length >= zeros.length ? ones : zeros
    pos++
  }

  console.log('O',data)
  return data[0]
}
function co2(report: string[]): string {
  let data: string[] = report
  let pos = 0;
  while (data.length > 1) {
    let zeros: string[] = []
    let ones: string[] = []

    data.map((n) => {
      if (n[pos] === '1') {
        ones.push(n)
      } else {
        zeros.push(n)
      }
    })

    data = ones.length >= zeros.length ? zeros : ones
    pos++
  }
  console.log('C',data)
  return data[0]
}
second()
