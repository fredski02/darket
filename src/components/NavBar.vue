<template>
    <div class="flex flex-row align-center items-center w-full justify-between pr-2 pl-4 py-2 bg-rose-400">
        <router-link to="/">
            <strong>DItem</strong>
        </router-link>
        <div class="items-center flex gap-x-2">
            <p>{{ principalShort }}</p>
            <ConnectButton />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ConnectButton } from "@connect2ic/vue"
import { useConnect } from "@connect2ic/vue"
import { computed } from "vue";

const {principal } = useConnect();

const principalShort = computed(() => {
    let p = principal.value?.slice(0, 5)
    return `${p ? p + '...' : '' }`
})

</script>

<style scoped></style>