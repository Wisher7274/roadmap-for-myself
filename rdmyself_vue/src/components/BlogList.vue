<script setup>
import { ref } from 'vue'
const posts = ref([
    { id: 'another-me', title: '世界上的另一个我', summary: '呜呜呜我的绿点' ,date: '2026.6.13' },
    { id: 'no-code-no-life', title: '全体都有，准备coding', summary: 'no code no life' ,date: '2026.6.14' },
    { id: 'rust-yes', title: '文档还得是看英文的', summary: 'rust好啊' ,date: '2026.6.16' },
    { id: 'twcss-yes', title: 'Tailwind CSS的恩情还不完', summary: '😭😭\/\/\/\/' ,date: '2026.6.1' },
])
</script>

<template>
    <main class="max-w-6xl mx-auto flex md:flex-row items-start px-5 py-7 gap-6">
        <section class="bg-white dark:bg-cyan-950 rounded-lg shadow p-6 min-w-2xs space-y-2">
          <h2 class="text-xl font-bold dark:text-gray-300">📚 博客文章</h2>
          <p class="text text-sm text-gray-600 hover:text-cyan-300 dark:text-gray-300 transition-colors"><RouterLink to ="/roadmap-for-myself/">返回主页</RouterLink></p>
        </section>
          <div class="space-y-6 md:flex-row flex-1 gap-6">
              <article v-for="post in posts" :key="post.id" class="bg-white dark:bg-cyan-950 rounded-lg shadow hover:shadow-md transition-shadow p-6">
              <h2 class="font-semibold text-lg text-blue-900 dark:text-gray-300 hover:text-blue-600 transition-colors"><RouterLink :to ="`/roadmap-for-myself/blog/${post.id}`">{{ post.title }}</RouterLink></h2>
              <time class="text-xs text-gray-400">{{ post.date }}</time>
              <p class="test-sm text-cyan-800 dark:text-gray-300 italic">{{ post.summary }}</p>
              </article>
          </div>
    </main>
</template>

