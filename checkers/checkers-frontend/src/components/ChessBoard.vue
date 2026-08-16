<script setup lang="ts">
import { inject, onMounted } from 'vue'
import Pieces from './Pieces.vue'
import Square from './Square.vue'
import { pieceColorCondition } from '@/piecesUtils'
import { Color, PawnType, type boardStateType } from '@/types'

let boardArray: { [key: string]: [Color, PawnType] } = {}

for (let i = 1; i <= 8; i++) {
  for (let j = 1; j <= 8; j++) {
    boardArray[`${i}_${j}`] = pieceColorCondition(i, j)
  }
}

const setState = inject('setState') as boardStateType
onMounted(() => {
  setState(boardArray)
})
</script>

<style>
.container {
  justify-content: center;
  align-items: center;
}

ul.no-bullets {
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
    <ul v-for="i in 8" :key="'row-' + i" class="no-bullets">
      <li v-for="j in 8" :key="'col-' + i + '-' + j">
        <Square :x="i" :y="j" :color="(i + j) % 2 === 0 ? Color.White : Color.Black">
          <Pieces :x="i" :y="j" />
        </Square>
      </li>
    </ul>
  </div>
</template>
