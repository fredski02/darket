import { createRouter, createWebHistory } from 'vue-router'

import Home from './pages/Home.vue'
import ProductSingle from './pages/ProductSingle.vue';
import CreateProduct from './pages/CreateProduct.vue';
import ManageProducts from './pages/ManageProducts.vue';

const routes = [
    { path: '/', name: 'home', component: Home, meta : { transition : 'slide-right'} },
    { path: '/product/:id', name: 'product-single', component: ProductSingle, meta : { transition : 'slide-right'} },
    { path: '/create-product', name: 'product-create', component: CreateProduct, meta : { transition : 'slide-right'} },
    { path: '/manage-products', name: 'products-manage', component: ManageProducts, meta : { transition : 'slide-right'} },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router;