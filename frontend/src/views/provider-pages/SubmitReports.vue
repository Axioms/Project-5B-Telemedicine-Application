<template>
  <div class="submitReports">
     <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Submit a Report</h1>

        <v-spacer></v-spacer>

        <v-autocomplete
            :items="patients"
            :filter="customFilter"
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
        
        <input type="file" multiple :name="uploadFieldName" :disabled="isSaving" @change="filesChange($event.target.name, $event.target.files); fileCount = $event.target.files.length"
            accept="image/*" class="input-file">
            <p v-if="isInitial">
              Drag your file(s) here to begin<br> or click to browse
            </p>
            <p v-if="isSaving">
              Uploading {{ fileCount }} files...
            </p>
        <br />
        <v-btn
            class="mt-5"
            color="success"
            label="Submit report"
            @click="submitReport"
        >Submit report</v-btn>


   </v-card>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Options from "vue-class-component";


@Options({
  props: {
    
  },
})
export default class submitReports extends Vue {
    isSaving = false;
}
</script>

