import Vue from "vue";
import Vuex from "vuex";
import createPersistedState from "vuex-persistedstate";
import "firebase/compat/firestore";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"
import { getAuth, updateEmail } from "firebase/auth";
import db from "@/main.ts"

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    isLoggedIn: false,
    isPatient: false,
    isProvider: false,
    profileEmail: null,
    profileFirstName: "",
    profileLastName: "",
    profileID: null,
    userType: null,
    user: null,
    birthday: "",
    insurance: "",
    ssn: "",
    insurancegroup: "",
    memberid: "",
    rxbin: "",
    rxpcn: "",
    rxgrp: "",
    cardholdername: "",
    cardnumber: "",
    cvv: "",
    expirationdate: ""
  },
  getters: {
    getIsLoggedIn: state => {
      return state.isLoggedIn;
    },
    getIsPatient: state => {
      return state.isPatient;
    },
    getIsProvider: state => {
      return state.isProvider;
    }
  },
  mutations: {
    updateUser(state, payload)
    {
      state.user = payload;
    },
    setProfileInfo(state, doc)
    {
      state.profileID = doc.id;
      state.profileEmail = doc.data().email;
      state.profileFirstName = doc.data().firstname;
      state.profileLastName = doc.data().lastname;
      state.userType = doc.data().usertype;
      state.birthday = doc.data().birthday;
      state.ssn = doc.data().ssn;
      state.insurance = doc.data().insurance;
      state.insurancegroup = doc.data().insurancegroup;
      state.memberid = doc.data().memberid;
      state.rxbin = doc.data().rxbin;
      state.rxpcn = doc.data().rxpcn;
      state.rxgrp = doc.data().rxgrp;
      state.cardholdername = doc.data().cardholdername;
      state.cardnumber = doc.data().cardnumber;
      state.cvv = doc.data().cvv;
      state.expirationdate = doc.data().expirationdate;
    },
    // method for changing account info on "Account" page
    changeAccountInfo(state, { firstName, lastName, email, birthday, ssn, insurance, insurancegroup, memberid, rxbin, rxpcn, rxgrp, cardholdername, cardnumber, cvv, expirationdate } ){
      state.profileFirstName = firstName;
      state.profileLastName = lastName;
      state.profileEmail = email;
      state.birthday = birthday;
      state.ssn = ssn;
      state.insurance = insurance;
      state.insurancegroup = insurancegroup;
      state.memberid = memberid;
      state.rxbin = rxbin;
      state.rxpcn = rxpcn;
      state.rxgrp = rxgrp;
      state.cardholdername = cardholdername;
      state.cardnumber = cardnumber;
      state.cvv = cvv;
      state.expirationdate = expirationdate;
      
      const currentUser = firebase.auth().currentUser;
      currentUser?.updateEmail(email);

      const dataBase = db.collection("users").doc(currentUser?.uid);
      dataBase.set({
        firstname: firstName,
        lastname: lastName,
        email: email,
        birthday: birthday,
        ssn: ssn,
        insurance: insurance,
        insurancegroup: insurancegroup,
        memberid: memberid,
        rxbin: rxbin,
        rxpcn: rxpcn,
        rxgrp: rxgrp,
        cardholdername: cardholdername,
        cardnumber: cardnumber,
        cvv: cvv,
        expirationdate: expirationdate
      })
    }
  },
  actions: {
    setIsLoggedIn(context, status){
      context.state.isLoggedIn = status;
    },
    setIsPatient(context, status) {
      context.state.isPatient = status;
    },
    setIsProvider(context, status) {
      context.state.isProvider = status;
    },
    async getCurrentUser({commit})
    {
      const database = await db.collection('users').doc(firebase?.auth()?.currentUser?.uid);
      const dbResults = await database.get();
      commit("setProfileInfo", dbResults);
      console.log(dbResults);
    },
    createAppointmentRequest(context, request){
      const appointmentRequest = {
        patient: firebase.auth().currentUser?.uid,
        startTime: request.startTime,
        comments: request.comments,
        approvedStatus: false,
        patientName: (context.state.profileFirstName + " " + context.state.profileLastName)
      }
      
      const res = db.collection('appointments').add(appointmentRequest);
    },
    createReport(context, request){
      const res = db.collection('reports').add(request);
    }
  },
  modules: {},
  plugins: [createPersistedState()]
});
