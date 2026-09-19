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
    for (let i = 1; i <= 8; i++) {
      for (let j = 1; j <= 8; j++) {
        const square = document.createElement('div')
        square.id = `${i}_${j}`
        square.className = 'square'
        document.body.appendChild(square)
      }
    }

    const wrapper = mount(App)
    const setState = (wrapper.vm as any).setState

    const newState = {
      '1_1': [Color.White, PawnType.PawnWhite],
      '1_2': [Color.Black, PawnType.PawnBlack],
      '2_1': [Color.White, PawnType.Dame],
      '2_2': [Color.Empty, PawnType.Empty]
    }

    setState(newState)

    // Verify pieces are added to DOM
    const p11 = document.getElementById('1_1')?.querySelector('.pawn.PawnWhite')
    expect(p11).toBeTruthy()
    expect(p11?.getAttribute('data-testid')).toBe('pawn')

    const p12 = document.getElementById('1_2')?.querySelector('.pawn.PawnBlack')
    expect(p12).toBeTruthy()

    const p21 = document.getElementById('2_1')?.querySelector('.dame.Dame')
    expect(p21).toBeTruthy()
    expect(p21?.getAttribute('data-testid')).toBe('dame')

    // Verify empty square
    const s22 = document.getElementById('2_2')
    expect(s22?.querySelector('.pawn, .dame')).toBeFalsy()

    // Test idempotency: calling setState again with same state shouldn't recreate elements
    const p11_before = document.getElementById('1_1')?.querySelector('.pawn.PawnWhite')
    setState(newState)
    const p11_after = document.getElementById('1_1')?.querySelector('.pawn.PawnWhite')
    expect(p11_before).toBe(p11_after)

    // Update state: remove one piece, change another
    const nextState = {
      '1_1': [Color.Empty, PawnType.Empty],
      '1_2': [Color.White, PawnType.Dame], // Changed from Black Pawn to White Dame
      '2_1': [Color.White, PawnType.Dame],
      '2_2': [Color.Empty, PawnType.Empty]
    }

    setState(nextState)

    expect(document.getElementById('1_1')?.querySelector('.pawn')).toBeFalsy()
    const p12_new = document.getElementById('1_2')?.querySelector('.dame.Dame')
    expect(p12_new).toBeTruthy()
    expect(p12_new?.classList.contains('White')).toBe(true)

    // Verify event listeners (dragstart) are added
    const dragStartEvent = new Event('dragstart')
    const spy = vi.fn()
    p12_new?.addEventListener('dragstart', spy)
    p12_new?.dispatchEvent(dragStartEvent)
    expect(spy).toHaveBeenCalled()
  })
})
