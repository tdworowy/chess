import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import Pieces from '../Pieces.vue'

describe('Pieces component', () => {
  it('renders properly', () => {
    const wrapperBlack = mount(Pieces, {
      propsData: {
        x: 1,
        y: 2
      }
    })
    const wrapperWhite = mount(Pieces, {
      propsData: {
        x: 8,
        y: 1
      }
    })
    expect(wrapperBlack.findAll('.pawnDark').length).toEqual(1)
    expect(wrapperWhite.findAll('.pawnLight').length).toEqual(1)
  })
})
