import { test } from '@playwright/test'
import { ChessBoard } from './components/ChessBoard'

test.describe('ChessBoard', () => {
  test('ChessBoard should render correctly', async ({ page }) => {
    await page.goto('/')
    await ChessBoard.getChaseBoard(page).assertRender()
  })
})
