import { Page } from 'playwright-core'
import { expect } from '@playwright/test'
import { getByTestId } from '../urils'

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
    squer: 'squer'
  }

  async assertRender() {
    const squers = getByTestId(this.page, this.testId.squer)
    expect(squers).toHaveCount(64)

    let i = 1
    let j = 1

    for (const squer of await squers.all()) {
      if (i % 2 !== 0 && j % 2 == 0) {
        expect(squer.locator(`[id='${i}_${j}']`)).toHaveClass('squareWhite')
      }
      if (i % 2 !== 0 && j % 2 !== 0) {
        expect(squer.locator(`[id='${i}_${j}']`)).toHaveClass('squareBlack')
      }
      if (i % 2 == 0 && j % 2 == 0) {
        expect(squer.locator(`[id='${i}_${j}']`)).toHaveClass('squareBlack')
      }
      if (i % 2 == 0 && j % 2 !== 0) {
        expect(squer.locator(`[id='${i}_${j}']`)).toHaveClass('squareWhite')
      }
    }
  }
}

export const getChaseBoard = (page: Page) => {
  return new ChessBoard(page)
}
