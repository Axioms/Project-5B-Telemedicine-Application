// Vue router documentation
// https://router.vuejs.org/guide/advanced/navigation-guards.html

import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import Home from "../views/Home.vue";
import Login from "../views/Login.vue";
import PatientPortal from "../views/PatientPortal.vue";
import ProviderPortal from "../views/ProviderPortal.vue";
import RequestAppointments from "../views/./patient-pages/RequestAppointments.vue";
import Registration from "@/views/Registration.vue";
import Account from "@/views/Account.vue";
import firebase from "firebase/compat";

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
    path: "/requestAppointments",
    name: "Request an appointment",
    component: RequestAppointments
  },
  {
    path: "/providerPortal",
    name: "Provider Portal",
    component: ProviderPortal
  },
  {
    path: "/registration",
    name: "Registration",
    component: Registration
  },
  {
    path: "/account",
    name: "account",
    component: Account
  },
];

const router = new VueRouter({
  routes,
});

/*
router.beforeEach((to,from,next) => {
  if(to.matched.some(record => record.meta.auth)){
    firebase.auth().onAuthStateChanged((user) => {
      if(!user)
      {
        next({
          path: '/'

        })
      } else {
        next()
      }
    })}else next()
})*/

export default router;
