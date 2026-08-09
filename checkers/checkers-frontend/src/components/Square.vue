<script setup lang="ts">
import { Api } from '@/Api'
import { checkersRules } from '@/rules'
import { Color, PawnType, Player, type boardStateType } from '@/types'
import { inject } from 'vue'

const testId = { 'data-testid': 'square' }
const classBlack = 'square squareBlack'
const classWhite = ' square squareWhite'

const setState = inject('setState') as boardStateType
const getState = inject('getState') as () => { [key: string]: [Color, PawnType] }

const props = defineProps<{
  x: number
  y: number
  color: Color
}>()

const cls: String = props.color === Color.Black ? classBlack : classWhite

function allowDrop(event: DragEvent) {
  event.preventDefault()
}

function updateBoard(
  boardState: { [key: string]: [Color, PawnType] },
  newBoardState: { [key: string]: [Color, PawnType] }
) {
  for (const [key, value] of Object.entries(newBoardState)) {
    if (JSON.stringify(boardState[key]) !== JSON.stringify(value)) {
      console.log(`${boardState[key]} !== ${value}`)
      if (value[0] == Color.Empty) {
        document.querySelector(`[id='${key}'][class*='pawn']`)?.remove()
      } else {
        const _class = value[1] === PawnType.Dame ? 'dame' : 'pawn'
        const newPawn = document.createElement('div')
        newPawn.id = key
        newPawn.className = `${_class} ${value[1]}`
        newPawn.setAttribute('data-testid', `${_class}`)
        newPawn.setAttribute('draggable', 'true')

        const square = document.querySelector(`[id='${key}'][class*='square']`)
        square?.appendChild(newPawn)
      }
    }
  }
}
// TODO handle DAME
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
  document.querySelector(`[id='${x}_${y}'][class*='pawn']`)?.remove()
  setState(boardState)
}

function drop(event: DragEvent) {
  let boardState = getState() as { [key: string]: [Color, PawnType] }

  const { target } = event
  event.preventDefault()
  const draggableElementId = event.dataTransfer!.getData('id')
  const targetElementId = (target as HTMLElement).getAttribute('id')

  const [startX, startY] = draggableElementId.split('_').map((id) => Number(id))
  const [endX, endY] = targetElementId!.split('_').map((id) => Number(id))

  const element = document.querySelector(`[id='${targetElementId}'][class*='pawn']`)
  const canBeat = checkersRules.canBeat(startX, startY, endX, endY, boardState)

  if ((checkersRules.canMove(startX, startY, endX, endY, boardState) && !element) || canBeat) {
    if (canBeat) {
      beat(startX, startY, endX, endY, boardState)
    }

    boardState[targetElementId!] = boardState[draggableElementId]
    boardState[draggableElementId] = [Color.Empty, PawnType.Empty]

    const element = document.querySelector(
      `[id='${draggableElementId}'][class*='pawn']`
    ) as HTMLElement
    ;(target as HTMLElement)!.appendChild(element)
    element!.id = (target as HTMLElement)!.id

    if (checkersRules.canBecomeDame(endX, endY, boardState)) {
      element.classList.add('dame')
      boardState[targetElementId!][1] = PawnType.Dame
    }

    setState(boardState)
    // TODO handle player better
    // TODO fix

    //AI move
    checkersRules.nextTurn()
    Api.healtCheck().then((statusCode) => {
      let boardStateTemp = getState() as { [key: string]: [Color, PawnType] }
      if (statusCode === 200) {
        Api.makeRandomMove(Player.Black, boardStateTemp).then((newBoardState) => {
          //console.log(next_move_json)
          updateBoard(boardStateTemp, newBoardState)
          setState(newBoardState)
        })
        checkersRules.nextTurn()
      }
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
  <div v-bind="testId" :class="cls" :id="x + '_' + y" v-on:drop="drop" v-on:dragover="allowDrop" />
</template>
