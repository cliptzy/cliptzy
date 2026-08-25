import { createRouter, createWebHistory } from "vue-router";
import DashboardView from "../views/DashboardView.vue";
import LoginView from "../views/LoginView.vue";
import { useAuthStore } from "../stores/auth";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      name: "login",
      component: LoginView,
      meta: { layout: "AuthLayout" },
    },
    {
      path: "/",
      name: "dashboard",
      component: DashboardView,
      meta: { requiresAuth: true, layout: "AppLayout" },
    },
    {
      path: "/clipper",
      name: "clipper",
      component: () => import("../views/ClipperView.vue"),
      meta: { requiresAuth: true, layout: "AppLayout" },
    },
    {
      path: "/compilation",
      name: "compilation",
      component: () => import("../views/CompilationView.vue"),
      meta: { requiresAuth: true, layout: "AppLayout" },
    },
    {
      path: "/upload",
      name: "upload",
      component: () => import("../views/UploadView.vue"),
      meta: { requiresAuth: true, layout: "AppLayout" },
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/SettingsView.vue"),
      meta: { requiresAuth: true, layout: "AppLayout" },
    },
  ],
});

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore();

  // Verify auth status on navigation if engine is ready
  if (!authStore.isLoggedIn && !authStore.isChecking) {
    await authStore.checkAuthStatus();
  }

  if (to.meta.requiresAuth && !authStore.isLoggedIn) {
    next("/login");
  } else if (to.path === "/login" && authStore.isLoggedIn) {
    next("/");
  } else {
    next();
  }
});

export default router;
