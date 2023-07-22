import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import ChessBoard from '../ChessBoard.vue'

describe('ChessBoard component', () => {
  it('renders properly', () => {
    const wrapper = mount(ChessBoard,{ 
      global: { 
        provide: { 
          setState: () =>{} 
        } 
      } 
    })
    expect(wrapper.findAll('.squareBlack').length).toEqual(32)
    expect(wrapper.findAll('.squareWhite').length).toEqual(32)
  })
})
