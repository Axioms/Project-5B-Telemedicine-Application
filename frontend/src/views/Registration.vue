<template src="./html/registration.html"></template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';

@Options({
  components: {

  },
})
export default class Registration extends Vue {
  validLoginForm = true;
  username = '';
  password = '';
  userType = '';
  show1 = false;
  email = '';
  firstname ='';
  lastname = '';
  picked = '';



  mounted() {
    this.$store.dispatch('setIsLoggedIn', false);
    this.$store.dispatch('setIsProvider', false);
    this.$store.dispatch('setIsPatient', false);
  }

  cancelButtonClicked (){
    history.back()
  }

}


</script>

<style>

</style>