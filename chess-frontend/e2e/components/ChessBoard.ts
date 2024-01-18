import { Page } from 'playwright-core'
import { expect } from '@playwright/test'
import { Pieces } from './Pieces'

export class ChessBoard {
  page!: Page
  pieces!: Pieces

  private constructor(page: Page) {
    this.page = page
    this.pieces = Pieces.getPieces(page)
  }

  readonly classes = {
    squareBlack: '.squareBlack',
    squareWhite: '.squareBlack'
  }

  readonly testId = {
    square: 'square'
  }

  async assertRender() {
    const squers = this.page.getByTestId(this.testId.square)
    await expect(squers).toHaveCount(64)

    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        if (i % 2 !== 0 && j % 2 === 0) {
          await expect(
            this.page.locator(`[id="${i}_${j}"][data-testid='${this.testId.square}']`)
          ).toHaveClass('square squareBlack')
        }
        if (i % 2 !== 0 && j % 2 !== 0) {
          await expect(
            this.page.locator(`[id="${i}_${j}"][data-testid='${this.testId.square}']`)
          ).toHaveClass('square squareWhite')
        }
        if (i % 2 === 0 && j % 2 === 0) {
          await expect(
            this.page.locator(`[id="${i}_${j}"][data-testid='${this.testId.square}']`)
          ).toHaveClass('square squareWhite')
        }
        if (i % 2 === 0 && j % 2 !== 0) {
          await expect(
            this.page.locator(`[id="${i}_${j}"][data-testid='${this.testId.square}']`)
          ).toHaveClass('square squareBlack')
        }
      }
    }
  }

  async dragPiceToSquare(piceId: string, squareId: string) {
    let pawn = this.page.locator(`[id="${piceId}"][data-testid='${this.pieces.testId.pawn}']`)
    let square = this.page.locator(`[id="${squareId}"][data-testid='${this.testId.square}']`)

    await pawn.dragTo(square)
  }

  async assertPiceOnSquare(id: string) {
    let pawnCss = `[id="${id}"][data-testid='${this.pieces.testId.pawn}']`
    let square = this.page.locator(`[id="${id}"][data-testid='${this.testId.square}']`)

    await expect(square.locator(pawnCss)).toBeVisible()
  }

  static getChaseBoard(page: Page) {
    return new ChessBoard(page)
  }
}
