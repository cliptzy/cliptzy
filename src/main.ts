import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import "@fontsource/inter";
import "@fontsource/geist-sans/600.css";
import "@fontsource/geist-sans/700.css";
import "@fontsource/geist-mono/400.css";
import "./assets/styles/main.css";

import CButton from "./components/CButton.vue";
import CInput from "./components/CInput.vue";
import CSlider from "./components/CSlider.vue";
import CToggle from "./components/CToggle.vue";
import CProgress from "./components/CProgress.vue";
import CCard from "./components/CCard.vue";
import CIconButton from "./components/CIconButton.vue";
import CBadge from "./components/CBadge.vue";
import CTooltip from "./components/CTooltip.vue";
import CDropdown from "./components/CDropdown.vue";
import CDivider from "./components/CDivider.vue";

import { attachConsole } from "@tauri-apps/plugin-log";

attachConsole().catch(console.error);

console.log("[Main] Starting Vue app...");
const app = createApp(App);

app.component("CButton", CButton);
app.component("CInput", CInput);
app.component("CSlider", CSlider);
app.component("CToggle", CToggle);
app.component("CProgress", CProgress);
app.component("CCard", CCard);
app.component("CIconButton", CIconButton);
app.component("CBadge", CBadge);
app.component("CTooltip", CTooltip);
app.component("CDropdown", CDropdown);
app.component("CDivider", CDivider);

app.use(createPinia());
app.use(router);

console.log("[Main] Mounting app...");
app.mount("#app");
console.log("[Main] App mounted!");
