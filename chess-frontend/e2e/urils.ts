import { Page } from 'playwright-core'

export const getByTestId = (page: Page, testId: string) => {
  return page.getByTestId(testId)
}
