import { createRouter, createWebHistory } from 'vue-router'
import VideoList from './components/VideoList.vue'
import VideoPlayer from './components/VideoPlayer.vue'

const routes = [
  { path: '/', name: 'home', component: VideoList },
  { path: '/player/:id', name: 'player', component: VideoPlayer }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router