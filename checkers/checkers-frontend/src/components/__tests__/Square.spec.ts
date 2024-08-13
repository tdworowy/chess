import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import Square from '../Square.vue'
import { Color } from '@/types'

describe('Square component', () => {
  it('renders properly - White', () => {
    const wrapperWhite = mount(Square, {
      global: {
        provide: {
          setState: () => {},
          getState: () => {}
        }
      },
      propsData: {
        x: 1,
        y: 2,
        color: Color.White
      }
    })
    expect(wrapperWhite.attributes('id')).toBe('1_2')
    expect(wrapperWhite.attributes('class')).toBe('square squareWhite')
  })

  it('renders properly - Black', () => {
    const wrapperBlack = mount(Square, {
      global: {
        provide: {
          setState: () => {},
          getState: () => {}
        }
      },
      propsData: {
        x: 6,
        y: 1,
        color: Color.Black
      }
    })
    expect(wrapperBlack.attributes('id')).toBe('6_1')
    expect(wrapperBlack.attributes('class')).toBe('square squareBlack')
  })
})
