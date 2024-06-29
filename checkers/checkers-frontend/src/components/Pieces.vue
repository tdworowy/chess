<script setup lang="ts">
import { Color, PawnType } from '@/types'
import { pieceColorCondiion } from './../piecesUtils'

const props = defineProps<{
  x: number
  y: number
}>()

const color = pieceColorCondiion(props.x, props.y)[0]

const pawnDark = `pawn ${PawnType.PawnDark}`
const pawnLight = `pawn ${PawnType.PawnLight}`

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
.dame.pawnDark {
  background-color: #270303;
}
.dame.pawnLight {
  background-color: #163315;
}
</style>

<template>
  <div
    v-if="color === Color.Dark"
    :id="x + '_' + y"
    :class="pawnDark"
    v-bind="testId"
    draggable="true"
    v-on:dragstart="drag"
  ></div>
  <div
    v-if="color === Color.Light"
    :id="x + '_' + y"
    :class="pawnLight"
    v-bind="testId"
    draggable="true"
    v-on:dragstart="drag"
  ></div>
</template>
