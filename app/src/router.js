import { createRouter, createWebHistory } from 'vue-router'
import List from './components/list.vue'
import player from './components/player.vue'

const routes = [
    { path: '/', name: 'home', component: List },
    { path: '/player', component: player, props: route => ({ path: route.query.src, type: route.query.type, url: route.query.url }) }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router