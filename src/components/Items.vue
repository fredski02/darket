<script setup lang="ts">
import { useCanister } from "@connect2ic/vue"
import { ref, watch, watchEffect } from "vue"
import debounce from 'lodash.debounce';

const [backend] = useCanister("backend")


let products = ref([])
let loading = ref(false);
let searchText = ref("");

const fetchProducts = async (searchString: string = "") => {
  loading.value = true;
  const data = await backend.value.search_products(searchString);
  products.value = data;
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
  if(newVal.length) {
    debouncedSearch(newVal);
  }
})

</script>
<template>
  <div class="py-4">
    <Toolbar class="px-2 py-2">
      <template #start>
        <Button icon="pi pi-plus" severity="secondary" outlined class="mr-2" />
        <Button icon="pi pi-print" class="mr-2" outlined severity="secondary" />
        <Button icon="pi pi-upload" severity="secondary" outlined />
      </template>

      <template #center>
        <span class="p-input-icon-left">
          <i class="pi" :class="loading ? 'pi-spinner pi-spin' : 'pi-search'" />
          <InputText :disabled="loading" v-model="searchText" placeholder="Search"/>
        </span>
      </template>

      <template #end>
        <SplitButton label="Save" icon="pi pi-check" severity="secondary" outlined></SplitButton>
      </template>
    </Toolbar>
  </div>
  <div class="grid grid-cols-4 gap-4" v-if="products && !loading">
    <Card v-for="product in products" :pt="{body : (options) => ({class : ['rounded-b']}) }">
      <template #header>
        <img alt="user header"
        class="rounded-t"
          src="https://t2.gstatic.com/licensed-image?q=tbn:ANd9GcQOO0X7mMnoYz-e9Zdc6Pe6Wz7Ow1DcvhEiaex5aSv6QJDoCtcooqA7UUbjrphvjlIc" />
      </template>
      <template #title> {{ product.name }} </template>
      <template #subtitle> {{ product.price }} ICP </template>
      <template #content>
        <p class="m-0">
          {{ product.id }}
        </p>
      </template>
      <template #footer>
        <router-link :to="`product/${product.id}`">
          <Button label="View" severity="danger"/>
        </router-link>
      </template>
    </Card>
  </div>
  <div v-if="loading" class="grid grid-cols-4 gap-4">
    <Card v-for="i in [0, 1, 2, 3]" :key="i" >
      <template #header>
        <Skeleton height="8rem"></Skeleton>
      </template>
      <template #content>
        <Skeleton width="10rem" class="mb-2"></Skeleton>
        <Skeleton width="5rem" class="mb-2"></Skeleton>
        <Skeleton height="2rem" class="mb-2"></Skeleton>
        <Skeleton width="10rem" height="4rem"></Skeleton>
      </template>
    </Card>
  </div>
  <div v-if="!products.length && !loading" class="items-center justify-center w-full text-center">
    <h2 class="mt-2 mb-2">No search results</h2>
  </div>
</template>
