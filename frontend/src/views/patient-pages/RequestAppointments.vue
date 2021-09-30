<!-- no routing yet for this view -->

<template>
  <div class="requestAppointments">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Sceduling Appointment</h1>
        <p class="text-center mt-4">Select a day to view available times.</p>

            <v-calendar color="accent"></v-calendar>
            <!-- add listener for when clicking a day to show available times? -->

        <br/>
        <v-btn
              :disabled="!validLoginForm"
              color="success"
              @click="submitScheduleAppointment"
              large
        >Schedule</v-btn>

   </v-card>
  </div>
</template>

