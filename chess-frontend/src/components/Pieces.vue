<script setup lang="ts">
const props = defineProps<{
  x: number
  y: number
}>()
const condition = (props: any) => {
  return (props.y % 2 === 0 && props.x % 2 !== 0) || (props.y % 2 !== 0 && props.x % 2 === 0)
}

const color =
  [1, 2, 3].includes(props.x) && condition(props)
    ? 'Dark'
    : [6, 7, 8].includes(props.x) && condition(props)
    ? 'Light'
    : ''
const pawnDark = 'pawn pawnDark'
const pawnLight = 'pawn pawnLight'

const testId = { 'data-testid': 'pawn' }

function drag(event: DragEvent) {
  const { target } = event
  event.dataTransfer?.setData('id', (<HTMLElement>target)?.id)
    // TODO change pieces id on drag
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
