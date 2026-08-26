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
    },
    {
      path: "/",
      name: "dashboard",
      component: DashboardView,
      meta: { requiresAuth: true },
    },
    {
      path: "/studio",
      name: "studio",
      component: () => import("../views/StudioView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/library",
      name: "library",
      component: () => import("../views/LibraryView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/SettingsView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/about",
      name: "about",
      component: () => import("../views/AboutView.vue"),
      meta: { requiresAuth: true },
    },
  ],
});

router.beforeEach(async (to, _from) => {
  console.log(`[Router] Navigating from ${_from.path} to ${to.path}`);
  const auth = useAuthStore();
  
  if (auth.isChecking) {
    console.log(`[Router] Checking auth status...`);
    await auth.checkAuthStatus();
    console.log(`[Router] Auth check complete. Logged in: ${auth.isLoggedIn}`);
  }

  const requiresAuth = to.matched.some(record => record.meta.requiresAuth);
  console.log(`[Router] requiresAuth: ${requiresAuth}, isLoggedIn: ${auth.isLoggedIn}, to.name: ${String(to.name)}`);
  
  if (requiresAuth && !auth.isLoggedIn) {
    console.log(`[Router] Redirecting to login`);
    return { name: 'login' };
  } else if (to.name === 'login' && auth.isLoggedIn) {
    console.log(`[Router] Already logged in, redirecting to dashboard`);
    return { name: 'dashboard' };
  }
  
  console.log(`[Router] Proceeding normally`);
});

export default router;
