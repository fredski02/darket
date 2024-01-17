<template>
    <div class="container mx-auto">
        <Card v-if="productData" class="mt-8" :pt="cardClasses">
            <template #title> 
                <div class="flex justify-between items-center">
                <span>{{ productData.name }}</span> 
                <Button icon="pi pi-cart-plus" rounded label="Add to cart"></Button>
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

                            <!-- <template #icons>
                                <button class="p-panel-header-icon p-link mr-2" @click="panelToggle">
                                    <span class="pi pi-cog"></span>
                                </button>
                                <Menu ref="menu" id="config_menu" :model="items" popup />
                            </template> -->
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
import { onMounted, ref } from 'vue';

const route = useRoute()
const itemId = route.params.id
const [backendService] = useCanister("backend")

const productData = ref();
const cardClasses = { root: { class: `shadow-none border border-solid border-grey` } }

onMounted(async () => {
    const data = await backendService.value.get_product(Number(itemId))
    productData.value = data.length ? data[0] : null
})

</script>

<style scoped></style>