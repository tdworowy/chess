<script setup lang="ts">
import { Api } from '@/Api'
import { checkersRules } from '@/rules'
import { Color, PawnType, Player, type boardStateType } from '@/types'
import { inject } from 'vue'

const testId = { 'data-testid': 'square' }
const classBlack = 'square squareBlack'
const classWhite = 'square squareWhite'

const setState = inject('setState') as boardStateType
const getState = inject('getState') as () => { [key: string]: [Color, PawnType] }

const props = defineProps<{
  x: number
  y: number
  color: Color
}>()

const cls: String = props.color === Color.Black ? classBlack : classWhite

function allowDrop(event: DragEvent) {
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
  event.preventDefault()
}

function dragStartHandler(event: DragEvent) {
  const target = event.target as HTMLElement
  console.log(`[dragstart] Piece ID: ${target.id}`)
  event.dataTransfer?.setData('id', target.id)
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
  }
}

function updateBoard(
  boardState: { [key: string]: [Color, PawnType] },
  newBoardState: { [key: string]: [Color, PawnType] }
) {
  for (const [key, value] of Object.entries(newBoardState)) {
    if (JSON.stringify(boardState[key]) !== JSON.stringify(value)) {
      console.log(`[updateBoard] ${boardState[key]} !== ${value}`)
      const existingPawn = document.querySelector(
        `[id='${key}'][class*='pawn'], [id='${key}'][class*='dame']`
      )

      if (value[0] == Color.Empty) {
        if (existingPawn) {
          console.log(`[updateBoard] Removing pawn from ${key}`)
          existingPawn.remove()
        }
      } else {
        const expectedClass = value[1] === PawnType.Dame ? 'dame' : 'pawn'

        if (existingPawn) {
          const hasCorrectType = existingPawn.classList.contains(value[1])
          const hasCorrectBaseClass = existingPawn.classList.contains(expectedClass)
          const hasCorrectTestId = existingPawn.getAttribute('data-testid') === expectedClass
          const hasCorrectWhiteClass =
            value[0] === Color.White
              ? existingPawn.classList.contains('White') ||
                existingPawn.classList.contains(PawnType.PawnWhite)
              : !existingPawn.classList.contains('White')

          const parentSquare = existingPawn.parentElement
          const isChildOfCorrectSquare =
            parentSquare && parentSquare.id === key && parentSquare.classList.contains('square')

          if (
            hasCorrectType &&
            hasCorrectBaseClass &&
            hasCorrectTestId &&
            isChildOfCorrectSquare &&
            hasCorrectWhiteClass
          ) {
            continue
          }
          console.log(`[updateBoard] DOM mismatch at ${key}, removing and re-adding.`)
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

        newPawn.addEventListener('dragstart', dragStartHandler)

        const square = document.querySelector(`[id='${key}'][class*='square']`)
        square?.appendChild(newPawn)
      }
    }
  }
}
function beat(
  startX: number,
  startY: number,
  endX: number,
  endY: number,
  boardState: { [key: string]: [Color, PawnType] }
) {
  const pawnType = boardState[`${startX}_${startY}`][1]
  let x, y
  if (pawnType === PawnType.Dame) {
    const dx = endX > startX ? 1 : -1
    const dy = endY > startY ? 1 : -1
    x = startX + dx
    y = startY + dy
    while (x !== endX && y !== endY) {
      if (boardState[`${x}_${y}`][0] !== Color.Empty) {
        break
      }
      x += dx
      y += dy
    }
  } else {
    y = startY > endY ? startY - 1 : startY + 1
    x = boardState[`${startX}_${startY}`][0] === Color.Black ? startX + 1 : startX - 1
  }

  boardState[`${x}_${y}`] = [Color.Empty, PawnType.Empty]
  console.log(`[beat] Removing pawn from ${x}_${y}`)
  document
    .querySelector(`[id='${x}_${y}'][class*='pawn'], [id='${x}_${y}'][class*='dame']`)
    ?.remove()
  setState(boardState)
}

function drop(event: DragEvent) {
  let boardState = getState() as { [key: string]: [Color, PawnType] }

  event.preventDefault()
  const draggableElementId = event.dataTransfer!.getData('id')
  const targetElementId = (event.currentTarget as HTMLElement).getAttribute('id')

  const [startX, startY] = draggableElementId.split('_').map((id) => Number(id))
  const [endX, endY] = targetElementId!.split('_').map((id) => Number(id))

  const elementOnTarget = document.querySelector(
    `[id='${targetElementId}'][class*='pawn'], [id='${targetElementId}'][class*='dame']`
  )
  const canBeat = checkersRules.canBeat(startX, startY, endX, endY, boardState)

  if (
    (checkersRules.canMove(startX, startY, endX, endY, boardState) && !elementOnTarget) ||
    canBeat
  ) {
    if (canBeat) {
      beat(startX, startY, endX, endY, boardState)
    }

    // Refresh board state after beat
    boardState = getState() as { [key: string]: [Color, PawnType] }
    boardState[targetElementId!] = boardState[draggableElementId]
    boardState[draggableElementId] = [Color.Empty, PawnType.Empty]

    const elementToMove = document.querySelector(
      `[id='${draggableElementId}'][class*='pawn'], [id='${draggableElementId}'][class*='dame']`
    ) as HTMLElement
    if (!elementToMove) {
      console.error(`[drop] Draggable element ${draggableElementId} not found in DOM!`)
      return
    }

    // Optimization: only append if parent is different
    const currentTarget = event.currentTarget as HTMLElement
    if (elementToMove.parentElement !== currentTarget) {
      currentTarget.appendChild(elementToMove)
    }
    elementToMove.id = currentTarget.id

    if (checkersRules.canBecomeDame(endX, endY, boardState)) {
      elementToMove.classList.remove('pawn')
      elementToMove.classList.remove(boardState[targetElementId!][1])
      elementToMove.classList.add('dame')
      boardState[targetElementId!][1] = PawnType.Dame
      elementToMove.classList.add(boardState[targetElementId!][1])
      if (boardState[targetElementId!][0] === Color.White) {
        elementToMove.classList.add('White')
      }
      elementToMove.setAttribute('data-testid', 'dame')
    }

    setState(boardState)
    // TODO handle player better
    // TODO fix

    if (canBeat && checkersRules.canAnyBeat(endX, endY, boardState)) {
      console.log(`[drop] Multi-beat available at ${endX}_${endY}, keeping turn.`)
      checkersRules.mustMovePiece = `${endX}_${endY}`
      return
    }

    console.log(`[drop] No multi-beat, switching turn.`)
    //AI move
    if ((window as any).disableAI) {
      console.log(`[drop] AI disabled, switching turn manually.`)
      checkersRules.nextTurn()
      return
    }

    console.log(`[drop] Triggering AI move.`)
    checkersRules.nextTurn()
    Api.healthCheck()
      .then((statusCode) => {
        let boardStateTemp = getState() as { [key: string]: [Color, PawnType] }
        if (statusCode === 200) {
          Api.makeRandomMove(Player.Black, boardStateTemp).then((newBoardState) => {
            //console.log(next_move_json)
            updateBoard(boardStateTemp, newBoardState)
            setState(newBoardState)
            checkersRules.nextTurn()
          })
        }
      })
      .catch(() => {
        // Do nothing, let the turn stay as it is (it's already switched to AI's color)
      })
  }
}
</script>

<style>
.square {
  height: 75px;
  width: 75px;
  position: relative;
}
.squareBlack {
  background-color: #000;
}
.squareWhite {
  background-color: #fff;
}
</style>

<template>
  <div v-bind="testId" :class="cls" :id="x + '_' + y" v-on:drop="drop" v-on:dragover="allowDrop">
    <slot />
  </div>
</template>
