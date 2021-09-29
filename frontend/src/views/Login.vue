<template src="./html/login.html"></template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';

@Options({
  components: {
    
  },
})
export default class Login extends Vue {
  validLoginForm = true;
  username = '';
  password = '';
  userType = '';


  // This needs to be updated to support actual login, but for now I'm just having it navigate to the next page
  submitLoginForm (){
    if(this.userType=="patient"){
      this.$router.push('patientPortal');
    }
    else if(this.userType=="provider"){
      this.$router.push('providerPortal');
    }
  }
}
</script>
