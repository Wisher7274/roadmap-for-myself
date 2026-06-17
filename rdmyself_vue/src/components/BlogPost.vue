<template>
  <main class="max-w-6xl p-8 mx-auto">
    <article class="bg-blue-100 dark:bg-blue-950 shadow-md p-8">
      <!-- 标题和日期可以用路由参数，但更推荐从 Markdown 里分离，这里先用简单方式 -->
      <h1 class="mb-3 text-xl font-bold dark:text-blue-100">文章 #{{ id }}</h1>
      <!-- v-html 渲染 Markdown 生成的 HTML -->
      <p v-if="loading">加载中</p>
      <div class="dark:text-blue-100" v-html="html"></div>
      <p class="mt-3 text-xs text-gray-500"><RouterLink to="/roadmap-for-myself/blog">← 返回列表</RouterLink></p>
    </article>
  </main>
</template>

<script setup>
import { ref, watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { marked } from 'marked'

const route = useRoute()
const id = route.params.id
const html = ref('')
const loading = ref(true)

watchEffect(async () => {
  if (!id) return
  loading.value = true
  try {
    const res = await fetch(`${import.meta.env.BASE_URL}posts/${id}.md`)  // 从 public 目录获取
    if (!res.ok) throw new Error('Not found')
    const raw = await res.text()
    html.value = marked(raw)
  } catch (e) {
    html.value = '<p>文章未找到</p>'
  } finally {
    loading.value = false
  }
})
</script>