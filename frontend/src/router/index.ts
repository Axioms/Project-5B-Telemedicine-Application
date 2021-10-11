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
import PatientCalendar from "../views/patient-pages/Calendar.vue";
import DocChat from "../views/patient-pages/DocChat.vue";
import PatientAccount from "../views/patient-pages/PatientAccount.vue";
import PatientReports from "../views/patient-pages/Reports.vue";
import Resources from "../views/patient-pages/Resources.vue";
import ScheduleAppointment from "../views/provider-pages/ScheduleAppointments.vue";
import ProviderCalendar from "../views/provider-pages/Calendar.vue";
import ProviderReports from "../views/provider-pages/Reports.vue";
import ScheduleCall from "../views/provider-pages/ScheduleCall.vue";
import PatientChat from "../views/provider-pages/PatientChat.vue";
import Education from "../views/provider-pages/Education.vue";

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
    path: "/patientAccount",
    name: "Account details",
    component: PatientAccount
  },
  {
    path: "/patientReports",
    name: "Patient Reports",
    component: PatientReports
  },
  {
    path: "/resources",
    name: "Patient Resources",
    component: Resources
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
    path: "/providerReports",
    name: "Provider Reports",
    component: ProviderReports
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
    path: "/education",
    name: "Education",
    component: Education
  },


  // Registration
  {
    path: "/registration",
    name: "Registration",
    component: Registration
  },
  
];

const router = new VueRouter({
  routes,
});

export default router;
