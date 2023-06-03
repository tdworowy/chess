import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import Pieces from '../Pieces.vue'

describe('pieces', () => {
  it('renders properly', () => {
    const wrapperBlack = mount(Pieces, {
      propsData: {
        x: 1,
        y: 1
      }
    })
    const wrapperWhite = mount(Pieces, {
      propsData: {
        x: 8,
        y: 1
      }
    })
    expect(wrapperBlack.findAll('.pawnBlack').length).toEqual(1)
    expect(wrapperWhite.findAll('.pawnWhite').length).toEqual(1)
  })
})
