<script setup lang="ts">
import NavBar from './components/NavBar.vue'
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
    providerUrl : `http://127.0.0.1:4943/?canisterId=be2us-64aaa-aaaaa-qaabq-cai`,
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
    <div>
      <router-view />
    </div>
  </Connect2ICProvider>
</template>

<style></style>
