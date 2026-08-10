<script setup lang="ts">
import { Color, PawnType } from '@/types'
import { pieceColorCondition } from '@/piecesUtils'

const props = defineProps<{
  x: number
  y: number
}>()

const color = pieceColorCondition(props.x, props.y)[0]

const pawnBlack = `pawn ${PawnType.PawnBlack}`
const pawnWhite = `pawn ${PawnType.PawnWhite}`

const testId = { 'data-testid': 'pawn' }

function drag(event: DragEvent) {
  const target = event.target as HTMLElement
  console.log(`[dragstart] Piece ID: ${target.id}`)
  event.dataTransfer?.setData('id', target.id)
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
  }
}
</script>

<style>
.pawn,
.dame {
  height: 50px;
  width: 50px;
  border-radius: 50%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: inline-block;
  position: absolute;
}
.PawnBlack {
  background-color: #960a0a;
}
.PawnWhite {
  background-color: #67c964;
}
.dame.Dame {
  background-color: #270303;
  border: 2px solid gold;
}
.dame.Dame.White {
  background-color: #163315;
  border: 2px solid gold;
}
</style>

<template>
  <div
    v-if="color === Color.Black"
    :id="x + '_' + y"
    :class="pawnBlack"
    v-bind="testId"
    draggable="true"
    v-on:dragstart="drag"
  ></div>
  <div
    v-if="color === Color.White"
    :id="x + '_' + y"
    :class="pawnWhite"
    v-bind="testId"
    draggable="true"
    v-on:dragstart="drag"
  ></div>
</template>
