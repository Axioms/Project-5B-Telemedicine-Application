<template>
  <div class="calendar">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center" v-if='loaded'>

          {{ $refs.calendar.title }}
        </h1>
        <v-sheet height="600">
          <v-calendar
            ref="calendar"
            :events="events"
            color="accent"
            v-model="selectedDay">
          </v-calendar>
        </v-sheet>
   </v-card><v-divider class="mt-5 pa-4"></v-divider>
   <v-card elevation="0" class="mt-5">
     <h2 class="text-center">My Upcoming Appointments</h2>
   </v-card>
  </div>
</template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';

import firebase from "firebase/compat/app"
import "firebase/compat/auth"
@Options({
  components: {

  },
})
export default class Calendar extends Vue
{
  loaded = false;
  events = [{name: "Appt. 9AM", start: "2021-10-26", end: "2021-10-26", timed: 1}
            ,{name: "Broken Arm 9AM", start: "2021-10-28", end: "2021-10-28", timed: 1}
            ,{name: "Accute Pain 10AM", start: "2021-10-29", end: "2021-10-29", timed: 1}
            ,{name: "Appt. 12PM", start: "2021-10-29", end: "2021-10-29", timed: 1}]

  mounted () {
    this.loaded = true;
  }
}

</script>

