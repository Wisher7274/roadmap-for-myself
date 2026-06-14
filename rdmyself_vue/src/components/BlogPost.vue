<template>
  <main class="content">
    <h1>文章详情</h1>
    <p>正在阅读第 {{ id }} 篇文章</p>
    <p>这tm是篇文章</p>
    <p><router-link to="/blog">← 返回列表</router-link></p>
  </main>
</template>

<script setup>
import { useRoute } from 'vue-router'
const route = useRoute()
const id = route.params.id   // 从 URL 取出动态参数
</script>