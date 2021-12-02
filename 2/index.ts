import * as fs from 'fs'

const content: string = fs.readFileSync('input.txt', 'utf8')

function first(): void {
  // const commands: string[] = ['forward 5', 'down 5', 'forward 8', 'up 3', 'down 8', 'forward 2']
  const commands: string[] = content.split('\n')
  let pos: { x: number, y: number } = { x: 0, y: 0 }
  for (const c of commands) {
    const command = c.split(' ')

    if (command[0] === 'forward') {
      pos.x = pos.x + Number(command[1])
    }
    if (command[0] === 'up') {
      pos.y = pos.y - Number(command[1])
    }
    if (command[0] === 'down') {
      pos.y = pos.y + Number(command[1])
    }
  }

  console.log(pos, pos.x * pos.y)
}

first()

function second(): void {
  // const commands: string[] = ['forward 5', 'down 5', 'forward 8', 'up 3', 'down 8', 'forward 2']
  const commands: string[] = content.split('\n')
  let pos: { x: number, y: number, aim: number } = { x: 0, y: 0, aim: 0 }
  for (const c of commands) {
    const command = c.split(' ')

    if (command[0] === 'forward') {
      pos.x = pos.x + Number(command[1])
      pos.y += Number(command[1]) * pos.aim
    }
    if (command[0] === 'up') {
      pos.aim = pos.aim - Number(command[1])
    }
    if (command[0] === 'down') {
      pos.aim = pos.aim + Number(command[1])
    }
  }
  console.log(pos, pos.x * pos.y)
}

second()
