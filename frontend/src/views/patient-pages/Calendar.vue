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
    <h3 class="text-center mt-8">
    <p
        mx-auto
        class="ma-5"
        max-width="40%"
        v-for="appointment in myAppointments"
        :key="appointment.id"
      >
      {{ appointment.id }} 
          <span class="ml-6" style="color: red;" v-if="!appointment.status"><br/>Appointment has not been confirmed</span>
          <span class="ml-6" style="color: green;" v-if="appointment.status"><br />Appointment confirmed</span>
      <br/>
      Comments: {{appointment.comments}}
      </p>
    </h3>
  </div>
</template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';

import firebase from "firebase/compat/app"
import "firebase/compat/auth"
import db from "@/main.ts"
@Options({
  components: {

  },
})
export default class Calendar extends Vue
{
  loaded = false;
  myAppointments = [];
  events = []

  mounted () {
    this.loaded = true;
    this.loadAppointments()
  }

  async loadAppointments () {
    const currentUser = firebase.auth().currentUser;
    const ref = db.collection('appointments');
    const snapshot = await ref.where('patient', '==', currentUser.uid).get();
    snapshot.forEach(doc => {
      this.myAppointments.push({
        id: doc.data().startTime,
        status: doc.data().approvedStatus,
        comments: doc.data().comments
      })
    });
    console.log(this.myAppointments)
  }

  populateCalendarEvents() {
    this.myAppointments.forEach(element => {
      this.events.push({start: element.id})
    })
  }
}

</script>

