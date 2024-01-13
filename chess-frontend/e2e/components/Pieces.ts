import { Page } from 'playwright-core'
import { expect } from '@playwright/test'

export class Pieces {
  page!: Page

  private constructor(page: Page) {
    this.page = page
  }

  classes = {
    pawnBlack: '.pawnBlack',
    pawnWhite: '.pawnWhite'
  }

  testId = {
    pawn: 'pawn'
  }

  async assertRenderPawns() {
    const pawns = this.page.getByTestId(this.testId.pawn)
    await expect(pawns).toHaveCount(24)
  }

  static getPieces(page: Page) {
    return new Pieces(page)
  }
}
