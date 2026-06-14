import { createRouter, createWebHistory } from 'vue-router'
import Home from '../components/Home.vue'
import BlogList from '../components/BlogList.vue'
import BlogPost from '../components/BlogPost.vue'

const routes = [
  { path: '/', name: 'home', component: Home },
  { path: '/blog', name: 'blog-list', component: BlogList },
  { path: '/blog/:id', name: 'blog-post', component: BlogPost },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router