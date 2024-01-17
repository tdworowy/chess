import { test } from '@playwright/test'
import { ChessBoard } from './components/ChessBoard'
import { before } from 'node:test'

test.describe('Game of checkers', () => {
  test('Whites can make a move', async ({ page }) => {
    await page.goto('/')
    let chaseBoard = ChessBoard.getChaseBoard(page)
    await chaseBoard.dragPiceToSquare('6_1', '5_2')
    await chaseBoard.assertPiceOnSquare('5_2')
  })
})
