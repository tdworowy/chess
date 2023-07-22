<script setup lang="ts">
import { checkersRules } from '@/rules'
import type { boardStateType } from '@/types'
import { inject } from 'vue'

const testId = { 'data-testid': 'square' }
const classWhite = 'square squareBlack'
const classBlack = ' square squareWhite'

const setState: boardStateType = <boardStateType>inject('setState')
const getState = <
  () => {
    [key: string]: string
  }
>inject('getState')

const props = defineProps<{
  i: number
  j: number
  color: String
}>()
const cls: String = props.color === 'black' ? classBlack : classWhite

function allowDrop(event: DragEvent) {
  event.preventDefault()
}

function drop(event: DragEvent) {
  const { target } = event
  event.preventDefault()
  const draggableElementId = event.dataTransfer!.getData('id')
  const targetElementId = (<HTMLElement>target).getAttribute('id')

  const [startX, startY] = draggableElementId.split('_').map((id) => Number(id))
  const [endX, endY] = targetElementId!.split('_').map((id) => Number(id))

  const element = document.querySelector(`[id='${targetElementId}'][class*='pawn']`)

  if (checkersRules.canMove(startX, startY, endX, endY) && !element) {
    const element = document.querySelector(
      `[id='${draggableElementId}'][class*='pawn']`
    ) as HTMLElement
    ;(<HTMLElement>target)!.appendChild(element)
    element!.id = (<HTMLElement>target)!.id

    let boardState = getState()
    setState(boardState) // TODO change state
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
  <div v-bind="testId" :class="cls" :id="i + '_' + j" v-on:drop="drop" v-on:dragover="allowDrop" />
</template>
