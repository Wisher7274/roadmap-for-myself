<template>
  <main class="content blog-post">
    <article>
      <!-- 标题和日期可以用路由参数，但更推荐从 Markdown 里分离，这里先用简单方式 -->
      <h1>文章 #{{ id }}</h1>
      <!-- v-html 渲染 Markdown 生成的 HTML -->
      <div class="markdown-body" v-html="html"></div>
      <p class="back"><RouterLink to="/blog">← 返回列表</RouterLink></p>
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
    const res = await fetch(`/posts/${id}.md`)  // 从 public 目录获取
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

<style scoped>
.blog-post {
  max-width: 50rem;           /* 文章阅读舒适宽度 */
  margin: 0 auto;
}

.back {
  margin-top: 2rem;
  text-align: center;
}

/* Markdown 内容样式（基础款，可后续美化） */
.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3) {
  margin-top: 1.5em;
  margin-bottom: 0.5em;
}
.markdown-body :deep(p) {
  line-height: 1.7;
  margin-bottom: 1em;
}
.markdown-body :deep(pre) {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 1rem;
  border-radius: 6px;
  overflow-x: auto;
}
.markdown-body :deep(code) {
  background: var(--color-background-soft);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-size: 0.9em;
}
</style>