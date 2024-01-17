<template>
    <div class="flex w-full justify-center items-center py-28 bg-rose-50">
        <div class="center text-center">
            <h1 class="text-4xl mb-2">Buy and Sell your items</h1>
            <p class="mb-6">List your items for just 10 Cents!</p>
            <div class="input-bar flex flex-row items-center justify-center gap-x-2">
                <Button>Sell your items</Button>
                <Button>How it works</Button>
            </div>
        </div>
    </div>
    <div class="container mx-auto">
        <Items></Items>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Items from '../components/Items.vue'

const seachText = ref("");
</script>

<style scoped></style>