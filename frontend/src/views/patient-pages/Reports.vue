<template>
  <div class="reports">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Reports</h1>
        
        <div>
        <v-expansion-panels
        
        >
          <v-expansion-panel v-for="Report in testReports" :key="Report.date">
            <v-expansion-panel-header>{{ Report.date }}</v-expansion-panel-header>
            <v-expansion-panel-content>
              <ul>
                {{ Report.details }}
              </ul>
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


@Options({
  props: {
    
  },
})
export default class Reports extends Vue {
  testReports =  { Report1: {id: 1, date: "Oct. 5, 2021", details: "Some details here"},  Report2: {id: 2, date: "Nov. 2, 2020", details: "Some details here."} };
}
</script>

