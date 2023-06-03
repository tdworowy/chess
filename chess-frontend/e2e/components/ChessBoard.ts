import { Page } from 'playwright-core'
import { expect } from '@playwright/test'

class ChessBoard {
  page!: Page

  constructor(page: Page) {
    this.page = page
  }

  classes = {
    squareBlack: '.squareBlack',
    squareWhite: '.squareBlack'
  }

  testId = {
    square: 'square'
  }
  
  async assertRender() {
    const squers = this.page.getByTestId(this.testId.square)
    await expect(squers).toHaveCount(64)

    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        if (i % 2 !== 0 && j % 2 === 0) {
          await expect(squers.filter({ has: this.page.locator(`[id="${i}_${j}"]`) })).toHaveClass(
            'square squareBlack'
          )
        }
        if (i % 2 !== 0 && j % 2 !== 0) {
          await expect(squers.filter({ has: this.page.locator(`[id="${i}_${j}"]`) })).toHaveClass(
            'square squareWhite'
          )
        }
        if (i % 2 === 0 && j % 2 === 0) {
          await expect(squers.filter({ has: this.page.locator(`[id="${i}_${j}"]`) })).toHaveClass(
            'square squareWhite'
          )
        }
        if (i % 2 === 0 && j % 2 !== 0) {
          await expect(squers.filter({ has: this.page.locator(`[id="${i}_${j}"]`) })).toHaveClass(
            'square squareBlack'
          )
        }
      }
    }
  }
}

export const getChaseBoard = (page: Page) => {
  return new ChessBoard(page)
}
