<script setup lang="ts">
import NavBar from './components/NavBar.vue'
import Footer from './components/Footer.vue'
import { createClient } from "@connect2ic/core"
import { InternetIdentity, defaultProviders } from "@connect2ic/core/providers"
import { ConnectDialog, Connect2ICProvider } from "@connect2ic/vue"
import { useCounterStore } from './store/global'
import "@connect2ic/core/style.css"

// import canisters
import * as backend from "./declarations/backend"

const backendService = backend as any
// create a client
const client = createClient({
  canisters: {
    backend: backendService
  },
  providers: [new InternetIdentity({
    providerUrl: `http://be2us-64aaa-aaaaa-qaabq-cai.localhost:4943/`,
    dev: true,
  })],
  globalProviderConfig: {
    dev: import.meta.env.DEV,
  },
})


const counterStore = useCounterStore()
</script>

<template>
  <Connect2ICProvider :client="client">
    <ConnectDialog />
    <NavBar />
    <div class="pt-20 wrapper">
      <router-view v-slot="{ Component, route }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </div>
    <Footer />
  </Connect2ICProvider>
</template>

<style>
.wrapper {
  min-height: 100vh;
  width: 100%;
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
