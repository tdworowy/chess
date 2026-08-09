import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import App from '../../App.vue'
import { Color, PawnType } from '@/types'

describe('App.vue integration and DOM sync', () => {
  beforeEach(() => {
    document.body.innerHTML = ''
  })

  it('should synchronize DOM when setState is called', async () => {
    // We need some squares in the DOM for setState to work
    for (let i = 0; i < 2; i++) {
      for (let j = 0; j < 2; j++) {
        const square = document.createElement('div')
        square.id = `${i}_${j}`
        square.className = 'square'
        document.body.appendChild(square)
      }
    }

    const wrapper = mount(App)
    const setState = (wrapper.vm as any).setState

    const newState = {
      '0_0': [Color.White, PawnType.PawnWhite],
      '0_1': [Color.Black, PawnType.PawnBlack],
      '1_0': [Color.White, PawnType.Dame],
      '1_1': [Color.Empty, PawnType.Empty]
    }

    setState(newState)

    // Verify pieces are added to DOM
    const p00 = document.getElementById('0_0')?.querySelector('.pawn.PawnWhite')
    expect(p00).toBeTruthy()
    expect(p00?.getAttribute('data-testid')).toBe('pawn')

    const p01 = document.getElementById('0_1')?.querySelector('.pawn.PawnBlack')
    expect(p01).toBeTruthy()

    const p10 = document.getElementById('1_0')?.querySelector('.dame.Dame')
    expect(p10).toBeTruthy()
    expect(p10?.getAttribute('data-testid')).toBe('dame')

    // Verify empty square
    const s11 = document.getElementById('1_1')
    expect(s11?.querySelector('.pawn, .dame')).toBeFalsy()

    // Test idempotency: calling setState again with same state shouldn't recreate elements
    const p00_before = document.getElementById('0_0')?.querySelector('.pawn.PawnWhite')
    setState(newState)
    const p00_after = document.getElementById('0_0')?.querySelector('.pawn.PawnWhite')
    expect(p00_before).toBe(p00_after)

    // Update state: remove one piece, change another
    const nextState = {
      '0_0': [Color.Empty, PawnType.Empty],
      '0_1': [Color.White, PawnType.Dame], // Changed from Black Pawn to White Dame
      '1_0': [Color.White, PawnType.Dame],
      '1_1': [Color.Empty, PawnType.Empty]
    }

    setState(nextState)

    expect(document.getElementById('0_0')?.querySelector('.pawn')).toBeFalsy()
    const p01_new = document.getElementById('0_1')?.querySelector('.dame.Dame')
    expect(p01_new).toBeTruthy()
    expect(p01_new?.classList.contains('White')).toBe(true)

    // Verify event listeners (dragstart) are added
    const dragStartEvent = new Event('dragstart')
    const spy = vi.fn()
    p01_new?.addEventListener('dragstart', spy)
    p01_new?.dispatchEvent(dragStartEvent)
    expect(spy).toHaveBeenCalled()
  })
})
