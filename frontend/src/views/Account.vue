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

  newBirthday = "";
  newSSN= "";

  newInsurance = "";
  newInsuranceGroup= "";
  newMemberID= "";
  newRxBIN= "";
  newRxPCN= "";
  newRxGRP= "";

  newCardholderName= "";
  newCardNumber= "";
  newCVV= "";
  newExpirationDate= "";


  firstname = this.$store.state.profileFirstName;
  email = this.$store.state.profileEmail;
  lastname = this.$store.state.profileLastName;

  birthday = this.$store.state.birthday;
  ssn = this.$store.state.ssn;

  insurance = this.$store.state.insurance;
  insurancegroup = this.$store.state.insurancegroup;
  memberid = this.$store.state.memberid;
  rxbin = this.$store.state.rxbin;
  rxpcn = this.$store.state.rxpcn;
  rxgrp = this.$store.state.rxgrp;

  cardholdername = this.$store.state.cardholdername;
  cardnumber = this.$store.state.cardnumber;
  cvv = this.$store.state.cvv;  
  expirationdate = this.$store.state.expirationdate;

  mounted() {
    this.newFirstName = this.firstname;
    this.newLastName = this.lastname;
    this.newEmail = this.email;

    this.newBirthday = this.birthday;
    this.newSSN = this.ssn;

    this.newInsurance = this.insurance;
    this.newInsuranceGroup = this.insurancegroup;
    this.newMemberID = this.memberid;
    this.newRxBIN = this.rxbin;
    this.newRxPCN = this.rxpcn;
    this.newRxGRP = this.rxgrp;

    this.newCardholderName = this.cardholdername;
    this.newCardNumber = this.cardnumber;
    this.newCVV = this.cvv;
    this.newExpirationDate = this.expirationdate;
  }

  isDisabled() {
    return !((this.newFirstName !== this.firstname) || (this.newLastName !== this.lastname) 
    || (this.newEmail !== this.email) || (this.newSSN !== this.ssn) 
    || (this.newInsurance !== this.insurance) || (this.newInsuranceGroup !== this.insurancegroup) 
    || (this.newMemberID !== this.memberid) || (this.newRxBIN !== this.rxbin) 
    || (this.newRxPCN !== this.rxpcn) || (this.newRxGRP !== this.rxgrp) 
    || (this.newCardholderName) !== this.cardholdername || (this.newCardNumber !== this.cardnumber) 
    || (this.newCVV !== this.cvv) || (this.newExpirationDate !== this.expirationdate)
	  || (this.newBirthday !== this.birthday));
  }

  submitAccountChanges(){
    this.$store.commit('changeAccountInfo', {firstName: this.newFirstName, lastName: this.newLastName, 
    email: this.newEmail, birthday: this.newBirthday, ssn: this.newSSN, insurance: this.newInsurance, 
    insurancegroup: this.newInsuranceGroup, memberid: this.newMemberID, rxbin: this.newRxBIN, rxpcn: this.newRxPCN,
    rxgrp: this.newRxGRP, cardholdername: this.newCardholderName, cardnumber: this.newCardNumber, cvv: this.newCVV,
    expirationdate: this.newExpirationDate});

    this.snackbar = true;
  }
}

</script>