<script setup lang="ts">
import ChessBoard from './components/ChessBoard.vue'
import { provide, ref } from 'vue'
import type { PawnType } from './types'

const boardState = ref({})

const setState = (newState: { [key: string]: [string, PawnType] }) => {
  boardState.value = newState
}
const getState = () => {
  return boardState.value
}

provide('setState', setState)
provide('getState', getState)
</script>

<template>
  <header></header>

  <main>
    <ChessBoard />
  </main>
</template>
