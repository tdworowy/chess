import { test } from '@playwright/test'
import { Pieces } from './components/Pieces'

test.describe('Pieces', () => {
  test('Pawns should render correctly', async ({ page }) => {
    await page.goto('/')
    await Pieces.getPieces(page).assertRenderPawns()
  })
})
