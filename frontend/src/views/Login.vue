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
  show1 = false;



  mounted() {
    this.$store.dispatch('setIsLoggedIn', false);
    this.$store.dispatch('setIsProvider', false);
    this.$store.dispatch('setIsPatient', false);
  }


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
