import { test } from '@playwright/test'
import { ChessBoard } from './components/ChessBoard'
import { Color, PawnType } from '../src/types'

test.describe('Checkers Game E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
    await page.evaluate(() => {
      ;(window as any).disableAI = true
    })
  })

  test('single beating', async ({ page }) => {
    const board = ChessBoard.getChaseBoard(page)

    const state: any = {}
    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        state[`${i}_${j}`] = [Color.Empty, PawnType.Empty]
      }
    }
    state['5_2'] = [Color.White, PawnType.PawnWhite]
    state['4_3'] = [Color.Black, PawnType.PawnBlack]

    await board.setBoardState(state)
    await board.setTurn(Color.White)
    await page.waitForTimeout(100)
    await board.dragPiceToSquare('5_2', '3_4')
    await page.waitForTimeout(300)

    await board.assertPiceOnSquare('3_4')
    await board.assertSquareEmpty('5_2')
    await board.assertSquareEmpty('4_3')

    // Test Black side beating
    state['3_4'] = [Color.White, PawnType.PawnWhite]
    state['2_5'] = [Color.Black, PawnType.PawnBlack]
    await board.setBoardState(state)
    await board.setTurn(Color.Black)
    await page.waitForTimeout(100)
    await board.dragPiceToSquare('2_5', '4_3')
    await page.waitForTimeout(300)
    await board.assertPiceOnSquare('4_3')
    await board.assertSquareEmpty('2_5')
    await board.assertSquareEmpty('3_4')
  })

  test('multiple beating', async ({ page }) => {
    const board = ChessBoard.getChaseBoard(page)

    const state: any = {}
    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        state[`${i}_${j}`] = [Color.Empty, PawnType.Empty]
      }
    }
    state['7_2'] = [Color.White, PawnType.PawnWhite]
    state['6_3'] = [Color.Black, PawnType.PawnBlack]
    state['4_5'] = [Color.Black, PawnType.PawnBlack]

    await board.setBoardState(state)
    await board.setTurn(Color.White)
    await page.waitForTimeout(100)

    // First jump
    await board.dragPiceToSquare('7_2', '5_4')
    await page.waitForTimeout(300)
    await board.assertPiceOnSquare('5_4')
    await board.assertSquareEmpty('7_2')
    await board.assertSquareEmpty('6_3')

    // Second jump (should still be White's turn because multi-jump is available)
    await board.dragPiceToSquare('5_4', '3_6')
    await page.waitForTimeout(300)
    await board.assertPiceOnSquare('3_6')
    await board.assertSquareEmpty('5_4')
    await board.assertSquareEmpty('4_5')
  })
})
