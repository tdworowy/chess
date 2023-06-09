<script setup lang="ts">
import Pieces from './Pieces.vue'
import Square from './Square.vue'

const black = 'black'
const white = 'white'
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
          <div v-if="i % 2 == 0">
            <Square :i="i" :j="j" :color="black" />
            <Pieces :x="i" :y="j" />
          </div>
          <div v-else>
            <Square :i="i" :j="j" :color="white" />
            <Pieces :x="i" :y="j" />
          </div>
        </div>
        <div v-else>
          <div v-if="i % 2 == 0">
            <Square :i="i" :j="j" :color="white" />
            <Pieces :x="i" :y="j" />
          </div>
          <div v-else>
            <Square :i="i" :j="j" :color="black" />
            <Pieces :x="i" :y="j" />
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>
