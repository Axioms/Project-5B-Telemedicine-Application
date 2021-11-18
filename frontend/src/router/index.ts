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

import PatientCalendar from "../views/patient-pages/Calendar.vue";
import DocChat from "../views/patient-pages/DocChat.vue";
import PatientReports from "../views/patient-pages/Reports.vue";
import PatientResources from "../views/patient-pages/Resources.vue";
import ScheduleAppointment from "../views/provider-pages/ScheduleAppointments.vue";
import ProviderCalendar from "../views/provider-pages/Calendar.vue";
import ScheduleCall from "../views/provider-pages/ScheduleCall.vue";
import PatientChat from "../views/provider-pages/PatientChat.vue";
import ProviderResources from "../views/provider-pages/Resources.vue";
import SubmitReports from "../views/provider-pages/SubmitReports.vue";

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

  // Patient pages
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
    path: "/patientCalendar",
    name: "Patient Calendar",
    component: PatientCalendar
  },
  {
    path: "/docChat",
    name: "Chat with your doctor",
    component: DocChat
  },
  {
    path: "/patientReports",
    name: "Patient Reports",
    component: PatientReports
  },
  {
    path: "/patientResources",
    name: "Patient Resources",
    component: PatientResources
  },

  // Provider pages
  {
    path: "/providerPortal",
    name: "Provider Portal",
    component: ProviderPortal
  },
  {
    path: "/scheduleAppointment",
    name: "Schedule an Appointment",
    component: ScheduleAppointment
  },
  {
    path: "/providerCalendar",
    name: "Provider Calendar",
    component: ProviderCalendar
  },
  {
    path: "/scheduleCall",
    name: "Schedule a Call",
    component: ScheduleCall
  },
  {
    path: "/patientChat",
    name: "Patient Chat",
    component: PatientChat
  },
  {
    path: "/providerResources",
    name: "Resources",
    component: ProviderResources
  },
  {
    path: "/submitReports",
    name: "Submit Reports",
    component: SubmitReports
  },

  // Login/registration
  {
    path: "/registration",
    name: "Registration",
    component: Registration
  },
  {
    path: "/account",
    name: "Account",
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
