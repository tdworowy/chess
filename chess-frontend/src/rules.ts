import { Color, PawnType } from './types'

class PawnsMoveRules {
  [PawnType.PawnDark](startX: number, startY: number, endX: number, endY: number) {
    return startX - endX === -1 && Math.abs(startY - endY) === 1
  }

  [PawnType.PawnLight](startX: number, startY: number, endX: number, endY: number) {
    return startX - endX === 1 && Math.abs(startY - endY) === 1
  }

  [PawnType.Dame](startX: number, startY: number, endX: number, endY: number) {
    return Math.abs(startX - endX) === 1 && Math.abs(startY - endY) === 1
  }

  [PawnType.Empty]() {
    return false
  }
}

export class CheckersRules {
  currentTurnColor = Color.Light
  nextTurnColor = Color.Dark
  pawnsMoveRules: PawnsMoveRules = new PawnsMoveRules()

  canMove(
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    boardState: { [key: string]: [Color, PawnType] }
  ): boolean {
    const color = boardState[`${startX}_${startY}`][0]
    const pawnType: PawnType = boardState[`${startX}_${startY}`][1]
    if (color !== this.currentTurnColor) return false
    return this.pawnsMoveRules[pawnType](startX, startY, endX, endY)
  }

  // TODO hadnle dame
  canBeat(
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    boardState: { [key: string]: [Color, PawnType] }
  ): boolean {
    const color = boardState[`${startX}_${startY}`][0]
    if (color !== this.currentTurnColor) return false

    const y = startY > endY ? startY - 1 : startY + 1
    if (color === Color.Dark) {
      if (
        boardState[`${startX + 1}_${y}`][0] === Color.Light &&
        boardState[`${endX}_${endY}`][0] === '' &&
        Math.abs(startY - endY) === 2 &&
        startX - endX === -2
      ) {
        return true
      }
    }
    if (color == Color.Light) {
      if (
        boardState[`${startX - 1}_${y}`][0] === Color.Dark &&
        boardState[`${endX}_${endY}`][0] === '' &&
        Math.abs(startY - endY) === 2 &&
        startX - endX === 2
      ) {
        return true
      }
    }
    return false
  }

  nextTurn() {
    const temp = this.currentTurnColor
    this.currentTurnColor = this.nextTurnColor
    this.nextTurnColor = temp
  }
}

export const getNewCheckersRules = () => {
  return new CheckersRules()
}

export const checkersRules = new CheckersRules()
