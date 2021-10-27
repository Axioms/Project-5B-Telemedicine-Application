<template>
  <div class="requestAppointments">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Scheduling Appointment</h1>
        <p class="text-center mt-4">Select a day to view available times.</p>
          <v-calendar color="accent" v-model="selectedDay"></v-calendar>
          
          <v-select
            class="mt-5"
            :items="times"
            label="Appointment Time"
            v-model="selectedTime"
          ></v-select>
          <br/>
          <v-text-field
            outlined
            label="Comments"
            v-model="apptComments"
            >
            
          </v-text-field>

        <br/>
        <v-btn
              :disabled="validApptRequest()"
              color="success"
              @click="submitAppointmentRequest()"
              large
        >Schedule</v-btn> {{ selectedDay }}T{{ selectedTime }}Z

   </v-card>

   <v-snackbar
        v-model="snackbar"
        :timeout="2000"
      >
        Your appointment request has been submitted!
        <template v-slot:action="{ attrs }">
          <v-btn
            color="success"
            text
            v-bind="attrs"
            @click="snackbar = false"
          >
            Close
          </v-btn>
        </template>
    </v-snackbar>
  </div>
</template>


<script lang="ts">
import Vue from "vue";
import Options from "vue-class-component";


@Options({
  props: {
    
  },
})
export default class scheduleCall extends Vue {
  times = ["9:00", "10:00", "11:00", "12:00", "1:00", "2:00", "3:00", "4:00", "5:00"];
  selectedTime = null;
  selectedDay = null;
  apptComments = null;
  snackbar = false;

  validApptRequest () {
    return !((this.selectedTime !== null) && (this.selectedDay !== null));
  }
  submitAppointmentRequest() {
    const request = {
      startTime: this.selectedDay+"T"+this.selectedTime+"Z",
      comments: this.apptComments
    }
    this.$store.dispatch('createAppointmentRequest', request);
    this.selectedDay = null;
    this.selectedTime = null;
    this.apptComments = null;
    this.snackbar = true;
  }
}
</script>

