<script setup>
import { ref } from 'vue'
import router from './router';
const mobileOpen = ref(false)
const toggleMobile = () => {
mobileOpen.value = !mobileOpen.value
}
</script>
<template>
  <div class="page">
    <header class="header">
      <div class="header-inner">
        <a class="logo" href="/">Wisher7274's blog & roadmap & ♡</a>

        <nav class="nav" :data-open="mobileOpen">
          <RouterLink to="/">首页</RouterLink>
          <RouterLink to ="/blog">博客</RouterLink>
          <a href="https://github.com/Wisher7274/roadmap-for-myself" target="_blank" rel="noopener">GitHub</a>
        </nav>

        <button class="menu-btn" @click="toggleMobile" aria-label="menu">
          <span class="bar"></span>
          <span class="bar"></span>
          <span class="bar"></span>
        </button>
      </div>
      <div class="header-bottom" v-if="mobileOpen">
        <nav class="nav-mobile">
          <RouterLink to="/">首页</RouterLink>
          <RouterLink to ="/blog">博客</RouterLink>
          <a href="https://github.com/Wisher7274/roadmap-for-myself" target="_blank" rel="noopener">GitHub</a>
        </nav>
      </div>
    </header>

    <router-view />

    <footer class="footer">
      <span>&copy; 2026 Jacob Lee. All rights reserved.</span>
      <p>Made by Wisher7274 with 💗</p>
    </footer>
  </div>
</template>

<style>
.page {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}
.content {
  flex: 1;
  max-width: 50rem;
  margin: 0 auto;
  text-align: left;
  padding: 2rem;
}
.header {
  position: sticky;
  top: 0;
  z-index: 40;
  /* 渐变 + 轻微毛玻璃效果 */
  background: linear-gradient(90deg, rgba(34,34,34,0.95), rgba(48,48,48,0.9));
  color: var(--vt-c-white);
  padding: 0.5rem 1rem;
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
  box-shadow: 0 4px 18px rgba(2,6,23,0.35);
}

.header-inner {
  display: flex;
  align-items: center;
  gap: 1rem;
  max-width: 1100px;
  margin: 0 auto;
}

.logo {
  font-weight: 700;
  color: var(--vt-c-white);
  text-decoration: none;
  font-size: 1.05rem;
  padding: 0.25rem 0.5rem;
}

.nav {
  margin-left: auto;
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.nav a {
  color: var(--vt-c-white);
  text-decoration: none;
  padding: 0.45rem 0.6rem;
  border-radius: 6px;
  transition: background-color 0.18s, transform 0.12s;
}

.nav a:hover {
  background: rgba(255,255,255,0.06);
  transform: translateY(-1px);
}

.menu-btn {
  display: none;
  background: transparent;
  border: none;
  margin-left: 0.5rem;
  padding: 0.3rem;
  cursor: pointer;
}

.menu-btn .bar {
  display: block;
  width: 20px;
  height: 2px;
  background: var(--vt-c-white);
  margin: 3px 0;
  border-radius: 2px;
}

.header-bottom {
  background: linear-gradient(180deg, rgba(0,0,0,0.03), transparent);
  padding: 0.5rem 1rem 0.75rem;
}

.nav-mobile {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  max-width: 1100px;
  margin: 0.25rem auto 0;
}

.nav-mobile a {
  padding: 0.5rem 0.75rem;
  border-radius: 6px;
  color: var(--vt-c-white);
  text-decoration: none;
  background: rgba(255,255,255,0.02);
}

@media (max-width: 720px) {
  .nav { display: none; }
  .menu-btn { display: inline-block; }
}

@media (min-width: 721px) {
  .header-bottom { display: none; }
}
.footer {
  background-color: #333;
  color: #fff;
  /* 上下 + 左右留白，文字不贴底栏边缘 */
  padding: 1rem 1.5rem;
  text-align: center;
  font-size: 0.875rem;
  opacity: 0.8;
}
</style>