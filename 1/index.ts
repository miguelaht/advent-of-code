import * as fs from 'fs'

const content: string = fs.readFileSync('input.txt', 'utf8')

function first(): void {
  // const measures = ['199','200','208','210','200','207','240','269','260','263']
  const measures: string[] = content.split('\n')
  let count: number = 0;
  for (let i: number = 1; i < measures.length; i++) {
    count += Number(measures[i]) >= Number(measures[i - 1]) ? 1 : 0
  }
  console.log('First', count)
}
first()

function second(): void {
  // const measures = ['199', '200', '208', '210', '200', '207', '240', '269', '260', '263']
  const measures: string[] = content.split('\n')
  let pos: number = 0;
  let len: number = measures.length - 1
  let count: number = 0;
  let prev: number = 0
  while (len - pos >= 2) {
    let sum: number = measures
      .slice(pos, pos + 3)
      .map((val) => Number(val))
      .reduce((previousValue: number, currentValue: number): number => previousValue + currentValue)

    if (prev != 0)
      count += sum > prev ? 1 : 0

    prev = sum
    pos++
  }

  console.log('Second', count)
}
second()
