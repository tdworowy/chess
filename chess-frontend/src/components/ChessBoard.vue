<script setup lang="ts">
import Pieces from './Pieces.vue'

const testId = { 'data-testid': 'square' }
const classWhite = 'square squareBlack'
const classBlack = ' square squareWhite'
</script>

<style>
.container {
  justify-content: center;
  align-items: center;
}
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

ul.no-bullets {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
li {
  display: inline-block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
</style>

<template>
  <div class="container">
  <ul v-for="i in 8" class="no-bullets">
    <li v-for="j in 8">
      <div v-if="j % 2 == 0">
        <div v-if="i % 2 == 0" v-bind="testId" :class="classBlack" :id="i + '_' + j">
          <Pieces :x="i" :y="j" />
        </div>
        <div v-else v-bind="testId" :class="classWhite" :id="i + '_' + j">
          <Pieces :x="i" :y="j" />
        </div>
      </div>
      <div v-else>
        <div v-if="i % 2 == 0" v-bind="testId" :class="classWhite" :id="i + '_' + j">
          <Pieces :x="i" :y="j" />
        </div>
        <div v-else v-bind="testId" :class="classBlack" :id="i + '_' + j">
          <Pieces :x="i" :y="j" />
        </div>
      </div>
    </li>
  </ul>
</div>
</template>
