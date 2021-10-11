<template src="./html/login.html"></template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';
import "firebase/compat/firestore";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"

@Options({
  components: {
    
  },
})
export default class Login extends Vue {
  validLoginForm = true;
  email = '';
  password = '';
  userType = '';
  show1 = false;
  error = false;
  errorMsg = "";


  mounted() {
    this.$store.dispatch('setIsLoggedIn', false);
    this.$store.dispatch('setIsProvider', false);
    this.$store.dispatch('setIsPatient', false);
  }


  // This needs to be updated to support actual login, but for now I'm just having it navigate to the next page
  submitLoginForm (){
    if(this.userType=="patient"){

      firebase.auth().signInWithEmailAndPassword(this.email, this.password)
      .then(() =>
      {
        this.error = false;
        this.errorMsg = "";
        this.$router.push('patientPortal')
        return;
      })
          .catch(err =>{

            this.error = true;
            this.errorMsg = err.message;
          });

    }
    else if(this.userType=="provider"){
      firebase.auth().signInWithEmailAndPassword(this.email, this.password)
          .then(() =>
          {
            this.error = false;
            this.errorMsg = "";
            this.$router.push('providerPortal');
            return;
          })
      .catch(err =>{

        this.error = true;
        this.errorMsg = err.message;
          });

    }


  }
  submitRegistrationForm (){
    this.$router.push('registration');
  }


}
</script>

<style>

</style>