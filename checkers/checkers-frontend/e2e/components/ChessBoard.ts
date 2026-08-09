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
    squareWhite: '.squareWhite'
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
    const pawn = this.page.locator(`[id="${piceId}"][data-testid='${this.pieces.testId.pawn}']`)
    const square = this.page.locator(`[id="${squareId}"][data-testid='${this.testId.square}']`)

    await pawn.waitFor({ state: 'visible' })
    await pawn.dragTo(square)
  }

  async assertPiceOnSquare(id: string) {
    const pawnCss = `[id="${id}"][data-testid='${this.pieces.testId.pawn}']`
    const square = this.page.locator(`[id="${id}"][data-testid='${this.testId.square}']`)

    await expect(square.locator(pawnCss)).toBeVisible()
  }

  async assertDameOnSquare(id: string) {
    const dameCss = `[id="${id}"][data-testid='${this.pieces.testId.dame}']`
    const square = this.page.locator(`[id="${id}"][data-testid='${this.testId.square}']`)

    await expect(square.locator(dameCss)).toBeVisible()
  }

  async assertSquareEmpty(id: string) {
    const square = this.page.locator(`[id="${id}"][data-testid='${this.testId.square}']`)
    await expect(square.locator('div')).toHaveCount(0)
  }

  async dragDameToSquare(piceId: string, squareId: string) {
    const dame = this.page.locator(`[id="${piceId}"][data-testid='${this.pieces.testId.dame}']`)
    const square = this.page.locator(`[id="${squareId}"][data-testid='${this.testId.square}']`)

    await dame.waitFor({ state: 'visible' })
    await dame.dragTo(square)
  }

  static getChaseBoard(page: Page) {
    return new ChessBoard(page)
  }

  async setBoardState(state: any) {
    await this.page.evaluate((s) => {
      ;(window as any).setBoardState(s)
    }, state)
  }

  async setTurn(color: string) {
    await this.page.evaluate((c) => {
      ;(window as any).checkersRules.currentTurnColor = c
      ;(window as any).checkersRules.nextTurnColor = c === 'White' ? 'Black' : 'White'
    }, color)
  }
}
