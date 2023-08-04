<script setup lang="ts">
import { checkersRules } from '@/rules'
import { Color, type boardStateType } from '@/types'
import { inject } from 'vue'

const testId = { 'data-testid': 'square' }
const classBlack = 'square squareBlack'
const classWhite = ' square squareWhite'

const setState: boardStateType = <boardStateType>inject('setState')
const getState = <
  () => {
    [key: string]: string
  }
>inject('getState')

const props = defineProps<{
  x: number
  y: number
  color: Color
}>()
const cls: String = props.color === Color.Dark ? classBlack : classWhite

function allowDrop(event: DragEvent) {
  event.preventDefault()
}

function beat(startX: number, startY: number, endY: number, boardState: { [key: string]: string }) {
  const color = boardState[`${startX}_${startY}`]
  const y = startY > endY ? startY - 1 : startY + 1
  const x = color === Color.Dark ? startX + 1 : startX - 1

  boardState[`${x}_${y}`] = ''
  document.querySelector(`[id='${x}_${y}'][class*='pawn']`)?.remove()
  setState(boardState)
}

function drop(event: DragEvent) {
  let boardState = getState()

  const { target } = event
  event.preventDefault()
  const draggableElementId = event.dataTransfer!.getData('id')
  const targetElementId = (<HTMLElement>target).getAttribute('id')

  const [startX, startY] = draggableElementId.split('_').map((id) => Number(id))
  const [endX, endY] = targetElementId!.split('_').map((id) => Number(id))

  const element = document.querySelector(`[id='${targetElementId}'][class*='pawn']`)
  const canBeat = checkersRules.canBeat(startX, startY, endX, endY, boardState)

  if ((checkersRules.canMove(startX, startY, endX, endY, boardState) && !element) || canBeat) {
    if (canBeat) {
      beat(startX, startY, endY, boardState)
    }

    boardState[targetElementId!] = boardState[draggableElementId]
    boardState[draggableElementId] = ''

    const element = document.querySelector(
      `[id='${draggableElementId}'][class*='pawn']`
    ) as HTMLElement
    ;(<HTMLElement>target)!.appendChild(element)
    element!.id = (<HTMLElement>target)!.id
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
