import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

const routes = [
  { path: '/', name: 'home', component: App },
//   { path: '/player/:id', name: 'player', component: VideoPlayer }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router