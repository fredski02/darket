import { createApp, inject, provide } from "vue"
import PrimeVue from 'primevue/config';
import { createPinia } from 'pinia'
import App from "./App.vue"
import router from './routes'
import mitt from 'mitt';
import DropZone from 'dropzone-vue';

import 'primeicons/primeicons.css'
import "./assets/css/index.css"
import 'primevue/resources/themes/lara-light-green/theme.css'
import 'dropzone-vue/dist/dropzone-vue.common.css';

const emitter = mitt();

const app = createApp(App);
const pinia = createPinia();
app.config.globalProperties.emitter = emitter;
app.use(pinia)
app.use(router);
app.use(PrimeVue);
app.use(DropZone)

app.mount("#root")
