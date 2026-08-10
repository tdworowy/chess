import { describe, expect, it, beforeEach } from 'vitest'
import { getNewCheckersRules, CheckersRules } from '../../rules'
import { Color, PawnType } from '@/types'

describe('Infinite moves bug reproduction', () => {
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

  it('should not allow a normal move if a multi-beat is available after a beat', () => {
    // Setup: White pawn at 6_3, Black pawns at 5_4 and 3_6.
    // White beats 5_4 and lands on 4_5.
    // From 4_5, White can beat 3_6 and land on 2_7.
    // The bug is likely that at 4_5, White could also move to 3_4 (normal move) if not restricted.

    board['6_3'] = [Color.White, PawnType.PawnWhite]
    board['5_4'] = [Color.Black, PawnType.PawnBlack]
    board['3_6'] = [Color.Black, PawnType.PawnBlack]
    board['8_1'] = [Color.White, PawnType.PawnWhite] // Another white piece

    expect(checkersRules.currentTurnColor).toBe(Color.White)

    // 1. White beats 5_4: 6_3 -> 4_5
    expect(checkersRules.canBeat(6, 3, 4, 5, board)).toBe(true)

    // Simulate the beat in the board state
    board['5_4'] = [Color.Empty, PawnType.Empty]
    board['4_5'] = [Color.White, PawnType.PawnWhite]
    board['6_3'] = [Color.Empty, PawnType.Empty]

    // In Square.vue logic:
    // if (canBeat && checkersRules.canAnyBeat(endX, endY, boardState)) { return; }
    // This means nextTurn() is NOT called.

    expect(checkersRules.canAnyBeat(4, 5, board)).toBe(true)
    // Manually set mustMovePiece as Square.vue would do
    checkersRules.mustMovePiece = '4_5'

    // Turn is still White because nextTurn() wasn't called.
    expect(checkersRules.currentTurnColor).toBe(Color.White)

    // Now, can White move the OTHER piece at 8_1?
    // In current implementation, canMove(8, 1, 7, 2, board) will return true because turn is White.
    expect(checkersRules.canMove(8, 1, 7, 2, board)).toBe(false)
    expect(checkersRules.canMove(4, 5, 3, 4, board)).toBe(false)
  })
})
