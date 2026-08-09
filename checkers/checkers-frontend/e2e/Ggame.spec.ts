import { test } from '@playwright/test'
import { ChessBoard } from './components/ChessBoard'

test.describe('Game of checkers', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
    await page.evaluate(() => {
      ;(window as any).disableAI = true
    })
  })
  test('Whites can make a move', async ({ page }) => {
    await page.goto('/')
    const chaseBoard = ChessBoard.getChaseBoard(page)
    await page.evaluate(() => {
      ;(window as any).disableAI = true
    })
    await chaseBoard.dragPiceToSquare('6_1', '5_2')
    await page.waitForTimeout(300)
    await chaseBoard.assertPiceOnSquare('5_2')
  })
})
