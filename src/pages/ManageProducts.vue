<template>
    <div class="container mx-auto">
        Manage my products
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useCanister } from "@connect2ic/vue"
import { onMounted, ref } from 'vue';

const route = useRoute()
const [backendService] = useCanister("backend")



</script>

<style scoped></style>