import { Color } from './types'

class CheckersRules {
  canMove(
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    boardState: { [key: string]: string }
  ): boolean {
    const color = boardState[`${startX}_${startY}`]
    if (color === Color.Dark) {
      return startX - endX === -1 && Math.abs(startY - endY) === 1
    }
    if (color == Color.Light) {
      return startX - endX === 1 && Math.abs(startY - endY) === 1
    }
    return false
  }

  canBeat(
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    boardState: { [key: string]: string }
  ): boolean {
    const color = boardState[`${startX}_${startY}`]

    if (startX === endX && startY === endY) return false
    if (color === Color.Dark) {
      const y = startY > endY ? startY - 1 : startY + 1
      if (
        boardState[`${startX + 1}_${y}`] === Color.Light &&
        boardState[`${endX}_${endY}`] === '' &&
        startX < endX
      ) {
        return true
      }
    }
    if (color == Color.Light) {
      const y = startY > endY ? startY - 1 : startY + 1
      if (
        boardState[`${startX - 1}_${y}`] === Color.Dark &&
        boardState[`${endX}_${endY}`] === '' &&
        startX > endX
      ) {
        return true
      }
    }
    return false
  }
}

export const checkersRules = new CheckersRules()
