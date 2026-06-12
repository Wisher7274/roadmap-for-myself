<script setup>
// 从 components 文件夹引入主页组件
import Home from './components/Home.vue'
</script>

<template>
  <!-- App.vue 只负责「挂载」当前要显示的页面，具体内容在 Home.vue 里 -->
  <Home />
</template>
