<template>
  <div class="submitReports">
     <v-card elevation="0" class="mx-auto mt-5 pa-6" max-width="60%" >
        <h1 class="text-center mt-4">Submit a Report</h1>

        <v-spacer></v-spacer>

        <v-autocomplete
            :items="patients"
            color="blue"
            v-model="selectedPatient"
            label="Select a patient to send report to"

        ></v-autocomplete>

        <v-textarea
        outlined
            label="Enter details"
            v-model="reportDetails"
            rows="3"
            required 
        ></v-textarea>
        
        <input type="file" accept="any" @change="previewImage" class="input-file">
          <p v-if="isSaving">
            Uploading files...
          </p>
        <br />

        <v-btn
            class="mt-5"
            color="success"
            label="Submit report"
            :disabled="validForm()"
            @click="submitReport()"
        >Submit report</v-btn>
   </v-card>
    <v-snackbar
        v-model="snackbar"
        :timeout="2000"
      >
        Your report has been submitted!
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
import firebase from "firebase/compat/app";
import "firebase/compat/auth"
import "firebase/compat/firestore";
import { collection, query, where, getDocs } from "firebase/firestore";
import { getDownloadURL, getStorage, ref, uploadBytes } from "firebase/storage";
import db from "@/main.ts"


@Options({
  props: {

  },
})
export default class submitReports extends Vue {

  // WARNING: THE CODE YOU ARE ABOUT TO SEE MAY BE SCARY
  // I'M JUST AS AFRAID OF IT AS YOU ARE
  // ...it does work though ♥

  selectedPatient = "";
  patients = "";
  reportDetails = "";
  isSaving = false;

  // File upload variables
  imageData: any;
  snackbar = false;


  mounted() {
    this.addPatient();
  }

  addPatient() {
    db.collection('users').where("usertype", "==", "Patient")
        .get()
        .then(querySnapshot => {
          const users = querySnapshot.docs.map(doc => doc.data().firstname+" "+doc.data().lastname).sort() as unknown
          const users2 = users as string
          this.patients = users2;
        })
  }

  async submitReport() {
    this.isSaving = true;
    var patientID = " ";
    var fname = this.selectedPatient.split(" ")[0];
    var lname = this.selectedPatient.split(" ")[1];
   
    var q = query((collection(db, 'users')), where("firstname", '==', fname), where("lastname", '==', lname));

    const querySnapshot = await getDocs(q);
    querySnapshot.forEach((doc) => {
      patientID = (doc.id);
    });
    var providerID = this.$store.state.profileID;

    var reportDate = new Date().toString();

    if(this.imageData){
      this.uploadImage(patientID, providerID, reportDate)
    }
    else{
      const request = {
          patientID: patientID,
          providerID: providerID,
          content: this.reportDetails,
          date: reportDate,
          url: "null"
        };
        this.$store.dispatch('createReport', request);
        this.isSaving = false;
        this.selectedPatient = "";
        this.reportDetails = "";
        this.imageData = null;
        this.snackbar = true;
    }
    
  }

  async uploadImage(patientID: string, providerID: string, reportDate: string) {
    const storage = getStorage();
    const storageRef = ref(storage, `${this.imageData.name}`);
    var result = "";

    uploadBytes(storageRef, this.imageData).then((snapshot) => {
      getDownloadURL(snapshot.ref).then((result) => {
        const request = {
          patientID: patientID,
          providerID: providerID,
          content: this.reportDetails,
          date: reportDate,
          url: result
        };
        this.$store.dispatch('createReport', request);
        this.isSaving = false;
        this.selectedPatient = "";
        this.reportDetails = "";
        this.imageData = null;
        this.snackbar = true;
      });
    });
    
  }

  previewImage(event: any) {
      this.imageData = event.target.files[0];
  }

  validForm() {
    return !((this.selectedPatient !== "") && (this.reportDetails !== null));
  }

}



</script>

