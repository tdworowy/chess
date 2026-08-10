import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import Square from '../Square.vue'
import { Color, PawnType } from '@/types'
import { checkersRules } from '@/rules'

vi.mock('@/Api', () => ({
  Api: {
    healtCheck: vi.fn(),
    makeRandomMove: vi.fn()
  }
}))

describe('Square.vue drop handler', () => {
  let boardState: { [key: string]: [Color, PawnType] }
  const setState = vi.fn((newState) => {
    boardState = { ...newState }
  })
  const getState = vi.fn(() => boardState)

  beforeEach(() => {
    vi.clearAllMocks()
    // Reset checkersRules to initial state (White's turn)
    checkersRules.currentTurnColor = Color.White
    checkersRules.nextTurnColor = Color.Black
    boardState = {}

    // Basic board setup
    for (let i = 0; i <= 9; i++) {
      for (let j = 0; j <= 9; j++) {
        boardState[`${i}_${j}`] = [Color.Empty, PawnType.Empty]
      }
    }

    // Clear DOM
    document.body.innerHTML = ''
    // Add global disableAI flag for tests
    ;(window as any).disableAI = true
  })

  const createSquareWrapper = (x: number, y: number, color: Color) => {
    return mount(Square, {
      global: {
        provide: {
          setState,
          getState
        }
      },
      props: { x, y, color }
    })
  }

  it('should switch turn after a simple move', async () => {
    boardState['6_1'] = [Color.White, PawnType.PawnWhite]
    const wrapper = createSquareWrapper(5, 2, Color.Black)

    // We need the source piece in DOM for document.querySelector in Square.vue
    const sourcePiece = document.createElement('div')
    sourcePiece.id = '6_1'
    sourcePiece.className = 'pawn PawnWhite'
    document.body.appendChild(sourcePiece)

    // Target square in DOM
    const targetSquare = document.createElement('div')
    targetSquare.id = '5_2'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('6_1')
      }
    } as unknown as DragEvent

    // Trigger drop
    await (wrapper.vm as any).drop(dragEvent)

    expect(boardState['5_2']).toEqual([Color.White, PawnType.PawnWhite])
    expect(boardState['6_1']).toEqual([Color.Empty, PawnType.Empty])
    expect(checkersRules.currentTurnColor).toBe(Color.Black)
  })

  it('should promote pawn to dame when reaching the end', async () => {
    boardState['2_2'] = [Color.White, PawnType.PawnWhite]
    const wrapper = createSquareWrapper(1, 1, Color.White)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '2_2'
    sourcePiece.className = 'pawn PawnWhite'
    document.body.appendChild(sourcePiece)

    const targetSquare = document.createElement('div')
    targetSquare.id = '1_1'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('2_2')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    expect(boardState['1_1']).toEqual([Color.White, PawnType.Dame])
    expect(sourcePiece.classList.contains('dame')).toBe(true)
    expect(sourcePiece.getAttribute('data-testid')).toBe('dame')
  })

  it('should handle single beat and switch turn', async () => {
    boardState['5_2'] = [Color.White, PawnType.PawnWhite]
    boardState['4_3'] = [Color.Black, PawnType.PawnBlack]

    const wrapper = createSquareWrapper(3, 4, Color.Black)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '5_2'
    sourcePiece.className = 'pawn PawnWhite'
    document.body.appendChild(sourcePiece)

    const beatenPiece = document.createElement('div')
    beatenPiece.id = '4_3'
    beatenPiece.className = 'pawn PawnBlack'
    document.body.appendChild(beatenPiece)

    const targetSquare = document.createElement('div')
    targetSquare.id = '3_4'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('5_2')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    expect(boardState['3_4']).toEqual([Color.White, PawnType.PawnWhite])
    expect(boardState['5_2']).toEqual([Color.Empty, PawnType.Empty])
    expect(boardState['4_3']).toEqual([Color.Empty, PawnType.Empty])
    expect(checkersRules.currentTurnColor).toBe(Color.Black)
  })

  it('should handle multi-beat and NOT switch turn', async () => {
    boardState['7_2'] = [Color.White, PawnType.PawnWhite]
    boardState['6_3'] = [Color.Black, PawnType.PawnBlack]
    boardState['4_5'] = [Color.Black, PawnType.PawnBlack]

    const wrapper = createSquareWrapper(5, 4, Color.Black)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '7_2'
    sourcePiece.className = 'pawn PawnWhite'
    document.body.appendChild(sourcePiece)

    const beatenPiece1 = document.createElement('div')
    beatenPiece1.id = '6_3'
    beatenPiece1.className = 'pawn PawnBlack'
    document.body.appendChild(beatenPiece1)

    const beatenPiece2 = document.createElement('div')
    beatenPiece2.id = '4_5'
    beatenPiece2.className = 'pawn PawnBlack'
    document.body.appendChild(beatenPiece2)

    const targetSquare = document.createElement('div')
    targetSquare.id = '5_4'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('7_2')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    expect(boardState['5_4']).toEqual([Color.White, PawnType.PawnWhite])
    expect(boardState['6_3']).toEqual([Color.Empty, PawnType.Empty])
    // Turn should still be White
    expect(checkersRules.currentTurnColor).toBe(Color.White)

    // Now perform the second jump
    const wrapper2 = createSquareWrapper(3, 6, Color.Black)
    const targetSquare2 = document.createElement('div')
    targetSquare2.id = '3_6'
    targetSquare2.className = 'square'
    document.body.appendChild(targetSquare2)

    const dragEvent2 = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare2,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('5_4')
      }
    } as unknown as DragEvent

    await (wrapper2.vm as any).drop(dragEvent2)

    expect(boardState['3_6']).toEqual([Color.White, PawnType.PawnWhite])
    expect(boardState['4_5']).toEqual([Color.Empty, PawnType.Empty])
    // Now it should be Black's turn
    expect(checkersRules.currentTurnColor).toBe(Color.Black)
  })

  it('should handle dame long range beating', async () => {
    boardState['6_3'] = [Color.White, PawnType.Dame]
    boardState['4_5'] = [Color.Black, PawnType.PawnBlack]

    const wrapper = createSquareWrapper(3, 6, Color.Black)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '6_3'
    sourcePiece.className = 'dame Dame'
    document.body.appendChild(sourcePiece)

    const beatenPiece = document.createElement('div')
    beatenPiece.id = '4_5'
    beatenPiece.className = 'pawn PawnBlack'
    document.body.appendChild(beatenPiece)

    const targetSquare = document.createElement('div')
    targetSquare.id = '3_6'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('6_3')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    expect(boardState['3_6']).toEqual([Color.White, PawnType.Dame])
    expect(boardState['6_3']).toEqual([Color.Empty, PawnType.Empty])
    expect(boardState['4_5']).toEqual([Color.Empty, PawnType.Empty])
    expect(checkersRules.currentTurnColor).toBe(Color.Black)
  })

  it('should NOT allow move when it is not the players turn', async () => {
    // Current turn is White, but we try to move a Black piece
    checkersRules.currentTurnColor = Color.White
    boardState['6_1'] = [Color.Black, PawnType.PawnBlack]

    const wrapper = createSquareWrapper(7, 2, Color.Black)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '6_1'
    sourcePiece.className = 'pawn PawnBlack'
    document.body.appendChild(sourcePiece)

    const targetSquare = document.createElement('div')
    targetSquare.id = '7_2'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('6_1')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    // Move should be rejected
    expect(boardState['7_2']).toEqual([Color.Empty, PawnType.Empty])
    expect(boardState['6_1']).toEqual([Color.Black, PawnType.PawnBlack])
    expect(checkersRules.currentTurnColor).toBe(Color.White)
  })

  it('should NOT allow moving to an occupied square', async () => {
    boardState['6_1'] = [Color.White, PawnType.PawnWhite]
    boardState['5_2'] = [Color.Black, PawnType.PawnBlack]

    const wrapper = createSquareWrapper(5, 2, Color.Black)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '6_1'
    sourcePiece.className = 'pawn PawnWhite'
    document.body.appendChild(sourcePiece)

    const targetSquare = document.createElement('div')
    targetSquare.id = '5_2'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    // Add the piece that occupies the target square to DOM
    const targetPiece = document.createElement('div')
    targetPiece.id = '5_2'
    targetPiece.className = 'pawn PawnBlack'
    targetSquare.appendChild(targetPiece)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('6_1')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    // Move should be rejected
    expect(boardState['5_2']).toEqual([Color.Black, PawnType.PawnBlack])
    expect(boardState['6_1']).toEqual([Color.White, PawnType.PawnWhite])
  })

  it('should NOT allow illegal moves (e.g., jumping over own piece)', async () => {
    boardState['7_2'] = [Color.White, PawnType.PawnWhite]
    boardState['6_3'] = [Color.White, PawnType.PawnWhite] // Own piece

    const wrapper = createSquareWrapper(5, 4, Color.Black)

    const sourcePiece = document.createElement('div')
    sourcePiece.id = '7_2'
    sourcePiece.className = 'pawn PawnWhite'
    document.body.appendChild(sourcePiece)

    const targetSquare = document.createElement('div')
    targetSquare.id = '5_4'
    targetSquare.className = 'square'
    document.body.appendChild(targetSquare)

    const dragEvent = {
      preventDefault: vi.fn(),
      currentTarget: targetSquare,
      dataTransfer: {
        getData: vi.fn().mockReturnValue('7_2')
      }
    } as unknown as DragEvent

    await (wrapper.vm as any).drop(dragEvent)

    // Move should be rejected
    expect(boardState['5_4']).toEqual([Color.Empty, PawnType.Empty])
    expect(boardState['7_2']).toEqual([Color.White, PawnType.PawnWhite])
  })
})
