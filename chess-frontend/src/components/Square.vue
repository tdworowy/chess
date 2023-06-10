<script setup lang="ts">
const testId = { 'data-testid': 'square' }
const classWhite = 'square squareBlack'
const classBlack = ' square squareWhite'

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
  let draggableElementId = event.dataTransfer!.getData('id')
  const element = document.querySelector(
    `[id='${draggableElementId}'][class*='pawn']`
  ) as HTMLElement
  ;(<HTMLElement>target)!.appendChild(element)
    element!.id = (<HTMLElement>target)!.id
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
