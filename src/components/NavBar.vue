<template>
    <div class="flex flex-row align-center items-center w-full justify-between pr-2 pl-4 py-2 bg-rose-400">
        <router-link to="/">
            <strong>DItem</strong>
        </router-link>
        <ConnectButton />
    </div>
</template>

<script setup lang="ts">
import { ConnectButton } from "@connect2ic/vue"


</script>

<style scoped></style>