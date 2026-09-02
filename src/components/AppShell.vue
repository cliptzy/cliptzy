<template>
    <div
        class="h-screen w-full bg-base-100 text-base-content flex flex-col font-sans overflow-hidden"
    >
        <!-- Title Bar (Full Width) -->
        <MacOsWindowsTitleBar />

        <div class="flex-1 flex overflow-hidden relative">
            <!-- Icon Rail Navigation -->
            <IconRail />

            <!-- Main Content Area -->
            <div class="flex-1 flex flex-col overflow-hidden">
                <main class="flex-1 overflow-y-auto relative scroll-smooth">
                    <router-view v-slot="{ Component }">
                        <transition name="fade-slide" mode="out-in">
                            <component :is="Component" />
                        </transition>
                    </router-view>
                </main>

                <!-- Global Status Monitor (Floating) -->
                <GlobalStatusBar />

                <!-- Toast Notifications -->
                <TerminalToast />
            </div>
        </div>

        <!-- Bottom Status Bar -->
        <StatusBar />
    </div>
</template>

<script setup lang="ts">
import MacOsWindowsTitleBar from "../components/MacOsWindowsTitleBar.vue";
import IconRail from "../components/IconRail.vue";
import GlobalStatusBar from "../components/GlobalStatusBar.vue";
import TerminalToast from "../components/TerminalToast.vue";
import StatusBar from "../components/StatusBar.vue";
</script>

<style scoped>
/* Page transition: 150ms opacity + translateY(4px) fade-up */
.fade-slide-enter-active,
.fade-slide-leave-active {
    transition:
        opacity 0.15s ease,
        transform 0.15s ease;
}

.fade-slide-enter-from {
    opacity: 0;
    transform: translateY(4px);
}

.fade-slide-leave-to {
    opacity: 0;
    transform: translateY(-4px);
}

/* Respect reduced motion preference */
@media (prefers-reduced-motion: reduce) {
    .fade-slide-enter-active,
    .fade-slide-leave-active {
        transition: opacity 0.1s ease;
    }

    .fade-slide-enter-from,
    .fade-slide-leave-to {
        transform: none;
    }
}
</style>
