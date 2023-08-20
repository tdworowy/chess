import { describe, it, expect } from 'vitest'
import { pieceColorCondiion, pieceCondition } from '../../piecesUtils'
import { Color, PawnType } from '@/types'

describe('Pieces uils', () => {
  it('test pieceCondition', () => {
    expect(pieceCondition(2, 2)).toBe(false)
    expect(pieceCondition(1, 1)).toBe(false)
    expect(pieceCondition(2, 1)).toBe(true)
    expect(pieceCondition(1, 2)).toBe(true)
  })
  it('test pieceColorCondiion', () => {
    expect(pieceColorCondiion(2, 1)).toStrictEqual([Color.Dark, PawnType.PawnDark])
    expect(pieceColorCondiion(6, 1)).toStrictEqual([Color.Light, PawnType.PawnLight])
    expect(pieceColorCondiion(2, 2)).toStrictEqual([Color.Empty, PawnType.Empty])
    expect(pieceColorCondiion(6, 2)).toStrictEqual([Color.Empty, PawnType.Empty])
  })
})
