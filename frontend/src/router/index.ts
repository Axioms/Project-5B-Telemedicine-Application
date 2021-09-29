// Vue router documentation
// https://router.vuejs.org/guide/advanced/navigation-guards.html

import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import Home from "../views/Home.vue";
import Login from "../views/Login.vue";
import PatientPortal from "../views/PatientPortal.vue";
import ProviderPortal from "../views/ProviderPortal.vue";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    name: "Login",
    component: Login,
  },
  {
    path: "/about",
    name: "About",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/About.vue"),
  },
  {
    path: "/patientPortal",
    name: "Patient Portal",
    component: PatientPortal
  },
  {
    path: "/providerPortal",
    name: "Provider Portal",
    component: ProviderPortal
  }
  
];

const router = new VueRouter({
  routes,
});

export default router;
