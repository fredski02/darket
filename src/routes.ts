import { createRouter, createWebHistory } from 'vue-router'

import Home from './pages/Home.vue'
import ProductSingle from './pages/ProductSingle.vue';

const routes = [
    { path: '/', name: 'home', component: Home },
    { path: '/product/:id', name: 'about', component: ProductSingle },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router;