<template>
    <div>
        <div class="flex w-full justify-center items-center py-28 bg-rose-50">
            <div class="center text-center">
                <h1 class="text-4xl mb-2">Cart <i class="pi pi-shopping-cart" style="font-size: 2.5rem"></i></h1>
                <p class="mb-6">Pay with over 20 different Cryptos coins.</p>
            </div>
        </div>
        <div class="w-2/3 mx-auto pt-6">
            <Card :pt="{ body: (options) => ({ class: ['border rounded'] }) }" class="shadow-none" :disabled="loading" showGridlines 
                v-if="cartItems.length">
                <template #content>
                    <DataTable :value="cartItems" class="px-3">
                        <template #header>
                            <div class="flex flex-wrap align-items-center justify-content-between gap-2">
                                <span class="text-xl text-900 font-bold">Cart items</span>
                            </div>
                        </template>
                        <Column field="id" header="ID"></Column>
                        <Column field="id" header="Link">
                            <template #body="slotProps">
                                <router-link :to="`/product/${slotProps.data.id}`">
                                    <Button severity="info" size="small" class="px-2 py-1">View</Button>
                                </router-link>
                            </template>
                        </Column>
                        <Column field="name" header="Name"></Column>
                        <Column field="price" header="Price"></Column>
                        <Column header="Delete">
                            <template #body="slotProps">
                                <Button severity="danger" size="small" class="px-2 py-1" :disabled="loading"
                                    @click="removeCartItem(slotProps.data.id)">Delete</Button>
                            </template>
                        </Column>
                        <!-- <template #footer> In total there are {{ products ? products.length : 0 }} products. </template> -->
                    </DataTable>
                </template>
            </Card>
            <Card :pt="{ body: (options) => ({ class: ['border rounded'] }) }" v-if="!cartItems.length">
                <template #content>
                    <div class="flex flex-col items-center">
                        <p class="mb-2">You have nothing in your cart</p>
                        <router-link to="/">
                            <Button>Find some products</Button>
                        </router-link>
                    </div>
                </template>
            </Card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useCanister } from "@connect2ic/vue"
import { ref, watchEffect } from 'vue';
import useEventBus from "../composables/useEmitter";

const [backend] = useCanister("backend")
const eventBus = useEventBus();

let cartItems = ref([])
let loading = ref(false);


const fetchCartItems = async () => {
    loading.value = true;
    const data = await backend.value.get_cart();
    cartItems.value = data;
    loading.value = false;
}


const addToCart = async (productId: number) => {
    loading.value = true;
    try {
        const data = await backend.value.add_to_cart(productId);
        await fetchCartItems();
        eventBus.emit('should-fetch-cart')
    } catch (e) {
        console.log(e)
    }
    loading.value = false;
}

const removeCartItem = async (productId: number) => {
    loading.value = true;
    try {
        const data = await backend.value.remove_from_cart(productId);
        await fetchCartItems();
        eventBus.emit('should-fetch-cart')
    } catch (e) {
        console.log(e)
    }
    loading.value = false;
}

watchEffect(() => {
    if (backend.value) {
        fetchCartItems();
    }
})

</script>

<style scoped></style>