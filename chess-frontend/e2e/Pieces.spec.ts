import { test } from '@playwright/test'
import { getPieces } from './components/Pieces'

test.describe('Pieces', () => {
  test('Pawns should render correctly', async ({ page }) => {
    await page.goto('/')
    await getPieces(page).assertRenderPawns()
  })
})
