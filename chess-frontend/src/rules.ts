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

class PawnsBeatRules {
  // TODO to rewrite, it dosen't check diagonally
  private checkIfPawnOnDiagonalUp(
    startX: number,
    startY: number,
    endX: number,
    color: Color,
    boardState: { [key: string]: [Color, PawnType] }
  ) {
    let j = startY + 1
    let pawnsFind = 0
    for (let i = startX + 1; i < endX; i++) {
      console.log(`${i}_${j}`)
      if (boardState[`${i}_${j}`][0] === color) {
        pawnsFind++
      }
      j++
    }
    return pawnsFind === 1
  }
  private checkIfPawnOnDiagonalDown(
    startX: number,
    startY: number,
    endX: number,
    color: Color,
    boardState: { [key: string]: [Color, PawnType] }
  ) {
    let j = startY - 1
    let pawnsFind = 0
    for (let i = startX - 1; i > endX; i--) {
      console.log(`${i}_${j}`)
      if (boardState[`${i}_${j}`][0] === color) {
        pawnsFind++
      }
      j--
    }
    return  pawnsFind === 1
  }
  [PawnType.PawnDark](
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    y: number,
    boardState: { [key: string]: [Color, PawnType] }
  ) {
    return (
      boardState[`${endX}_${endY}`][0] === Color.Empty &&
      boardState[`${startX + 1}_${y}`][0] === Color.Light &&
      Math.abs(startY - endY) === 2 &&
      startX - endX === -2
    )
  }

  [PawnType.PawnLight](
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    y: number,
    boardState: { [key: string]: [Color, PawnType] }
  ) {
    return (
      boardState[`${endX}_${endY}`][0] === Color.Empty &&
      boardState[`${startX - 1}_${y}`][0] === Color.Dark &&
      Math.abs(startY - endY) === 2 &&
      startX - endX === 2
    )
  }

  [PawnType.Dame](
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    y: number,
    boardState: { [key: string]: [Color, PawnType] }
  ) {
    const opposedColor = boardState[`${startX}_${startY}`][0] === Color.Dark ? Color.Light : Color.Dark
    return (
      boardState[`${endX}_${endY}`][0] === Color.Empty &&
      startX !== endX &&
      startY !== endY &&
      Math.abs(startY - endY) == Math.abs(startX - endX) &&
      (this.checkIfPawnOnDiagonalUp(
        startX,
        startY,
        endX,
        opposedColor,
        boardState
      ) ||
        this.checkIfPawnOnDiagonalDown(
          startX,
          startY,
          endX,
          opposedColor,
          boardState
        ))
    )
  }

  [PawnType.Empty]() {
    return false
  }
}

export class CheckersRules {
  currentTurnColor = Color.Light
  nextTurnColor = Color.Dark
  pawnsMoveRules: PawnsMoveRules = new PawnsMoveRules()
  pawnsBeatRules: PawnsBeatRules = new PawnsBeatRules()

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
    const pawnType: PawnType = boardState[`${startX}_${startY}`][1]
    const y = startY > endY ? startY - 1 : startY + 1

    return this.pawnsBeatRules[pawnType](startX, startY, endX, endY, y, boardState)
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
