import { createApp } from "vue";
import { createPinia } from 'pinia';
import App from "./App.vue";
import router from './router';
import '@fontsource/geist-sans';
import './assets/styles/main.css';

import BaseButton from './components/BaseButton.vue';
import SpatialInput from './components/SpatialInput.vue';
import RangeSlider from './components/RangeSlider.vue';

import { attachConsole } from '@tauri-apps/plugin-log';

attachConsole().catch(console.error);

console.log('[Main] Starting Vue app...');
const app = createApp(App);

app.component('BaseButton', BaseButton);
app.component('SpatialInput', SpatialInput);
app.component('RangeSlider', RangeSlider);

app.use(createPinia());
app.use(router);

console.log('[Main] Mounting app...');
app.mount("#app");
console.log('[Main] App mounted!');
