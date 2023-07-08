import { describe, it, expect } from 'vitest'
import { pieceColorCondiion, pieceCondition } from '../piecesUtils'

describe('Pieces uils', () => {
  it('test pieceCondition', () => {
    expect(pieceCondition(2, 2)).toBe(false)
    expect(pieceCondition(1, 1)).toBe(false)
    expect(pieceCondition(2, 1)).toBe(true)
    expect(pieceCondition(1, 2)).toBe(true)
  })
  it('test pieceColorCondiion', () => {
    expect(pieceColorCondiion(2, 1)).toBe('Dark')
    expect(pieceColorCondiion(6, 1)).toBe('Light')
    expect(pieceColorCondiion(2, 2)).toBe('')
    expect(pieceColorCondiion(6, 2)).toBe('')
   
  })
})
