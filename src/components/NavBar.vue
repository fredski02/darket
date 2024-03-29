<template>
    <div class="bg-rose-100 border-b border-rose-200 w-full fixed z-50 px-2 py-4">
        <div class="flex flex-row align-center items-center w-full justify-between  container mx-auto">
            <div>
                <router-link to="/">
                    <strong>D A R K E T</strong>
                </router-link>
                <router-link to="/">
                    | Browse market
                </router-link>
            </div>
            <div class="items-center flex gap-x-2 item-center">
                <div v-if="isLoggedIn" :style="{ position: 'relative', height: '46px', width: '50px' }">
                    <SpeedDial severity="danger" :model="userMenuItems" direction="down" showIcon="pi pi-bars"
                        buttonClass="p-button-outlined h-12 w-12 text-rose-500"
                        :tooltipOptions="{ position: 'left', event: 'hover' }" />
                </div>
                <router-link to="/cart" v-if="isLoggedIn">
                    <Button :label="numInCart ? numInCart + '' : ''" icon="pi pi-shopping-cart" severity="danger" rounded outlined />
                </router-link>
                <ConnectButton />
                <span>{{ principalShort }}</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ConnectButton } from "@connect2ic/vue"
import { useConnect } from "@connect2ic/vue"
import { computed, onMounted, ref, toRefs, watchEffect } from "vue";
import { useRouter } from "vue-router";
import SpeedDial from "primevue/speeddial";
import { useCanister } from "@connect2ic/vue"
import useEventBus from "../composables/useEmitter";

const router = useRouter();
const { principal } = toRefs(useConnect());

const [backend] = useCanister("backend")
const eventBus = useEventBus();
const cartItems = ref([]);

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

const principalShort = computed(() => {
    let p = principal.value?.slice(0, 5)
    return `${p ? p + '...' : ''}`
})

const fetchCartItems = async () => {
    try {
        const data = await backend.value.get_cart();
        cartItems.value = data;
    } catch (e) {
        console.log(e)
    }
}

const numInCart = computed(() => {
    return cartItems.value.length
})

const isLoggedIn = computed(() => !!principal.value)



watchEffect(() => {
  if (backend.value) {
    fetchCartItems()
  }
})

onMounted(() => {
    eventBus.on('should-fetch-cart', () => {
        fetchCartItems()
    })
})


</script>

<style scoped></style>