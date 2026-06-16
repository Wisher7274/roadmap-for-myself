import { createRouter, createWebHistory } from 'vue-router'
import Home from '../components/Home.vue'
import BlogList from '../components/BlogList.vue'
import BlogPost from '../components/BlogPost.vue'

const routes = [
  { path: '/roadmap-for-myself', name: 'home', component: Home },
  { path: '/roadmap-for-myself/blog', name: 'blog-list', component: BlogList },
  { path: '/roadmap-for-myself/blog/:id', name: 'blog-post', component: BlogPost },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router