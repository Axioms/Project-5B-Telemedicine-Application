<template src="./html/registration.html"></template>

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
  error = null;
  errorMsg = "";


  mounted() {
    this.$store.dispatch('setIsLoggedIn', false);
    this.$store.dispatch('setIsProvider', false);
    this.$store.dispatch('setIsPatient', false);
  }

  cancelButtonClicked (){
    //history.back()
    this.$router.push('/');
  }
  async signUpRequest ()
  {
    if(this.email == "" || this.firstname == "" || this.lastname == "" || this.password == "" ||this.picked == "")
    {
      alert('Please fill in all the fields');
    }
    else
    {
      const firebaseAuth = await firebase.auth();
      const createUser = await firebaseAuth.createUserWithEmailAndPassword(this.email, this.password);
      const result = await createUser;
      const dataBase = db.collection("users").doc(result.user.uid);
      await dataBase.set({
        firstname: this.firstname,
        lastname: this.lastname,
        usertype: this.picked,
        email: this.email,
      })
      alert('Account created for '+this.email);
      await this.$router.push('/');
    }

  }

}


</script>

<style>
.error{
  text-align: center;
  font-size: 12px;
  color: red;
}
</style>