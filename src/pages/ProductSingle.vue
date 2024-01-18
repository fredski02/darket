<template>
    <div class="container mx-auto">
        <Card v-if="productData" class="mt-8" :pt="cardClasses">
            <template #title>
                <div class="flex justify-between items-center">
                    <span>{{ productData.name }}</span>
                    <Button :icon="loading ? 'pi pi-spinner pi-spin' : 'pi pi-cart-plus'" rounded
                        :label="`Add to cart${numInCart ? '(' + numInCart + ')' : ''}`" @click="() => addToCart()"></Button>
                </div>
            </template>
            <template #content>
                <div class="flex gap-x-4">
                    <div class="w-1/3">
                        <img alt="" class="rounded"
                            src="https://t2.gstatic.com/licensed-image?q=tbn:ANd9GcQOO0X7mMnoYz-e9Zdc6Pe6Wz7Ow1DcvhEiaex5aSv6QJDoCtcooqA7UUbjrphvjlIc" />
                    </div>
                    <div class="w-2/3">
                        <Panel toggleable>
                            <template #header>
                                <div class="flex items-center gap-2">
                                    <Avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png"
                                        size="large" shape="circle" />
                                    <span class="font-bold">Amy Elsner</span>

                                </div>
                            </template>
                            <p class="m-0">
                                {{ productData.description }}
                            </p>
                            <template #footer>
                                <div class="flex flex-wrap items-center justify-between gap-1">
                                    <div class="flex items-center gap-0">
                                        <Button icon="pi pi-user" rounded text></Button>
                                        <Button icon="pi pi-bookmark" severity="secondary" rounded text></Button>
                                    </div>
                                    <span class="p-text-secondary">Updated 2 hours ago</span>
                                </div>
                            </template>
                        </Panel>
                    </div>
                </div>
            </template>
        </Card>
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useCanister } from "@connect2ic/vue"
import { computed, onMounted, ref } from 'vue';
import useEventBus from "../composables/useEmitter";
const route = useRoute()
const itemId = route.params.id
const [backend] = useCanister("backend")
const eventBus = useEventBus()

const productData = ref();
const cartItems = ref([]);
const loading = ref(false);
const cardClasses = { root: { class: `shadow-none border border-solid border-grey` } }

const addToCart = async () => {
    const id = parseInt(productData.value.id)
    try {
        loading.value = true
        const data = await backend.value.add_to_cart(id);
        eventBus.emit('should-fetch-cart')
        await fetchCartItems();
    } catch (e) {
        console.log(e)
    }
    loading.value = false;
}

const fetchCartItems = async () => {
    loading.value = true;
    try {
        const data = await backend.value.get_cart();
        cartItems.value = data;
    } catch (e) {
        console.log(e)
    }
    loading.value = false;
}

const numInCart = computed(() => {
    const id = parseInt(productData.value.id)
    return cartItems.value.filter((item) => parseInt(item.id) === id).length
})


onMounted(async () => {
    const data = await backend.value.get_product(Number(itemId))
    productData.value = data.length ? data[0] : null
    fetchCartItems();
})

</script>

<style scoped></style>