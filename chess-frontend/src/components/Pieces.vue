<script setup lang="ts">
const props = defineProps<{
  x: number
  y: number
}>()

const color = [1, 2, 3].includes(props.x) ? 'black' : [6, 7, 8].includes(props.x) ? 'white' : ''
const pawnBlack = 'pawn pawnBlack'
const pawnWhite = 'pawn pawnWhite'

const testId = { 'data-testid': 'pawn' }
</script>

<style>
.pawn {
  height: 50px;
  width: 50px;
  border-radius: 50%;
  display: inline-block;
}
.pawnBlack {
  background-color: #000;
}
.pawnWhite {
  background-color: #fff;
}
</style>

<template>
  <div v-if="color === 'black'" :id="x + '_' + y" :class="pawnBlack" v-bind="testId"></div>
  <div v-if="color === 'white'" :id="x + '_' + y" :class="pawnWhite" v-bind="testId"></div>
</template>
