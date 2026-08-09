import { describe, expect, it, beforeEach } from 'vitest'
import { getNewCheckersRules, CheckersRules } from '../rules'
import { Color, PawnType } from '../types'

describe('CheckersRules.canAnyBeat tests', () => {
  let checkersRules: CheckersRules
  let board: { [key: string]: [Color, PawnType] }

  beforeEach(() => {
    checkersRules = getNewCheckersRules()
    board = {}
    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        board[`${i}_${j}`] = [Color.Empty, PawnType.Empty]
      }
    }
  })

  it('should return true if a pawn can beat', () => {
    board['5_2'] = [Color.White, PawnType.PawnWhite]
    board['4_3'] = [Color.Black, PawnType.PawnBlack]
    // White can beat Black: (5,2) -> (3,4)
    expect(checkersRules.canAnyBeat(5, 2, board)).toBe(true)
  })

  it('should return false if a pawn cannot beat', () => {
    board['5_2'] = [Color.White, PawnType.PawnWhite]
    board['4_3'] = [Color.White, PawnType.PawnWhite] // Same color
    expect(checkersRules.canAnyBeat(5, 2, board)).toBe(false)
  })

  it('should return true if a dame can beat', () => {
    board['6_3'] = [Color.White, PawnType.Dame]
    board['4_5'] = [Color.Black, PawnType.PawnBlack]
    // White dame can beat Black: (6,3) -> (3,6) or (2,7) or (1,8)
    expect(checkersRules.canAnyBeat(6, 3, board)).toBe(true)
  })

  it('should handle multi-beat scenario correctly', () => {
    board['7_2'] = [Color.White, PawnType.PawnWhite]
    board['6_3'] = [Color.Black, PawnType.PawnBlack]
    board['4_5'] = [Color.Black, PawnType.PawnBlack]

    // First beat: (7,2) -> (5,4)
    expect(checkersRules.canAnyBeat(7, 2, board)).toBe(true)

    // After first beat:
    const boardAfterFirstBeat = { ...board }
    boardAfterFirstBeat['7_2'] = [Color.Empty, PawnType.Empty]
    boardAfterFirstBeat['6_3'] = [Color.Empty, PawnType.Empty]
    boardAfterFirstBeat['5_4'] = [Color.White, PawnType.PawnWhite]

    // Can beat again: (5,4) -> (3,6)
    expect(checkersRules.canAnyBeat(5, 4, boardAfterFirstBeat)).toBe(true)
  })

  it('should return false for canAnyBeat if it is not the pieces turn', () => {
    board['5_2'] = [Color.White, PawnType.PawnWhite]
    board['4_3'] = [Color.Black, PawnType.PawnBlack]

    // Switch to Black's turn
    checkersRules.nextTurn()

    // White pawn at (5,2) can beat Black at (4,3) BUT it is not White's turn
    expect(checkersRules.canAnyBeat(5, 2, board)).toBe(false)
  })
})
