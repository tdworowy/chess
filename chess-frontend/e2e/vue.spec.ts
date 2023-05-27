import { test } from '@playwright/test'
import { getChaseBoard } from './components/ChessBoard'

test.describe('ChessBoard', () => {
  test('ChessBoard should render correctly', async ({ page }) => {
    await page.goto('/')
    //getChaseBoard(page).assertRender()
  })
})
