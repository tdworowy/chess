import { describe, it, expect } from 'vitest'
import { pieceColorCondition, pieceCondition } from '../../piecesUtils'
import { Color, PawnType } from '@/types'

describe('Pieces uils', () => {
  it('test pieceCondition', () => {
    expect(pieceCondition(2, 2)).toBe(false)
    expect(pieceCondition(1, 1)).toBe(false)
    expect(pieceCondition(2, 1)).toBe(true)
    expect(pieceCondition(1, 2)).toBe(true)
  })
  it('test pieceColorCondition', () => {
    expect(pieceColorCondition(2, 1)).toStrictEqual([Color.Black, PawnType.PawnBlack])
    expect(pieceColorCondition(6, 1)).toStrictEqual([Color.White, PawnType.PawnWhite])
    expect(pieceColorCondition(2, 2)).toStrictEqual([Color.Empty, PawnType.Empty])
    expect(pieceColorCondition(6, 2)).toStrictEqual([Color.Empty, PawnType.Empty])
  })
})
