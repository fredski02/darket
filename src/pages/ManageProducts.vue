<template>
    <div>
        <div class="flex w-full justify-center items-center py-28 bg-rose-50">
            <div class="center text-center">
                <h1 class="text-4xl mb-2">Mange your Products</h1>
                <p class="mb-6">You sold 42 items this year</p>
            </div>
        </div>
        <div class="w-2/3 mx-auto pt-6">
            <Card :pt="{ body: (options) => ({ class: ['border rounded'] }) }" class="shadow-none" :disalbed="loading">
                <template #content>
                    <DataTable :value="products" class="px-3">
                        <template #header>
                            <div class="flex flex-wrap align-items-center justify-content-between gap-2">
                                <span class="text-xl text-900 font-bold">Products</span>
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
                        <Column header="Status">
                            <template #body="slotProps">
                                <Tag value="INSTOCK" severity="primary" />
                            </template>
                        </Column>
                        <Column header="Actions">
                            <template #body="slotProps">
                                <Button severity="info" size="small" class="px-2 py-1" :disabled="loading">Edit</Button>
                            </template>
                        </Column>
                        <Column header="Delete">
                            <template #body="slotProps">
                                <Button severity="danger" size="small" class="px-2 py-1" :disabled="loading"
                                    @click="deleteProduct(slotProps.data.id)">Delete</Button>
                            </template>
                        </Column>
                        <!-- <template #footer> In total there are {{ products ? products.length : 0 }} products. </template> -->
                    </DataTable>
                </template>
            </Card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useCanister } from "@connect2ic/vue"
import { onMounted, ref, watch, watchEffect } from 'vue';
import debounce from 'lodash.debounce';

const [backend] = useCanister("backend")


let products = ref([])
let loading = ref(false);
let searchText = ref("");

const fetchProducts = async (searchString: string = "") => {
    loading.value = true;
    const data = await backend.value.get_user_products();
    products.value = data;
    loading.value = false;
}

const deleteProduct = async (id: string) => {
    const productId = parseInt(id);
    if (typeof productId !== 'number') {
        console.log('delete product frontend - error parsing product number');
    }
    loading.value = true
    try {
        const result = await backend.value.delete_product(productId);
        console.log(result);
        fetchProducts();
    } catch (e) {

    }
    loading.value = false;

}

const debouncedSearch = debounce((searchStr: string) => {
    return fetchProducts(searchStr)
}, 400)


watchEffect(() => {
    if (backend.value) {
        fetchProducts();
    }
})

watch(searchText, (newVal, oldVal) => {
    if (newVal.length) {
        debouncedSearch(newVal);
    }
})


</script>

<style scoped></style>