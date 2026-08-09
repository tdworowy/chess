import { test } from '@playwright/test'
import { ChessBoard } from './components/ChessBoard'
import { Color, PawnType } from '../src/types'

test.describe('Checkers Queen E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
    await page.evaluate(() => {
      ;(window as any).disableAI = true
    })
  })

  test('turning to queen (promotion)', async ({ page }) => {
    const board = ChessBoard.getChaseBoard(page)

    const state: any = {}
    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        state[`${i}_${j}`] = [Color.Empty, PawnType.Empty]
      }
    }
    state['2_2'] = [Color.White, PawnType.PawnWhite]

    await board.setBoardState(state)
    await board.setTurn(Color.White)

    await board.dragPiceToSquare('2_2', '1_1')

    await board.assertDameOnSquare('1_1')

    // Test Black side promotion
    state['1_1'] = [Color.Empty, PawnType.Empty]
    state['7_7'] = [Color.Black, PawnType.PawnBlack]
    await board.setBoardState(state)
    await board.setTurn(Color.Black)
    await board.dragPiceToSquare('7_7', '8_8')
    await board.assertDameOnSquare('8_8')
  })

  test('queen moving and beating', async ({ page }) => {
    const board = ChessBoard.getChaseBoard(page)

    const state: any = {}
    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        state[`${i}_${j}`] = [Color.Empty, PawnType.Empty]
      }
    }
    state['6_3'] = [Color.White, PawnType.Dame]
    state['4_5'] = [Color.Black, PawnType.PawnBlack]

    await board.setBoardState(state)
    await board.setTurn(Color.White)

    // Queen move (without beating)
    await board.dragDameToSquare('6_3', '5_2')
    await board.assertDameOnSquare('5_2')

    // Reset for beating test
    state['5_2'] = [Color.Empty, PawnType.Empty]
    state['6_3'] = [Color.White, PawnType.Dame]
    await board.setBoardState(state)
    await board.setTurn(Color.White)

    // Queen beating: (6,3) jumps over (4,5) to (3,6)
    await board.dragDameToSquare('6_3', '3_6')
    await board.assertDameOnSquare('3_6')
    await board.assertSquareEmpty('6_3')
    await board.assertSquareEmpty('4_5')
  })
})
