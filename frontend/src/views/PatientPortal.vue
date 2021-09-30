<template src="./html/patient-portal.html"></template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';

@Options({
  components: {
    
  },
})
export default class PatientPortal extends Vue {
    mounted() {
        this.$store.dispatch('setIsLoggedIn', true);
        this.$store.dispatch('setIsPatient', true);
    }
}
</script>
