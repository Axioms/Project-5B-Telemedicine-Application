<template>
  <div class="scheduleAppointments">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Schedule Appointments</h1>
        <div>
          <div>
            <v-data-table
                :headers="headers"
                :items="patients"
                item-key="name"
                class="elevation-1"
                :search="search"

            >
              <template v-slot:top>
                <v-text-field
                    v-model="search"
                    label="Search"
                    class="mx-4"
                ></v-text-field>
              </template>
              <template v-slot:body.append>
                <tr>
                  <td></td>
                  <td colspan="4"></td>
                </tr>
              </template>
              <template v-slot:item.actions="{ item }" >
                <v-icon
                    large
                    class="mr-2"
                    style="color: green;"
                    @click="confirmAppointment(item, true)"
                    v-if="!item.apptConfirmed"
                  >
                    mdi-check
                  </v-icon>
                  <v-icon
                    large
                    @click="confirmAppointment(item, false)"
                    style="color: red;"
                    v-if="!item.apptConfirmed"
                  >
                    mdi-close-octagon-outline
                </v-icon>
              </template>
            </v-data-table>
          </div>
        </div>
   </v-card>
  </div>
</template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';
import "firebase/compat/firestore";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"
import db from "@/main.ts"

@Options({
  components: {
    
  },
})
export default class ScheduleAppointments extends Vue {
  headers = [ {
      text: 'Patient',
      align: 'start',
      value: 'name',
    },
    { text: 'Comments', value: 'comments'},
    { text: 'Date', value: 'date' },
    { text: 'Confirmed', value: 'apptConfirmed'},
    { text: 'Actions', value: 'actions', sortable: false}
    ]
  patients: any = [];
  search = "";


  created () {
    this.populateAppointments();
  }

  async populateAppointments() {
    
    const ref = db.collection('appointments');
    const snapshot = await ref.get();
    snapshot.forEach(doc => {

      this.patients.push({
        name: doc.data().patientName,
        date: (new Date(doc.data().startTime).toString()),
        comments: doc.data().comments,
        apptConfirmed: doc.data().approvedStatus,
        apptID: doc.id
      })
    })
  }

  async confirmAppointment(appointment: any, status: boolean){
    if(!status){
      const res = await db.collection('appointments').doc(appointment.apptID).delete();
    }
    else if (status){
      const ref = db.collection('appointments').doc(appointment.apptID);

      // Set the 'capital' field of the city
      const res = await ref.update({approvedStatus: true});
    }
    this.patients = [];
    this.populateAppointments();
  }

}
</script>