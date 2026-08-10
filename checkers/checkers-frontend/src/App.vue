<script setup lang="ts">
import ChessBoard from './components/ChessBoard.vue'
import { provide, ref } from 'vue'
import { Color, PawnType } from './types'

const boardState = ref<{ [key: string]: [Color, PawnType] }>({})

const setState = (newState: { [key: string]: [Color, PawnType] }) => {
  boardState.value = { ...newState }

  // Update DOM manually for E2E tests and consistency
  console.log(`[setState] Updating DOM for ${Object.keys(newState).length} squares.`)
  for (const [key, value] of Object.entries(newState)) {
    const existingPawn = document.querySelector(
      `[id='${key}'][class*='pawn'], [id='${key}'][class*='dame']`
    )

    if (value[0] === Color.Empty) {
      if (existingPawn) {
        console.log(`[setState] Removing pawn from ${key}`)
        existingPawn.remove()
      }
    } else {
      const expectedClass = value[1] === PawnType.Dame ? 'dame' : 'pawn'

      // Check if the current DOM already matches the expected state
      if (existingPawn) {
        const hasCorrectId = existingPawn.id === key
        const hasCorrectType = existingPawn.classList.contains(value[1])
        const hasCorrectBaseClass = existingPawn.classList.contains(expectedClass)
        const hasCorrectTestId = existingPawn.getAttribute('data-testid') === expectedClass
        const hasCorrectWhiteClass =
          value[0] === Color.White
            ? existingPawn.classList.contains('White') ||
              existingPawn.classList.contains(PawnType.PawnWhite)
            : !existingPawn.classList.contains('White')

        // Ensure it's in the correct square
        const parentSquare = existingPawn.parentElement
        const isChildOfCorrectSquare =
          parentSquare && parentSquare.id === key && parentSquare.classList.contains('square')

        if (
          hasCorrectId &&
          hasCorrectType &&
          hasCorrectBaseClass &&
          hasCorrectTestId &&
          isChildOfCorrectSquare &&
          hasCorrectWhiteClass
        ) {
          // DOM is already correct, skip update to avoid disrupting E2E drag-and-drop
          continue
        }
        console.log(`[setState] DOM mismatch at ${key}, removing and re-adding.`)
        existingPawn.remove()
      }

      const newPawn = document.createElement('div')
      newPawn.id = key
      newPawn.className = `${expectedClass} ${value[1]}`
      if (value[0] === Color.White && value[1] === PawnType.Dame) {
        newPawn.classList.add('White')
      }
      newPawn.setAttribute('data-testid', `${expectedClass}`)
      newPawn.setAttribute('draggable', 'true')

      newPawn.addEventListener('dragstart', (event: DragEvent) => {
        const target = event.target as HTMLElement
        console.log(`[dragstart] Piece ID: ${target.id}`)
        event.dataTransfer?.setData('id', target.id)
        if (event.dataTransfer) {
          event.dataTransfer.effectAllowed = 'move'
        }
      })

      const square = document.querySelector(`[id='${key}'][class*='square']`)
      if (square) {
        square.appendChild(newPawn)
      } else {
        console.error(`[setState] Square not found for key ${key}`)
      }
    }
  }
}
const getState = () => {
  return boardState.value
}

provide('setState', setState)
provide('getState', getState)

if (typeof window !== 'undefined') {
  ;(window as any).setBoardState = setState
  ;(window as any).getBoardState = getState
}
</script>

<template>
  <header></header>

  <main>
    <ChessBoard />
  </main>
</template>
