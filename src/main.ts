import { createApp } from "vue"
import PrimeVue from 'primevue/config';
import { createPinia } from 'pinia'
import App from "./App.vue"
import router from './routes'

import 'primeicons/primeicons.css'

import "./assets/css/index.css"
import 'primevue/resources/themes/lara-light-green/theme.css'


const app = createApp(App);
const pinia = createPinia()

app.use(pinia)
app.use(router);
app.use(PrimeVue);


app.mount("#root")
