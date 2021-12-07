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
  error = false;
  errorMsg = "";
  birthday = "";
  insurance = ["Ambetter from Peach State Health Plan",
    "Anthem Blue Cross and Blue Shield",
    "Bright Health",
    "Friday Health Plans",
    "HS - Aetna CVS Health",
    "HS - Alliant Health Plans",
    "HS - Ambetter from Peach State Health Plan",
    "HS - Blue Cross Blue Shield Healthcare Plan of Georgia, Inc",
    "HS - Bright HealthCare",
    "HS - CareSource",
    "HS - Cigna HealthCare of Georgia, Inc.",
        "HS - Friday Health Plans",
    "HS - Kaiser Permanente",
    "HS - Oscar Health Plan of Georgia",
    "HS - UnitedHealthcare",
    "Kaiser Permanente GA"];
  selectedInsurance = "";

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
    if(this.email == "" || this.firstname == "" || this.lastname == "" || this.password == "" ||this.picked == "" ||this.birthday == "" || this.selectedInsurance == "")
    {
      this.error = true;
      this.errorMsg = "Fill out all the fields"

    }
    else
    {
      const firebaseAuth = await firebase.auth();
      const createUser: any = await firebaseAuth.createUserWithEmailAndPassword(this.email, this.password)
    .catch((error) => {
      this.error = true;
      this.errorMsg = error.message;

    })
      const result = await createUser;
      const dataBase = db.collection("users").doc(result.user.uid);
      await dataBase.set({
        firstname: this.firstname,
        lastname: this.lastname,
        usertype: this.picked,
        email: this.email,
        birthday: this.birthday,
        insurance: this.selectedInsurance
      })
      alert('Account created for '+this.email);
      this.error = false;
      this.errorMsg ="";
      await this.$router.push('/');
    }

  }

}


</script>

<style>

</style>