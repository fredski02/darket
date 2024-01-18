import { createRouter, createWebHistory } from 'vue-router'

import Home from './pages/Home.vue'
import ProductSingle from './pages/ProductSingle.vue';
import CreateProduct from './pages/CreateProduct.vue';
import ManageProducts from './pages/ManageProducts.vue';

const routes = [
    { path: '/', name: 'home', component: Home },
    { path: '/product/:id', name: 'product-single', component: ProductSingle },
    { path: '/create-product', name: 'product-create', component: CreateProduct },
    { path: '/manage-products', name: 'products-manage', component: ManageProducts },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router;