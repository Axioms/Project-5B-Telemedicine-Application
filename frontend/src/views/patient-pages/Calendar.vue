<template>
  <div class="calendar">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h2 class="text-center" v-if='loaded'>

          {{ $refs.calendar.title }}
        </h2>
        <v-calendar
          ref="calendar"
          :events="events">
        </v-calendar>
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
  events = [{name: "Appointment #1", start: "2021-10-26", end: "2021-10-26", timed: 1}]

  mounted () {
    this.loaded = true;
  }
}

</script>

