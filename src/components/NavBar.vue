<template>
    <div
        class="flex flex-row align-center items-center w-full justify-between pr-2 pl-4 py-2 bg-rose-100 border-b border-rose-200">
        <router-link to="/">
            <strong>D A R K E T</strong>
        </router-link>
        <div class="items-center flex gap-x-2 item-center">
            <div v-if="isLoggedIn" :style="{ position: 'relative', height: '46px', width: '50px' }">
                <SpeedDial severity="danger" :model="userMenuItems" direction="down" showIcon="pi pi-bars"
                     buttonClass="p-button-outlined h-12 w-12 text-rose-500" :tooltipOptions="{ position: 'left' }" />
            </div>
            <router-link to="/manage-cart">
                <Button aria-label="Cart" icon="pi pi-shopping-cart" severity="danger" rounded outlined />
            </router-link>
            <ConnectButton />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ConnectButton } from "@connect2ic/vue"
import { useConnect } from "@connect2ic/vue"
import { computed, ref, toRefs } from "vue";
import { useRouter } from "vue-router";
import SpeedDial from "primevue/speeddial";

const router = useRouter();
const { principal } = toRefs(useConnect());

const userMenuItems = ref([
    {
        label: 'Sell a product',
        icon: 'pi pi-money-bill',
        command: () => {
            router.push('/create-product')
        }
    },
    {
        label: 'My Products',
        icon: 'pi pi-list',
        command: () => {
            router.push('/manage-products')
        }
    },
    {
        label: 'My Account',
        icon: 'pi pi-user',
        command: () => {
            router.push('/user')
        }
    },
])

const isLoggedIn = computed(() => !!principal.value)


</script>

<style scoped></style>