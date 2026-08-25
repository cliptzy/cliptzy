import { createApp } from "vue";
import { createPinia } from 'pinia';
import App from "./App.vue";
import router from './router';
import '@fontsource/geist-sans';
import './assets/styles/main.css';

console.log('[Main] Starting Vue app...');
const app = createApp(App);
app.use(createPinia());
app.use(router);

console.log('[Main] Mounting app...');
app.mount("#app");
console.log('[Main] App mounted!');
