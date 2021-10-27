<template>
  <div class="calendar">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center" v-if='loaded'>

          {{ $refs.calendar.title }}
        </h1>
        <v-sheet height="600">
          <v-calendar
            ref="calendar"
            :events="events">
          </v-calendar>
        </v-sheet>
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
          <span class="ml-6" style="color: red; font-size: smaller;" v-if="!appointment.status"><br/>Appointment has not been confirmed</span>
          <span class="ml-6" style="color: green; font-size: smaller;" v-if="appointment.status"><br />Appointment confirmed</span>
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
    this.loadAppointments();
  }

  async loadAppointments () {
    const currentUser = firebase.auth().currentUser;
    const ref = db.collection('appointments');
    const snapshot = await ref.where('patient', '==', currentUser.uid).get();
    snapshot.forEach(doc => {
      
      let date = doc.data().startTime;
      this.events.push({
        name: "Appointment",
        start: date,
        end: date,
        color: "blue"
      })

      // Populate upcoming appointments
      // // Only add dates that are after the current date.
      if(this.isDateAfterToday(date)){
        this.myAppointments.push({
          id: new Date(date).toString(),
          status: doc.data().approvedStatus,
          comments: doc.data().comments
        })
      }
    });
  }

  populateCalendarEvents() {
    this.myAppointments.forEach(element => {
      this.events.push({start: element.id})
    })
  }

  isDateAfterToday(date) {
    return new Date(date) > new Date(new Date().toDateString());
  }
}

</script>

