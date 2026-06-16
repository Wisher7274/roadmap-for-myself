<script setup>
import { ref } from 'vue'
const posts = ref([
    { id: 'another-me', title: '世界上的另一个我', summary: '呜呜呜我的绿点' ,date: '2026.6.13' },
    { id: 'no-code-no-life', title: '全体都有，准备coding', summary: 'no code no life' ,date: '2026.6.14' },
    { id: 'rust-yes', title: '文档还得是看英文的', summary: 'rust好啊' ,date: '2026.6.16' },
])
</script>

<template>
    <main class="content">
        <section class="hbox">
            <section class="intro">
                <h1>Wisher7274's blogs</h1>
                <img src="../assets/avatar.png" alt="avator">
            </section>
            <div>
                <article v-for="post in posts" :key="post.id" class="card">
                <h2><RouterLink :to ="`/roadmap-for-myself/blog/${post.id}`">{{ post.title }}</RouterLink></h2>
                <time>{{ post.date }}</time>
                <p>{{ post.summary }}</p>
                </article>
            </div>
        </section>
    </main>
</template>

<style scoped>
/* 左右两栏的主容器 */
.hbox {
  display: flex;           /* 横向排列子元素 */
  gap: 2rem;               /* 左右两栏间距 */
  align-items: flex-start; /* 顶部对齐（避免高度不一致时错位） */
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  margin: 1rem;
}

/* 左侧：标题 + 头像 */
.intro {
  width: 200px;            /* 固定左栏宽度，不伸缩 */
  flex-shrink: 0;          /* 容器空间不足时也不缩小 */
  text-align: center;      /* 内容居中（标题、头像） */
  margin: 1rem;
}

.intro h1 {
  font-size: 1.5rem;
  margin-bottom: 1rem;
}

.intro img {
  width: 100%;
  max-width: 120px;
  border-radius: 50%;      /* 圆形头像，视喜好可改 */
  margin-top: 0.5rem;
}

/* 右侧：文章列表（自动占满剩余空间） */
.hbox > div {
  flex: 1;                 /* 撑满剩余宽度 */
  min-width: 0;            /* 防止内容溢出撑破容器（flex 子元素默认 min-width: auto） */
}

/* 文章卡片 */
.card {
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 1.25rem;
  margin-bottom: 1rem;
  margin: 1rem;
}

.card h2 {
  margin: 0 0 0.5rem;
  font-size: 1.25rem;
}

.card time {
  display: block;
  color: var(--vt-c-text-secondary);
  font-size: 0.875rem;
  margin-bottom: 0.75rem;
}

.card p {
  margin: 0;
  line-height: 1.6;
}

/* 可选：博客页外层 content 放宽一些 */
.content {
  min-width: 80rem;
  max-width: 100rem;        /* 原先 50rem，拓宽以容纳双栏 */
}
</style>