<template>
  <div class="reports">
    <v-card elevation="0" class="mx-auto mt-5 pa-7" max-width="60%">
        <h1 class="text-center mt-4">Reports</h1>
        
        <div>
        <v-expansion-panels
        
        >
          <v-expansion-panel v-for="Report in testReports" :key="Report.date">
            <v-expansion-panel-header>{{ Report.date }}</v-expansion-panel-header>
            <v-expansion-panel-content>
              <ul>
                {{ Report.content }}
              </ul>
              <div v-if="hasLink(Report)">
                <ul class="text-center"><a v-bind:href='Report.url' target="_blank">View Attachment</a></ul>
              </div>
              
            </v-expansion-panel-content>
          </v-expansion-panel>
        </v-expansion-panels>
        </div>
   </v-card>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Options from "vue-class-component";
import db from "@/main.ts"


@Options({
  props: {
    
  },
})
export default class Reports extends Vue {
  
  testReports: any = [];

 async mounted() {
    this.testReports = await this.getReports()
  }

  getReports() {
    return db.collection('reports').where("patientID", "==", this.$store.state.profileID)
        .get()
        .then(function(querySnapshot) { 
            return querySnapshot.docs.map(doc => doc.data())
        })
        
  }

  test() {
    console.log(this.testReports)
  }

  hasLink(Report: any) {
    if(Report.url != "null" || Report.url == "" || Report.url == null || Report.url == " "){
      return true;
    }
    return false;
  }
}
</script>

