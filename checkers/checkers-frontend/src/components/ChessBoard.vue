<script setup lang="ts">
import { inject } from 'vue'
import Pieces from './Pieces.vue'
import Square from './Square.vue'
import { pieceColorCondiion } from './../piecesUtils'
import { Color, PawnType, type boardStateType } from '@/types'

let boardArray: { [key: string]: [Color, PawnType] } = {}

for (let i = 1; i <= 8; i++) {
  for (let j = 1; j <= 8; j++) {
    boardArray[`${i}_${j}`] = pieceColorCondiion(i, j)
  }
}

const setState: boardStateType = <boardStateType>inject('setState')
setState(boardArray)
</script>

<style>
.container {
  justify-content: center;
  align-items: center;
}

ul.no-bullets {
  list-style-type: none;
  padding: 0;
  margin: 0;
  list-style: none;
}
li {
  display: inline-block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  vertical-align: top;
}
</style>

<template>
  <div class="container">
    <ul v-for="i in 8" class="no-bullets">
      <li v-for="j in 8">
        <div v-if="j % 2 == 0">
          <div v-if="i % 2 == 0">
            <Square :x="i" :y="j" :color="Color.White" />
            <Pieces :x="i" :y="j" />
          </div>
          <div v-else>
            <Square :x="i" :y="j" :color="Color.Black" />
            <Pieces :x="i" :y="j" />
          </div>
        </div>
        <div v-else>
          <div v-if="i % 2 == 0">
            <Square :x="i" :y="j" :color="Color.Black" />
            <Pieces :x="i" :y="j" />
          </div>
          <div v-else>
            <Square :x="i" :y="j" :color="Color.White" />
            <Pieces :x="i" :y="j" />
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>
