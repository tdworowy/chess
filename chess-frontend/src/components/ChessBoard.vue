<script setup lang="ts"></script>

<style>
.squareBlack {
  height: 50px;
  width: 50px;
  background-color: #000;
}
.squareWhite {
  height: 50px;
  width: 50px;
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
  <ul v-for="i in 4" class="no-bullets">
    <li v-for="j in 8">
      <div v-if="j % 2 == 0">
        <div class="squareBlack"></div>
        <div class="squareWhite"></div>
      </div>
      <div v-else>
        <div class="squareWhite"></div>
        <div class="squareBlack"></div>
      </div>
    </li>
  </ul>
</template>
