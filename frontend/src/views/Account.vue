<template src="./html/account.html"></template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';
import { getAuth } from "firebase/auth";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"
import db from "@/main.ts"

@Options({
  components: {

  },
})
export default class Account extends Vue
{
  newFirstName = "";
  newLastName = "";
  newEmail = "";
  snackbar = false;

  firstname = this.$store.state.profileFirstName;
  email = this.$store.state.profileEmail;
  lastname = this.$store.state.profileLastName;

  // auth = getAuth();
  // user = this.auth.currentUser;
  // profile = db.collection('profiles').doc(this.user.uid);

  mounted() {
    this.newFirstName = this.firstname;
    this.newLastName = this.lastname;
    this.newEmail = this.email;
  }

  isDisabled() {
    return !((this.newFirstName !== this.firstname) || (this.newLastName !== this.lastname) || (this.newEmail !== this.email));
  }

  submitAccountChanges(){
    this.$store.commit('changeAccountInfo', {firstName: this.newFirstName, lastName: this.newLastName, email: this.newEmail});
    this.snackbar = true;
  }
}

</script>