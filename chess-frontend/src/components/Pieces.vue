<script setup lang="ts">
import { pieceColorCondiion } from './piecesUtils'

const props = defineProps<{
  x: number
  y: number
}>()

const color = pieceColorCondiion(props.x, props.y)

const pawnDark = 'pawn pawnDark'
const pawnLight = 'pawn pawnLight'

const testId = { 'data-testid': 'pawn' }

function drag(event: DragEvent) {
  const { target } = event
  event.dataTransfer?.setData('id', (<HTMLElement>target)?.id)
}
</script>

<style>
.pawn {
  height: 50px;
  width: 50px;
  border-radius: 50%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: inline-block;
  position: absolute;
}
.pawnDark {
  background-color: #960a0a;
}
.pawnLight {
  background-color: #67c964;
}
</style>

<template>
  <div
    v-if="color === 'Dark'"
    :id="x + '_' + y"
    :class="pawnDark"
    v-bind="testId"
    draggable="true"
    v-on:dragstart="drag"
  ></div>
  <div
    v-if="color === 'Light'"
    :id="x + '_' + y"
    :class="pawnLight"
    v-bind="testId"
    draggable="true"
    v-on:dragstart="drag"
  ></div>
</template>
