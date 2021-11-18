<template>
  <div class="scheduleCall">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Schedule Call</h1>

        <v-spacer></v-spacer>

        <v-autocomplete
            :items="patients"
            :filter="customFilter"
            color="blue"
            v-model="selectedPatient"
            label="Select a patient to schedule a call with"
        ></v-autocomplete>

        <v-menu
            :close-on-content-click="true"
            lazy
            transition="scale-transition"
            max-width="290px"
            min-width="290px"
          >
          <template v-slot:activator="{ on }">
            <v-text-field
              label="Select a date"
              v-model="selectedDate"
              readonly
              v-on="on"
            ></v-text-field>
          </template>
          <v-date-picker
            locale="en-in"
            v-model="selectedDate"
            no-title
          ></v-date-picker>
        </v-menu>

        <v-menu
            :close-on-content-click="false"
            lazy
            transition="scale-transition"
            max-width="290px"
            min-width="290px"
          >
            <template v-slot:activator="{ on }">
              <v-text-field
                label="Select a time"
                v-model="selectedTime"
                readonly
                v-on="on"
              ></v-text-field>
            </template>
            <v-time-picker
              format="ampm"
              v-model="selectedTime"
              no-title
            ></v-time-picker>
          </v-menu>

          <v-btn
            color="success"
            label="Submit call time"
            @click="confirmScheduling"
          >Schedule Call</v-btn>


   </v-card>


    <v-snackbar
        v-model="snackbar"
        :timeout="timeout"
      >
        Your call has been scheduled!

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
  selectedPatient = "";
  patients = ["John", "George", "Chris", "Alex"];
  selectedDate: any = "";
  selectedTime: any = "";
  snackbar = false;
  timeout = 2000;

  private confirmScheduling (): void {
    // PUSH CALL TIME TO PATIENT'S CALENDAR
    // FOR NOW, RESET FIELDS AND SHOW SUCCESS MESSAGE
    this.selectedDate = "";
    this.selectedTime = "";
    this.selectedPatient = "";
    this.snackbar = true;
  }
}
</script>
